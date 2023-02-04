use bevy::app::App;
use bevy::prelude::{Assets, AssetServer, BuildChildren, Color, Commands, Component, default, Entity, Handle, Image, Name, Plugin, Reflect, Res, ResMut, Resource, SpatialBundle, SpriteSheetBundle, StartupStage, TextureAtlas, TextureAtlasSprite, Transform, Vec2, Vec3};

use crate::configuration_properties::{ASCII_SPRITES_COLUMNS, ASCII_SPRITES_FILE_MAP_PATH, ASCII_SPRITES_PADDING, ASCII_SPRITES_ROWS, ASCII_SPRITES_TILE_SIZE, MAP_TILE_SIZE};

///Load ascii sprites from png file and load them as a resource
pub struct AsciiSprite;

#[derive(Resource)]
pub struct AsciiSheet(pub Handle<TextureAtlas>);

#[derive(Component, Reflect)]
pub struct AsciiText;

impl Plugin for AsciiSprite {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_ascii_resource);
    }
}


fn load_ascii_resource(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image: Handle<Image> = assets.load(ASCII_SPRITES_FILE_MAP_PATH);
    let atlas: TextureAtlas = TextureAtlas::from_grid(
        image,
        Vec2::splat(ASCII_SPRITES_TILE_SIZE),
        ASCII_SPRITES_COLUMNS,
        ASCII_SPRITES_ROWS,
        Option::from(Vec2::splat(ASCII_SPRITES_PADDING)),
        None,
    );

    let atlas_handle: Handle<TextureAtlas> = texture_atlases.add(atlas);

    commands.insert_resource(AsciiSheet(atlas_handle));
}


pub fn spawn_ascii_text(
    commands: &mut Commands,
    ascii: &AsciiSheet,
    text_to_print: &str,
    position_offset: Vec3,
) -> Entity {
    let mut char_sprites: Vec<Entity> = Vec::new();
    let color = Color::rgb(0.5, 0.3, 0.7);
    for (i, char) in text_to_print.chars().enumerate() {
        assert!(char as usize <= 255);

        char_sprites.push(
            spawn_ascii_sprite(
                commands,
                ascii,
                char as usize,
                color,
                Vec3::new(i as f32 * MAP_TILE_SIZE, 0.0, 0.0),
                format!("text-letter{}-{}", i, char),
            )
        );
    }

    commands.spawn(SpatialBundle::default())
        .insert(Name::new(format!("Text:{}", text_to_print)))
        .insert(Transform {
            translation: position_offset,
            ..default()
        })
        .insert(AsciiText)
        .push_children(&char_sprites)
        .id()
}


pub fn spawn_ascii_sprite(
    commands: &mut Commands,
    ascii: &AsciiSheet,
    index: usize,
    color: Color,
    translation: Vec3,
    sprite_name: String,
) -> Entity {
    let mut sprite: TextureAtlasSprite = TextureAtlasSprite::new(index);
    sprite.color = color;
    sprite.custom_size = Some(Vec2::splat(MAP_TILE_SIZE));

    commands.spawn(SpriteSheetBundle {
        sprite,
        texture_atlas: ascii.0.clone(),
        transform: Transform {
            translation,
            ..default()
        },
        ..default()
    }).insert(Name::new(sprite_name))
        .id()
}

