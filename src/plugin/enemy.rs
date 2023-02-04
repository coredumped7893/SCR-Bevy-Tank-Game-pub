use crate::configuration_properties::*;
use crate::utilsystems::enemy_position_provider::generate_random_position;
use crate::utilsystems::tank_sprite_resource::TankEnemyTexture;
use bevy::app::App;
use bevy::prelude::*;

pub struct EnemyPlugin;

#[derive(Component, Reflect)]
pub struct Enemy;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_enemy);
    }
}

fn spawn_enemy(mut commands: Commands, texture: Res<TankEnemyTexture>) {
    commands
        .spawn(SpriteBundle {
            transform: Transform {
                scale: Vec3::splat(0.013),
                translation: Vec3::new(
                    generate_random_position(),
                    -17.9 * MAP_TILE_SIZE,
                    Z_PLAYER + 1.0,
                ),
                ..default()
            },
            texture: texture.0.clone(),
            visibility: Visibility { is_visible: true },
            ..default()
        })
        .insert(Name::new("Enemy_1"))
        .insert(Enemy {});
}
