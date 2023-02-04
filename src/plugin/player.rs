use bevy::prelude::{
    default, App, BuildChildren, Camera, Children, Color, Commands, Component, Input,
    IntoSystemDescriptor, KeyCode, Name, Plugin, Query, Reflect, Res, SpriteBundle, StartupStage,
    SystemSet, TextureAtlasSprite, Time, Transform, Vec2, Vec3, Visibility, With, Without,
};

use crate::configuration_properties::{MAP_GRAVITY, MAP_TILE_SIZE, PLAYER_MOVE_SPEED, Z_PLAYER};
use crate::plugin::ascii_sprite::{spawn_ascii_sprite, spawn_ascii_text, AsciiSheet};
use crate::plugin::map::TileCollider;
use crate::state::MainGameState;
use crate::utilsystems::player_aim::{
    spawn_aim_crosshair, update_aim_position, PlayerAim, PlayerCrosshair, CH_RADIUS,
};
use crate::utilsystems::tank_sprite_resource::{
    load_enemy_tank_textures, load_tank_textures, TankTexture,
};
use crate::utilsystems::wall_collision::wall_collision_check;

pub struct PlayerPlugin;

#[derive(Component, Reflect)]
pub struct Player {
    pub(crate) speed: f32,
    just_moved: bool,
    ground_contact: bool,
    vertical_velocity: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_exit(MainGameState::COMBAT).with_system(hide_player))
            .add_system_set(SystemSet::on_enter(MainGameState::COMBAT).with_system(show_player))
            .add_system_set(
                SystemSet::on_update(MainGameState::COMBAT)
                    .with_system(camera_follow.after(player_movement))
                    .with_system(player_movement)
                    .with_system(update_aim_position),
            )
            .add_startup_system_to_stage(StartupStage::PreStartup, load_tank_textures)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_enemy_tank_textures)
            .add_startup_system(spawn_tank_player)
            .add_startup_system(spawn_aim_crosshair);
    }
}

fn hide_player(
    player_query: Query<&mut Visibility, With<Player>>,
    children_query: Query<&Children, With<Player>>,
    child_visibility_query: Query<&mut Visibility, Without<Player>>,
) {
    change_player_visibility(player_query, children_query, child_visibility_query, false);
}

fn show_player(
    player_query: Query<&mut Visibility, With<Player>>,
    children_query: Query<&Children, With<Player>>,
    child_visibility_query: Query<&mut Visibility, Without<Player>>,
) {
    change_player_visibility(player_query, children_query, child_visibility_query, true);
}

fn change_player_visibility(
    mut player_query: Query<&mut Visibility, With<Player>>,
    children_query: Query<&Children, With<Player>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Player>>,
    visible: bool,
) {
    let mut player_vis = player_query.single_mut();
    player_vis.is_visible = visible;

    if let Ok(children) = children_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                child_vis.is_visible = visible;
            }
        }
    }
}

type AimFilter = (
    With<PlayerCrosshair>,
    Without<Player>,
    Without<TileCollider>,
);

/// Handle player movement. moving up and down is disabled. Simple gravity is simulated
fn player_movement(
    mut player_query: Query<(&mut Player, &PlayerAim, &mut Transform)>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    mut aim_transform_query: Query<(&mut Transform), AimFilter>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player, player_aim, mut transform) = player_query.single_mut();
    let mut player_ch = aim_transform_query.single_mut();
    player.just_moved = false;

    player.vertical_velocity += -MAP_GRAVITY * time.delta_seconds();

    let y_delta: f32 = player.vertical_velocity * time.delta_seconds();

    let mut x_delta = 0.0;
    if keyboard.pressed(KeyCode::A) {
        x_delta -= player.speed * MAP_TILE_SIZE * time.delta_seconds()
    }
    if keyboard.pressed(KeyCode::D) {
        x_delta += player.speed * MAP_TILE_SIZE * time.delta_seconds()
    }

    let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
    if !wall_query
        .iter()
        .any(|t| wall_collision_check(t.translation, target))
    {
        if x_delta != 0.0 {
            player.just_moved = true;
        }
        player_ch.translation = target;
        player_ch.translation.x += CH_RADIUS * player_aim.angle.to_radians().cos();
        player_ch.translation.y += CH_RADIUS * player_aim.angle.to_radians().sin();

        transform.translation = target;
    }

    let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
    if !wall_query
        .iter()
        .any(|t| wall_collision_check(t.translation, target))
    {
        if y_delta != 0.0 {
            player.just_moved = true;
        }
        player.ground_contact = false;

        player_ch.translation = target;
        player_ch.translation.x += CH_RADIUS * player_aim.angle.to_radians().cos();
        player_ch.translation.y += CH_RADIUS * player_aim.angle.to_radians().sin();

        transform.translation = target;
    } else {
        player.ground_contact = true;
        player.vertical_velocity = 0.0;
    }
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x + 250.0;
    camera_transform.translation.y = player_transform.translation.y + 150.0;
}

fn spawn_tank_player(mut commands: Commands, texture: Res<TankTexture>) {
    commands
        .spawn(SpriteBundle {
            transform: Transform {
                scale: Vec3::splat(0.15),
                translation: Vec3::new(-400.0, -17.9 * MAP_TILE_SIZE, Z_PLAYER + 1.0),
                ..default()
            },
            texture: texture.0.clone(),
            visibility: Visibility { is_visible: true },
            ..default()
        })
        .insert(Name::new("Player_1"))
        .insert(PlayerAim { angle: 5.0 })
        .insert(Player {
            speed: PLAYER_MOVE_SPEED,
            just_moved: false,
            ground_contact: false,
            vertical_velocity: 10.0,
        });
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut sprite: TextureAtlasSprite = TextureAtlasSprite::new(13);
    sprite.color = Color::rgb(0.3, 0.3, 0.9);
    sprite.custom_size = Some(Vec2::splat(MAP_TILE_SIZE));

    let player_entity = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        10,
        Color::rgb(0.3, 0.3, 0.9),
        Vec3::new(2.0 * MAP_TILE_SIZE, -17.0 * MAP_TILE_SIZE, Z_PLAYER),
        "Player".to_string(),
    );

    let text_ent = spawn_ascii_text(&mut commands, &ascii, " ", Vec3::new(0.0, 50.0, 0.0));

    let _ = commands
        .entity(player_entity)
        .insert(Player {
            speed: PLAYER_MOVE_SPEED,
            just_moved: false,
            ground_contact: false,
            vertical_velocity: 10.0,
        })
        .insert(PlayerAim { angle: 5.0 })
        .add_child(text_ent)
        .id();

    let background = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        0,
        Color::rgb(0.1, 0.1, 0.1),
        Vec3::new(0.0, 0.0, -1.0),
        "Background".to_string(),
    );

    commands.entity(player_entity).push_children(&[background]);
}
