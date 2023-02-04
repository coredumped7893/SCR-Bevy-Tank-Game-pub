use crate::configuration_properties::{MAP_TILE_SIZE, Z_MAP};
use bevy::asset::AssetServer;
use bevy::prelude::{
    default, Commands, Handle, Image, Name, Res, SpriteBundle, Transform, Vec3, Visibility,
};

pub fn spawn_mountain(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load("mountain.png");

    commands
        .spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-91.0, -16.0 * MAP_TILE_SIZE, Z_MAP + 1.0),
                ..default()
            },
            texture,
            visibility: Visibility { is_visible: true },
            ..default()
        })
        .insert(Name::new("Map_mountain"));
}

pub fn spawn_ground(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load("bg2.png");

    commands
        .spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -11.0 * MAP_TILE_SIZE, Z_MAP - 1.0),
                ..default()
            },
            texture,
            visibility: Visibility { is_visible: true },
            ..default()
        })
        .insert(Name::new("Map_ground"));
}
