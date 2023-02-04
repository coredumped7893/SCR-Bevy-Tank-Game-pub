use crate::plugin::ascii_sprite::AsciiText;
use crate::plugin::enemy::Enemy;
use crate::plugin::explosion::Explosion;
use crate::plugin::player::Player;
use crate::plugin::projectile::{Projectile, ProjectileTexture};
use crate::utilsystems::player_aim::{PlayerAim, PlayerCrosshair};
use bevy::prelude::{App, Plugin};
use bevy_inspector_egui::quick::{
    ResourceInspectorPlugin, StateInspectorPlugin, WorldInspectorPlugin,
};

use crate::state::{MainGameState, ProjectilePresent};
use crate::utilsystems::tank_sprite_resource::{TankEnemyTexture, TankTexture};

pub(crate) struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin)
                .register_type::<Player>()
                .register_type::<Projectile>()
                .register_type::<Explosion>()
                .register_type::<AsciiText>()
                .register_type::<TankTexture>()
                .register_type::<PlayerCrosshair>()
                .register_type::<PlayerAim>()
                .register_type::<Enemy>()
                .register_type::<TankEnemyTexture>()
                .add_plugin(StateInspectorPlugin::<ProjectilePresent>::default())
                .add_plugin(StateInspectorPlugin::<MainGameState>::default());
            // .add_plugin(ResourceInspectorPlugin::<ProjectileTexture>::default());
        }
    }
}
