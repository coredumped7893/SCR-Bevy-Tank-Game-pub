use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::{default, App, ClearColor, PluginGroup, WindowDescriptor, WindowPlugin};
use bevy::window::PresentMode;
use bevy::DefaultPlugins;
use PresentMode::AutoVsync;

use configuration_properties as config;

use crate::camera_util::spawn_camera;
use crate::config::{RESOLUTION_RATIO, WINDOW_HEIGHT};
use crate::configuration_properties::WINDOW_TITLE;
use crate::plugin::ascii_sprite::AsciiSprite;
use crate::plugin::debug::DebugPlugin;
use crate::plugin::enemy::EnemyPlugin;
use crate::plugin::explosion::ExplosionPlugin;
use crate::plugin::map::MapPlugin;
use crate::plugin::player::PlayerPlugin;
use crate::plugin::projectile::ProjectilePlugin;
use crate::state::MainGameState;

mod plugin;
mod state;

mod camera_util;
mod configuration_properties;
mod utilsystems;

fn main() {
    App::new()
        .insert_resource(ClearColor(config::CLEAR_COLOR))
        .add_state(MainGameState::COMBAT)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: WINDOW_TITLE.to_string(),
                width: WINDOW_HEIGHT * RESOLUTION_RATIO,
                height: WINDOW_HEIGHT,
                present_mode: AutoVsync,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(spawn_camera)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(AsciiSprite)
        .add_plugin(PlayerPlugin)
        .add_plugin(ProjectilePlugin)
        .add_plugin(ExplosionPlugin)
        .add_plugin(EnemyPlugin)
        .run();
}
