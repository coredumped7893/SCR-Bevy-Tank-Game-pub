use std::fs::File;
use std::io::{BufRead, BufReader};

use bevy::app::App;
use bevy::prelude::{
    BuildChildren, Color, Commands, Component, GlobalTransform, Name, Plugin, Query, Res,
    SpatialBundle, SystemSet, Transform, Vec3, Visibility, With,
};

use crate::configuration_properties::{
    MAP_LAYOUT_FILE_PATH, MAP_TILE_SIZE, RESOLUTION_RATIO, WINDOW_HEIGHT, Z_MAP,
};
use crate::plugin::ascii_sprite::{spawn_ascii_sprite, AsciiSheet};
use crate::state::MainGameState;
use crate::utilsystems::map_textures::{spawn_ground, spawn_mountain};

pub struct MapPlugin;

#[derive(Component)]
pub struct Map;

///Handle player collisions with map
#[derive(Component)]
pub struct TileCollider;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(MainGameState::COMBAT).with_system(show_map))
            .add_system_set(SystemSet::on_exit(MainGameState::COMBAT).with_system(hide_map))
            .add_startup_system(generate_map_outline)
            .add_startup_system(spawn_mountain)
            .add_startup_system(spawn_ground);
    }
}

fn set_map_visibility(mut map_vis: Query<&mut Visibility, With<Map>>, visible: bool) {
    let mut map_visibility = map_vis.single_mut();
    map_visibility.is_visible = visible;
}

fn hide_map(map_vis: Query<&mut Visibility, With<Map>>) {
    eprintln!("Hide map");
    set_map_visibility(map_vis, false);
}

fn show_map(map_vis: Query<&mut Visibility, With<Map>>) {
    eprintln!("Show map");
    set_map_visibility(map_vis, true);
}

fn generate_map_outline(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let file: File = File::open(MAP_LAYOUT_FILE_PATH).expect("No map file found");
    let mut tiles = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                if char == '.' {
                    continue; //We don't need to render dots. Usable for debug purposes ;)
                }
                let map_tile_entity = spawn_ascii_sprite(
                    &mut commands,
                    &ascii,
                    // char as usize,
                    ' ' as usize,
                    Color::WHITE,
                    Vec3::new(
                        x as f32 * MAP_TILE_SIZE + (RESOLUTION_RATIO * WINDOW_HEIGHT / -2.0),
                        -(y as f32) * MAP_TILE_SIZE,
                        Z_MAP,
                    ),
                    //Vec3::new(x as f32 * MAP_TILE_SIZE + (RESOLUTION_RATIO*WINDOW_HEIGHT/-2.0) + 30.0, -(y as f32) * MAP_TILE_SIZE - (WINDOW_HEIGHT/-4.0), Z_MAP),
                    format!("BG_tile{}-{}", char, x),
                );

                if char == '#' {
                    commands.entity(map_tile_entity).insert(TileCollider);
                }

                tiles.push(map_tile_entity);
            }
        }
    }

    commands
        .spawn(SpatialBundle::default())
        .insert(Map)
        .insert(Visibility { is_visible: true })
        .insert(Name::new("Map"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&tiles);
}
