use crate::configuration_properties::{MAP_TILE_SIZE, Z_MAP, Z_PLAYER};
use crate::plugin::player::Player;
use bevy::prelude::*;
use std::ops::Mul;

#[derive(Component, Reflect)]
pub struct PlayerAim {
    pub(crate) angle: f32,
}

pub const CH_RADIUS: f32 = 60.0;

#[derive(Component, Reflect)]
pub struct PlayerCrosshair;

pub fn spawn_aim_crosshair(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load("ch1.png");

    commands
        .spawn(SpriteBundle {
            transform: Transform {
                scale: Vec3::splat(1.0).mul(MAP_TILE_SIZE / 170.0),
                translation: Vec3::new(0.0, -1000.0 * MAP_TILE_SIZE, Z_PLAYER + 1.0),
                ..default()
            },
            texture,
            visibility: Visibility { is_visible: true },
            ..default()
        })
        .insert(PlayerCrosshair {})
        .insert(Name::new("aim_ch"));
}

pub fn update_aim_position(
    keyboard: Res<Input<KeyCode>>,
    mut aim_query: Query<&mut PlayerAim, With<Player>>,
) {
    let mut aim_struct = aim_query.get_single_mut().unwrap();

    if keyboard.pressed(KeyCode::W) {
        aim_struct.angle += 0.07;
    }
    if keyboard.pressed(KeyCode::S) {
        aim_struct.angle -= 0.07;
    }
}
