use crate::consts::*;
use crate::map::MapPosition;
use crate::unit::*;
use std::fmt;

use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub unit: Option<Unit>,
}

#[derive(Component, Clone, PartialEq)]
pub enum TileType {
    Floor,
    Wall,
}

#[derive(Component, Debug)]
pub struct Passable(pub bool);



impl TileType {
    pub fn to_sprite_id(&self) -> usize {
        match self {
            TileType::Floor => SPRITE_ID_FLOOR,
            TileType::Wall => SPRITE_ID_WALL,
        }
    }
    pub fn to_passable(&self) -> Passable {
        match self {
            TileType::Floor => Passable(true),
            TileType::Wall => Passable(false),
        }
    }
}

#[derive(Bundle)]
pub struct TileBundle {
    pub tile: Tile,
    pub position: MapPosition,
    pub passable: Passable,
    pub sprite: SpriteSheetBundle,
}