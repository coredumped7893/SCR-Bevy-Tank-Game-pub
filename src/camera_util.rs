use bevy::prelude::{Camera2dBundle, Commands};

pub(crate) fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}