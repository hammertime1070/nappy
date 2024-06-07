use bevy::asset;
use bevy::prelude::*;

mod consts;
mod map;
mod player;
mod tile;
mod input;

use consts::*;
use map::*;
use player::*;
use input::*;

use tile::Tile;
use tile::TileBundle;
use tile::TileType;
use tile::Passable;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin { primary_window: Some(Window { resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(), ..Default::default()}), ..Default::default() }).set(ImagePlugin::default_nearest()))
        .add_plugins(InputPlugin)
        .add_systems(Startup, setup)
        .run()
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle::default());
    let texture_handle = asset_server.load("sprite_sheet.png");
    let texture_atlas = TextureAtlasLayout::from_grid( Vec2::new(32.0, 32.0), 1, 4, None, None);
    let atlas_handle = texture_atlases.add(texture_atlas);
    let map = Map::new(16, 16);
    spawn_tiles(&mut commands, &map, &atlas_handle, &texture_handle);
    spawn_player(&mut commands, &atlas_handle, &texture_handle);
}

fn spawn_player(commands: &mut Commands, atlas_handle: &Handle<TextureAtlasLayout>, texture_handle: &Handle<Image>) {
    let map_position = MapPosition::new(0, 0);
    let (sprite_x, sprite_y) = calculate_sprite_position(&map_position);
    commands.spawn(PlayerBundle {
        player: Player,
        position: map_position,
        sprite: SpriteSheetBundle {
            atlas: TextureAtlas {
                layout: atlas_handle.clone(),
                index: SPRITE_ID_PLAYER,
            },
            texture: texture_handle.clone(),
            transform: Transform::from_xyz(sprite_x, sprite_y, Z_INDEX_PLAYER),
            ..Default::default()
        },
    });
}

fn spawn_tiles(
    commands: &mut Commands,
    map: &Map,
    atlas_handle: &Handle<TextureAtlasLayout>,
    texture_handle: &Handle<Image>,
) {
    for (tile_i, tile_type) in map.tiles.iter().enumerate() {
        let map_position = MapPosition {
            x: tile_i % map.width,
            y: tile_i / map.width,
        };
        let (sprite_x, sprite_y) = calculate_sprite_position(&map_position);
        commands.spawn(TileBundle {
            tile: Tile,
            tiletype: tile_type.clone(),
            position: map_position,
            passable: tile_type.to_passable(),
            sprite: SpriteSheetBundle {
                atlas: TextureAtlas {
                    layout: atlas_handle.clone(),
                    index: tile_type.to_sprite_id(),
                },
                texture: texture_handle.clone(),
                transform: Transform::from_xyz(sprite_x, sprite_y, Z_INDEX_TERRAIN),
                ..Default::default()
            }
        });
    }
}

fn calculate_sprite_position(map_position: &MapPosition) -> (f32, f32) {
    let top_left_x = WINDOW_WIDTH / -2.0;
    let top_left_y = WINDOW_HEIGHT / 2.0;
    (
        top_left_x
            + map_position.x as f32 * SPRITE_TILE_WIDTH
            + SPRITE_TILE_WIDTH / 2.0,
        top_left_y
            - map_position.y as f32 * SPRITE_TILE_HEIGHT
            - SPRITE_TILE_HEIGHT / 2.0,
    )
}