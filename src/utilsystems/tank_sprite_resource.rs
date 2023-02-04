use bevy::prelude::{AssetServer, Commands, Handle, Image, Reflect, Res, Resource};

use crate::configuration_properties::{SPRITE_ENEMY_TANK_TEXTURE_FILE, SPRITE_TANK_TEXTURE_FILE};

#[derive(Resource, Reflect)]
pub struct TankTexture(pub Handle<Image>);

#[derive(Resource, Reflect)]
pub struct TankEnemyTexture(pub Handle<Image>);

pub fn load_tank_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load(SPRITE_TANK_TEXTURE_FILE);
    commands.insert_resource(TankTexture(texture));
}

pub fn load_enemy_tank_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load(SPRITE_ENEMY_TANK_TEXTURE_FILE);
    commands.insert_resource(TankEnemyTexture(texture));
}
