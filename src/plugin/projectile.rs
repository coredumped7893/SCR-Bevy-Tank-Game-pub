use std::f32::consts::FRAC_PI_2;
use std::ops::Mul;

use bevy::app::App;
use bevy::ecs::schedule::ShouldRun;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{
    default, AssetServer, Commands, Component, DespawnRecursiveExt, Entity, Handle, Image, Input,
    KeyCode, Mut, Name, Plugin, Quat, Query, Reflect, Res, ResMut, Resource, SpriteBundle,
    StartupStage, State, SystemSet, Time, Transform, Visibility, With, Without,
};

use crate::configuration_properties::{
    MAP_GRAVITY, MAP_TILE_SIZE, PROJECTILE_MUZZLE_VELOCITY, SPRITE_PROJECTILE_TEXTURE_FILE,
    Z_PROJECTILE,
};
use crate::plugin::explosion::{spawn_animation, ExplosionTexturesSheet};
use crate::plugin::map::TileCollider;
use crate::plugin::player::Player;
use crate::state::{MainGameState, ProjectilePresent};
use crate::utilsystems::player_aim::PlayerAim;
use crate::utilsystems::wall_collision::wall_collision_check;

pub struct ProjectilePlugin;

///Cache loaded texture file to avoid opening file every time - should reduce io
#[derive(Resource, Reflect)]
pub struct ProjectileTexture(pub Handle<Image>);

#[derive(Reflect)]
pub enum ProjectileType {
    HE,
    AP,
}

#[derive(Component, Reflect)]
pub struct Projectile {
    speed_vector: Vec2,
    variant: ProjectileType,
}

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_texture_asset)
            .add_state(ProjectilePresent::NA)
            .add_system_set(
                SystemSet::on_update(MainGameState::COMBAT) // Activate below systems only for combat state
                    .with_system(fire),
            )
            .add_system_set(
                SystemSet::new()
                    .label("Projectile movement handler")
                    .with_run_criteria(movement_state_validator)
                    .with_system(movement_handler),
            );
    }
}

///Checks if movement_handler can be safely run
/// Projectile entity has to be spawned before
fn movement_state_validator(
    main_game_state_query: Res<State<MainGameState>>,
    projectile_state: Res<State<ProjectilePresent>>,
) -> ShouldRun {
    if *main_game_state_query.current() == MainGameState::COMBAT
        && *projectile_state.current() == ProjectilePresent::PRESENT
    {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn load_texture_asset(mut commands: Commands, asset_server: Res<AssetServer>) {
    let p_texture: Handle<Image> = asset_server.load(SPRITE_PROJECTILE_TEXTURE_FILE);
    commands.insert_resource(ProjectileTexture(p_texture));
}

fn movement_handler(
    mut projectile_query: Query<(Entity, &mut Projectile, &mut Transform)>,
    time: Res<Time>,
    commands: Commands,
    sheet_query: Res<ExplosionTexturesSheet>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Projectile>)>,
) {
    if projectile_query.is_empty() {
        return;
    }
    let (entity, mut projectile, mut p_transform) = projectile_query.single_mut();

    //Calculate new vertical speed
    projectile.speed_vector.y += -MAP_GRAVITY * time.delta_seconds(); // acceleration * time = speed
    projectile.speed_vector.x -= projectile.speed_vector.x * 0.0004; // Very simple air resistance approx. :)

    let delta_x: f32 = projectile.speed_vector.x * time.delta_seconds();
    let delta_y: f32 = projectile.speed_vector.y * time.delta_seconds(); // speed * time = distance

    //Check for collision with walls or ground or player
    let tmp_transform_target: Vec3 = p_transform.translation + Vec3::new(delta_x, delta_y, 0.0);
    if !wall_query
        .iter()
        .any(|t| wall_collision_check(t.translation, tmp_transform_target))
    {
        point_to_flight_direction(
            &mut p_transform,
            Vec2::new(projectile.speed_vector.x, projectile.speed_vector.y),
        );
        p_transform.translation = tmp_transform_target; //Move projectile in a single tick
    } else {
        //Collided with ground or player
        stop_movement(&mut projectile);
        explode_remove(commands, entity, p_transform.into_inner(), sheet_query);
    }
}

fn explode_remove(
    mut commands: Commands,
    projectile_entity: Entity,
    projectile_transform: &Transform,
    sheet_query: Res<ExplosionTexturesSheet>,
) {
    //1. remove projectile entity
    commands.entity(projectile_entity).despawn_recursive();

    //2. spawn explosion animation on its place
    spawn_animation(
        &mut commands,
        Vec3::new(
            projectile_transform.translation.x,
            projectile_transform.translation.y,
            Z_PROJECTILE,
        ),
        &sheet_query,
    );
}

///Calculate new angle and set quaternion value
fn point_to_flight_direction(transform: &mut Transform, velocity: Vec2) {
    let angle: f32 = velocity.y.atan2(velocity.x) - FRAC_PI_2;
    transform.rotation = Quat::from_rotation_z(angle);
}

fn stop_movement(projectile: &mut Mut<Projectile>) {
    projectile.speed_vector = Vec2::splat(0.0);
}

/// Fire new bullet - spawn entity and set its initial speed
/// listen on spacebar press
fn fire(
    mut commands: Commands,
    texture: Res<ProjectileTexture>,
    keyboard: Res<Input<KeyCode>>,
    mut projectile_state_query: ResMut<State<ProjectilePresent>>,
    aim_query: Query<&mut PlayerAim, With<Player>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_transform = player_query.get_single().unwrap();
    let aim = aim_query.get_single().unwrap();
    let angle: f32 = aim.angle; //Take value from aiming component

    //y = sin(a)*v
    //x = cos(a)*v

    if keyboard.pressed(KeyCode::Space)
        && projectile_state_query.current() == &ProjectilePresent::NA
    {
        let new_stationary_projectile: Entity = produce_projectile(
            &mut commands,
            &texture,
            // Vec2::new(-15.0 * MAP_TILE_SIZE, -8.0 * MAP_TILE_SIZE),
            Vec2::new(
                player_transform.translation.x,
                player_transform.translation.y,
            ),
        );

        commands
            .entity(new_stationary_projectile)
            .insert(Projectile {
                speed_vector: produce_initial_speed_vector(angle),
                variant: ProjectileType::HE,
            });

        //Change state - block multiple entities at once
        projectile_state_query
            .set(ProjectilePresent::PRESENT)
            .expect("Cannot change projectile state :/");
    }
}

fn produce_initial_speed_vector(angle: f32) -> Vec2 {
    let radian_angle: f32 = f32::to_radians(angle);
    Vec2::new(f32::cos(radian_angle), f32::sin(radian_angle)).mul(PROJECTILE_MUZZLE_VELOCITY)
}

fn produce_projectile(
    commands: &mut Commands,
    texture: &Res<ProjectileTexture>,
    initial_position: Vec2,
) -> Entity {
    let texture = texture.0.clone();
    return commands
        .spawn(SpriteBundle {
            texture,
            transform: Transform {
                translation: Vec3::new(initial_position.x, initial_position.y, Z_PROJECTILE),
                scale: Vec3::splat(1.0).mul(MAP_TILE_SIZE / 180.0),
                ..default()
            },
            visibility: Visibility { is_visible: true },
            ..default()
        })
        .insert(Projectile {
            speed_vector: Vec2::splat(0.0),
            variant: ProjectileType::HE,
        })
        .insert(Name::new("Projectile"))
        .id();
}
