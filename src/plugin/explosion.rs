use crate::configuration_properties::EXPLOSION_SPRITES_FILE_MAP_PATH;
use crate::configuration_properties::EXPLOSION_SPRITES_ROWS;
use crate::configuration_properties::EXPLOSION_SPRITES_TILE_SIZE_X;
use crate::configuration_properties::EXPLOSION_SPRITES_TILE_SIZE_Y;
use crate::configuration_properties::{EXPLOSION_SPRITES_COLUMNS, MAP_TILE_SIZE};
use std::ops::Mul;

use bevy::app::App;
use bevy::prelude::{
    default, AssetServer, Assets, Commands, Component, DespawnRecursiveExt, Entity, Handle, Image,
    Name, Plugin, Query, Reflect, Res, ResMut, Resource, SpriteSheetBundle, StartupStage,
    TextureAtlas, TextureAtlasSprite, Time, Timer, Transform, Vec2, Vec3,
};
use bevy::time::TimerMode;

pub struct ExplosionPlugin;

#[derive(Component, Reflect)]
pub struct Explosion {
    timer: Timer,
    next_frame: Option<u16>,
}

#[derive(Resource)]
pub struct ExplosionTexturesSheet(pub Handle<TextureAtlas>);

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_explosion_sprites)
            // .add_startup_system(test_animation)
            .add_system(animate_frames);
    }
}

fn load_explosion_sprites(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image: Handle<Image> = assets.load(EXPLOSION_SPRITES_FILE_MAP_PATH);
    let atlas: TextureAtlas = TextureAtlas::from_grid(
        image,
        Vec2::new(EXPLOSION_SPRITES_TILE_SIZE_X, EXPLOSION_SPRITES_TILE_SIZE_Y),
        EXPLOSION_SPRITES_COLUMNS,
        EXPLOSION_SPRITES_ROWS,
        Option::from(Vec2::splat(1.0)),
        None,
    );

    let atlas_handle: Handle<TextureAtlas> = texture_atlases.add(atlas);

    commands.insert_resource(ExplosionTexturesSheet(atlas_handle));
}

fn animate_frames(
    mut explosions_query: Query<(Entity, &mut Explosion, &mut TextureAtlasSprite)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    //Every time timer finished, advance animation by one frame
    //If it was last frame, despawn entity
    for (entity, mut explosion_component, sprite) in explosions_query.iter_mut() {
        explosion_component.timer.tick(time.delta()); //Advance internal animation timer for given entity
        if explosion_component.timer.just_finished() {
            handle_next_frame(&entity, &mut commands, sprite.into_inner());
        }
    }
}

//Switch frames or despawn
fn handle_next_frame(
    entity: &Entity,
    commands: &mut Commands,
    explosion_sprite: &mut TextureAtlasSprite,
) {
    if explosion_sprite.index < 7 {
        explosion_sprite.index += 1;
    } else {
        commands.entity(*entity).despawn_recursive();
    }
}

fn test_animation(mut commands: Commands, sheet_query: Res<ExplosionTexturesSheet>) {
    spawn_animation(&mut commands, Vec3::new(0.0, -140.0, 200.0), &sheet_query);
}

pub fn spawn_animation(
    commands: &mut Commands,
    position: Vec3,
    sheet: &ExplosionTexturesSheet,
) -> Entity {
    let sprite: TextureAtlasSprite = TextureAtlasSprite::new(0);

    commands
        .spawn(SpriteSheetBundle {
            sprite,
            transform: Transform {
                scale: Vec3::splat(1.0).mul(MAP_TILE_SIZE / 60.0),
                translation: position,
                ..default()
            },
            texture_atlas: sheet.0.clone(),
            ..default()
        })
        .insert(Explosion {
            timer: Timer::from_seconds(0.07, TimerMode::Repeating), //@TODO Move time to properties
            next_frame: Some(1), //Animation always starts from first frame, so naturally index 1 is always the next one :)
        })
        .insert(Name::new("Exp"))
        .id()
}
