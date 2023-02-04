use crate::configuration_properties::MAP_TILE_SIZE;
use bevy::prelude::{Vec2, Vec3};
use bevy::sprite::collide_aabb::collide;

pub fn wall_collision_check(wall_translation: Vec3, target_player_pos: Vec3) -> bool {
    let collision = collide(
        target_player_pos,
        Vec2::splat(MAP_TILE_SIZE * 0.95),
        wall_translation,
        Vec2::splat(MAP_TILE_SIZE),
    );
    collision.is_some()
}
