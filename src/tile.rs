use crate::consts::*;
use crate::map::MapPosition;

use bevy::prelude::*;

#[derive(Component)]
pub struct Tile;

#[derive(Component, Clone)]
pub enum TileType {
    Floor,
    Wall,
}

#[derive(Component)]
pub struct Passable(pub bool);



impl TileType {
    pub fn to_sprite_id(&self) -> usize {
        match self {
            TileType::Floor => SPRITE_ID_FLOOR,
            TileType::Wall => SPRITE_ID_WALL,
        }
    }
}

#[derive(Bundle)]
pub struct TileBundle {
    pub tile: Tile,
    pub tiletype: TileType,
    pub position: MapPosition,
    pub passable: Passable,
    pub sprite: SpriteSheetBundle,
}