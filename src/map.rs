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

    pub fn as_sprite_coordinates(&self) -> (f32, f32) {
        (
            (self.x as f32 * SPRITE_TILE_WIDTH) + (SPRITE_TILE_WIDTH / 2.0),
            ((-1f32 * self.y as f32) * SPRITE_TILE_HEIGHT) + (-(SPRITE_TILE_HEIGHT / 2.0)),
        )
    }
    pub fn left(&self) -> Result<Self, String> {
        if self.x == 0 {
            return Err("left can't be out of bounds".into());
        }
        Ok(Self::new(self.x - 1, self.y))
    }

    pub fn right(&self) -> Result<Self, String> {
        Ok(Self::new(self.x + 1, self.y))
    }

    pub fn up(&self) -> Result<Self, String> {
        if self.y == 0 {
            return Err("up can't be out of bounds".into());
        }
        Ok(Self::new(self.x, self.y - 1))
    }

    pub fn down(&self) -> Result<Self, String> {
        Ok(Self::new(self.x, self.y + 1))
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

    pub fn as_tile_index(&self, pos: &MapPosition) -> Result<usize, String> {
        let index = pos.x + pos.y * self.width;
        if index >= self.tiles.len() {
            return Err("index out of bounds".into());
        }
        Ok(index)
    }

    pub fn move_unit(
        &mut self,
        previous_position: &mut MapPosition,
        new_position: &MapPosition,
    ) -> Result<(), String> {
        let index_from = self.as_tile_index(previous_position)?;
        let index_to = self.as_tile_index(new_position)?;
        self.tiles[index_to].unit = self.tiles[index_from].unit.take();
        *previous_position = *new_position;
    }

}

pub fn move_left(
    map: &mut Map,
    position: &mut MapPosition,
) -> Result<(), String> {
    map.move_unit(position, &position.left()?)?;
    Ok(())
}

pub fn move_right(
    map: &mut Map,
    position: &mut MapPosition,
) -> Result<(), String> {
    map.move_unit(position, &position.right()?)?;
    Ok(())
}

pub fn move_up(
    map: &mut Map,
    position: &mut MapPosition,
) -> Result<(), String> {
    map.move_unit(position, &position.up()?)?;
    Ok(())
}

pub fn move_down(
    map: &mut Map,
    position: &mut MapPosition,
) -> Result<(), String> {
    map.move_unit(position, &position.down()?)?;
    Ok(())
}