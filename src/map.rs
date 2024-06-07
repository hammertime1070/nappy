use bevy::prelude::*;

use crate::tile::TileType;


#[derive(Component)]
pub struct MapPosition {
    pub x: usize,
    pub y: usize,
}

impl MapPosition {
    pub fn new(x: usize, y: usize) -> Self {
        MapPosition{ x, y }
    }
}

#[derive(Component)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<TileType>,
    pub entities: Vec<Option<Entity>>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let mut tiles = Vec::new();
        for i in 0..(width * height) {
            let x = i / width;
            let y = i % height;
            if y == 0 || y == height -1 || x == width - 1 {
                tiles.push(TileType::Wall)
            } else {
                tiles.push(TileType::Floor)
            }
        }
        return Map {
            width,
            height,
            tiles: tiles.clone(),
            entities: vec![None; tiles.len()],
        };
    }

    pub fn add_entity(&mut self, entity: Entity, pos_x: usize, pos_y: usize) {
        self.entities[pos_x + pos_y * self.width] = Some(entity)
    }
}