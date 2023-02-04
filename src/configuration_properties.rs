use bevy::prelude::Color;

pub const CLEAR_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

pub const RESOLUTION_RATIO: f32 = 16.0 / 9.0;

pub const WINDOW_HEIGHT: f32 = 800.0;

pub const MAP_TILE_SIZE: f32 = 20.0;

pub const PLAYER_MOVE_SPEED: f32 = 5.0;

pub const WINDOW_TITLE: &str = "RustiArti | SCR MM";

pub const MAP_LAYOUT_FILE_PATH: &str = "assets/map.mp";

pub const ASCII_SPRITES_FILE_MAP_PATH: &str = "Ascii.png";

pub const EXPLOSION_SPRITES_FILE_MAP_PATH: &str = "explosion_sprites.png";

pub const SPRITE_PROJECTILE_TEXTURE_FILE: &str = "D25T_shell.png";

pub const SPRITE_TANK_TEXTURE_FILE: &str = "tank_player.png";

pub const SPRITE_ENEMY_TANK_TEXTURE_FILE: &str = "tank_enemy.png";

pub const ASCII_SPRITES_PADDING: f32 = 2.0;

pub const ASCII_SPRITES_ROWS: usize = 16;

pub const ASCII_SPRITES_COLUMNS: usize = 16;

pub const EXPLOSION_SPRITES_ROWS: usize = 2;

pub const EXPLOSION_SPRITES_COLUMNS: usize = 4;

pub const ASCII_SPRITES_TILE_SIZE: f32 = 9.0;

pub const EXPLOSION_SPRITES_TILE_SIZE_X: f32 = 250.0;

pub const EXPLOSION_SPRITES_TILE_SIZE_Y: f32 = 350.0;

pub const Z_MAP: f32 = 100.0;

pub const Z_PLAYER: f32 = 150.0;

pub const Z_PROJECTILE: f32 = 200.0;

/// Kpep it relative to the map size
pub const MAP_GRAVITY: f32 = 3.0 * MAP_TILE_SIZE;

pub const PROJECTILE_MUZZLE_VELOCITY: f32 = 220.0;
