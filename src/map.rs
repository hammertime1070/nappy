use bevy::prelude::*;
use rand::Rng;
use rand::prelude::SliceRandom;

use crate::tile::*;
use crate::consts::*;



#[derive(Component, Clone, Copy)]
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
    pub tiles: Vec<Tile>,
    pub entities: Vec<Option<Entity>>,
}

impl Map {
    // pub fn new(width: usize, height: usize) -> Self {
    //     let mut tiles = Vec::new();
    //     for i in 0..(width * height) {
    //         let x = i / width;
    //         let y = i % height;
    //         if y == 0 || y == height -1 || x == width - 1 {
    //             tiles.push(Tile {tile_type: TileType::Wall, unit: None })
    //         } else {
    //             tiles.push(Tile {tile_type: TileType::Floor, unit: None })
    //         }
    //     }
    //     return Map {
    //         width,
    //         height,
    //         tiles: tiles.clone(),
    //         entities: vec![None; tiles.len()],
    //     };
    // }

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
        println!("moving unit");
        Ok(())
    }

    // Flood Fill Algorithm changes
    pub fn new_ff(width: usize, height: usize) -> Self {
        let mut tiles = vec![Tile { tile_type: TileType::Floor, unit: None }; width * height];
        for x in 0..width {
            for y in 0..height {
                let index = x + y * width;
                if x == 0 || y == 0 || x == width - 1 || y == height - 1 || rand::random::<f32>() < 0.3 {
                    tiles[index] = Tile { tile_type: TileType::Wall, unit: None };
                } else {
                    tiles[index] = Tile { tile_type: TileType::Floor, unit: None };
                }
            }
        }
        // Create the map
        let mut map = Self { width, height, tiles: tiles.clone(), entities: vec![None; tiles.len()]};
        // Ensure player spawn is changed to a floor tile
        let spawn_position = map.select_player_spawn_location();
        let spawn_index = spawn_position.x + spawn_position.y * width;
        map.tiles[spawn_index] = Tile { tile_type: TileType::Floor, unit: None };

        // Ensure player exit is changed to a floor tile
        let exit_position = map.select_player_exit_location();
        let exit_index = exit_position.x + exit_position.y * width;
        map.tiles[exit_index] = Tile { tile_type: TileType::Floor, unit: None };

        // Check map connectivity
        map.ensure_map_connectivity(spawn_position, exit_position);
        return map
    }

    pub fn in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize
    }

    pub fn get_tile(&self, x: isize, y: isize) -> Option<&Tile> {
        if self.in_bounds(x, y) {
            let index = x + y * self.width as isize;
            Some(&self.tiles[index as usize])
        } else {
            None
        }
    }

    pub fn random_passable_tile(&self) -> Option<(usize, usize)> {
        let mut rng = rand::thread_rng();
        let mut tries = 1000;
        while tries > 0 {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);
            if let Some(tile) = self.get_tile(x as isize, y as isize) {
                // TODO make this look less stupid
                if tile.tile_type.to_passable().0 {
                    return Some((x, y));
                }
            }
            tries -= 1;
        }
        None
    }

    pub fn select_player_spawn_location(&self) -> MapPosition {
        let x = self.width /2;
        let y = self.height - 1;
        if let Some(tile) = self.get_tile(x as isize, y as isize) {
            return MapPosition::new(x, y);
        }
        MapPosition::new(0, 0)
    }

    pub fn select_player_exit_location(&self) -> MapPosition {
        let x = self.width /2;
        let y = 0;
        if let Some(tile) = self.get_tile(x as isize, y as isize) {
            return MapPosition::new(x, y);
        }
        MapPosition::new(1, 1)
    }

    pub fn ensure_map_connectivity(&mut self, start_pos: MapPosition, exit_pos: MapPosition) {
        let mut visited = vec![false; self.width * self.height];
        let start_index = start_pos.x + start_pos.y * self.width;
        let mut queue = vec![(start_pos.x, start_pos.y)];
        visited[start_index] = true;
    
        // Perform BFS to find all connected floor tiles
        while let Some((x, y)) = queue.pop() {
            for (nx, ny) in self.get_passable_neighbors(x as isize, y as isize) {
                let index = nx as usize + ny as usize * self.width;
                if !visited[index] {
                    visited[index] = true;
                    queue.push((nx as usize, ny as usize));
                }
            }
        }
    
        // Get the number of passable tiles found during BFS
        let mut connected_passable_count = visited.iter().filter(|&v| *v).count();
    
        // Check if the number of connected passable tiles matches the total passable tiles
        let total_passable_count = self.tiles.iter().filter(|t| t.tile_type.to_passable().0).count();
        println!("Connected Passable Count: {}, Total Passable Count: {}", connected_passable_count, total_passable_count);
        if connected_passable_count == total_passable_count {
            return; // No need to shuffle
        }
    
        // Exclude player spawn and exit from passable indices
        let mut passable_indices: Vec<usize> = (0..self.tiles.len())
            .filter(|&i| self.tiles[i].tile_type.to_passable().0 && i != start_index && i != exit_pos.x + exit_pos.y * self.width)
            .collect();
    
        // Perform Fisher-Yates shuffle until passable tile count matches
        let mut rng = rand::thread_rng();
        while connected_passable_count != total_passable_count {
                passable_indices.shuffle(&mut rng);
                println!("Shuffling passable_indices");
                let mut shuffled_indices_iter = passable_indices.iter().copied();
                while let Some(index) = shuffled_indices_iter.next() {
                if !visited[index] {
                    visited[index] = true;
                    connected_passable_count += 1;
                    println!("Connected Passable Count: {}, Total Passable Count: {}", connected_passable_count, total_passable_count);
                if connected_passable_count == total_passable_count {
                    return; // All passable tiles are now connected
                    }
                }
            }
        }
    }

    // Helper function for checking map connectivity, returns a vector for coordinates of a tile
    pub fn get_passable_neighbors(&self, x: isize, y: isize) -> Vec<(isize, isize)> {
        let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        let mut neighbors = Vec::new();
        for &(dx, dy) in &directions {
            let nx = x + dx;
            let ny = y + dy;
            if self.in_bounds(nx, ny) {
                let index = nx as usize + ny as usize * self.width;
                if self.tiles[index].tile_type.to_passable().0 {
                    neighbors.push((nx, ny));
                }
            }
        }
        neighbors
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