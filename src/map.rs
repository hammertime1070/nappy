use bevy::prelude::*;
use rand::Rng;
use rand::prelude::SliceRandom;

use crate::tile::*;
use crate::consts::*;



#[derive(Component, Clone, Copy, PartialEq)]
pub struct MapPosition {
    pub x: usize,
    pub y: usize,
}

impl MapPosition {
    pub fn new(x: usize, y: usize) -> Self {
        MapPosition{ x, y }
    }

    pub fn distance(&self, target_pos: &MapPosition) -> usize {
        // This is an annoying way to get the manhattan distance
        // Turns out using usizes for stuff is very annoying
        let dx = if self.x > target_pos.x { self.x - target_pos.x } else { target_pos.x - self.x };
        let dy = if self.y > target_pos.y { self.y - target_pos.y } else { target_pos.y - self.y };
        return dx + dy;
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

#[derive(Component, Clone)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Tile>,
    pub entities: Vec<Option<Entity>>,
}

impl Map {
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

    // TODO: Incorporate this function to simplify movements
    pub fn check_if_valid_move(&mut self, new_position: &MapPosition) -> Result<(bool), String> {
        let index_to = self.as_tile_index(new_position)?;
        let target_tile = self.tiles[index_to];
        if target_tile.is_walkable() && !target_tile.is_occupied() {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn new_rd(width: usize, height: usize) -> Self {
        let mut tiles = Vec::new();
        // Ensure dimensions are odd
        let width = if width % 2 == 0 { width + 1 } else { width };
        let height = if height % 2 == 0 { height + 1 } else { height };
        // Initialize Walls and Corners as Walls
        for i in 0..(width * height) {
            let (x, y) = index_to_coords(i, width);
            if x == 0 || y == 0 || y == height -1 || x == width - 1 {
                tiles.push(Tile {tile_type: TileType::Wall, unit: None })
            } else {
                tiles.push(Tile {tile_type: TileType::Floor, unit: None })
            }
        }
        // Perform recursive division maze generation
        Self::divide(&mut tiles, 0, 0, width, height, choose_orientation(width, height), width);
        // Create the map object
        let mut map = Self {
            width,
            height,
            tiles,
            entities: vec![None; width * height],
        };
         // Ensure player spawn is changed to a floor tile
         let spawn_position = map.select_player_spawn_location();
         let spawn_index = spawn_position.x + spawn_position.y * width;
         map.tiles[spawn_index] = Tile { tile_type: TileType::Floor, unit: None };
 
         // Ensure player exit is changed to a floor tile
         let exit_position = map.select_player_exit_location();
         let exit_index = exit_position.x + exit_position.y * width;
         map.tiles[exit_index] = Tile { tile_type: TileType::Floor, unit: None };

        // Ensure tiles above entrance and below exit are floor tiles
        //TODO: Fix this obvious hacked together garbage
        // Currently there is a bug where if there is a central corridor wall the ensuring connectivity deletes all the walls
        if spawn_position.y > 0 {
            let above_spawn_index = spawn_index - width;
            map.tiles[above_spawn_index].tile_type = TileType::Floor;
        }
        if exit_position.y < height - 1 {
            let below_exit_index = exit_index + width;
            map.tiles[below_exit_index].tile_type = TileType::Floor;
        }

         // Ensure map connectivity
         let mut connected_map = map.clone();
         connected_map.ensure_map_connectivity(spawn_position, exit_position);
        
        return connected_map;

    }

    fn divide(tiles: &mut [Tile], x: usize, y: usize, width: usize, height: usize, orientation: char, map_width: usize) {
        let minimum_wall_size: usize = 8;
        // Base Case
        if width < minimum_wall_size || height < minimum_wall_size {
            return;
        }

        let horizontal = orientation == 'h';

        // Determine where wall will be drawn
        let mut wx = x + if horizontal { 0 } else {rand::thread_rng().gen_range(0..width - 1) };
        let mut wy = y + if horizontal { rand::thread_rng().gen_range(0..height - 1) } else { 0 };

        // Where will passage be drawn
        let px = wx + if horizontal { rand::thread_rng().gen_range(0..width) } else { 0 };
        let py = wy + if horizontal { 0 } else { rand::thread_rng().gen_range(0..height) };

        // What direction will the wall be drawn
        let dx = if horizontal { 1 } else { 0 };
        let dy = if horizontal { 0 } else { 1 };

        // How long will the wall be
        let length = if horizontal { width } else { height };

        // Create the wall 
        for _ in 0..length {
            if wx != px || wy != py {
                let index = coords_to_index(wx, wy, map_width);
                tiles[index].tile_type = TileType::Wall;
                println!("Placed wall at ({}, {})", wx, wy);
            }
            wx += dx;
            wy += dy;
        }

        // Determine the bounds of the created rooms and recurse
        let (nx, ny, w, h) = if horizontal {
            (x, y, width, wy - y)
        } else {
            (x, y, wx - x, height)
        };
        Self::divide(tiles, nx, ny, w, h, choose_orientation(w, h), map_width);
        let (nx, ny, w, h) = if horizontal {
            (x, wy + 1, width, y + height - wy - 1)
        } else {
            (wx + 1, y, x + width - wx - 1, height)
        };
        Self::divide(tiles, nx, ny, w, h, choose_orientation(w, h), map_width);

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

    pub fn random_passable_tile(&self) -> MapPosition {
        let mut rng = rand::thread_rng();
        let mut tries = 1000;
        while tries > 0 {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);
            if let Some(tile) = self.get_tile(x as isize, y as isize) {
                // TODO make this look less stupid
                if tile.tile_type.to_passable().0 {
                    return MapPosition::new(x, y);
                }
            }
            tries -= 1;
        }
        return MapPosition::new(0, 0);
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
        println!("Checking Map connectivity");
        let mut visited = vec![false; self.width * self.height];
        let start_index = coords_to_index(start_pos.x, start_pos.y, self.width);
        let mut queue = vec![(start_pos.x, start_pos.y)];
        visited[start_index] = true;
    
        // Perform BFS to find all connected floor tiles
        while let Some((x, y)) = queue.pop() {
            for (nx, ny) in self.get_passable_neighbors(x as isize, y as isize) {
                let index = coords_to_index(nx as usize, ny as usize, self.width);
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
        // Identify disconnected areas
        let mut disconnected_tiles = vec![];
        for (index, &is_visited) in visited.iter().enumerate() {
            if self.tiles[index].tile_type == TileType::Floor && !is_visited {
                disconnected_tiles.push(index);
            }
        }

        for index in disconnected_tiles {
            let (x, y) = index_to_coords(index as usize, self.width); 
            // modify existing walls to create a connection
            for (nx, ny) in self.get_all_neighbors(x as isize, y as isize) {
                let adj_index = coords_to_index(nx as usize, ny as usize, self.width);
                if self.tiles[adj_index].tile_type == TileType::Wall && !self.is_in_border(nx as isize, ny as isize) {
                    println!("Tile at {}{} is in border is {}", nx, ny, self.is_in_border(nx, ny));
                    // Turn it into a floor
                    self.tiles[adj_index].tile_type = TileType::Floor;
                    println!("Changed wall at ({}, {}) to floor", nx, ny);
                    self.ensure_map_connectivity(start_pos, exit_pos);
                    return;
                }
            }
        }
    }

    fn is_in_border(&self, x: isize, y: isize) -> bool {
        x == 0 || y == 0 || x == self.width as isize - 1 || y == self.height as isize - 1
    }

    pub fn get_all_neighbors(&self, x:isize, y: isize) -> Vec<(isize, isize)> {
        let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        let mut neighbors = Vec::new();
        for &(dx, dy) in &directions {
            let nx = x + dx;
            let ny = y + dy;
            if self.in_bounds(nx, ny) {
                let index = nx as usize + ny as usize * self.width;
                    neighbors.push((nx, ny));
            }
        }
        return neighbors;
    }

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

/// Returns a vector of reachable positions from a specific map position.
pub fn enumerate_reachable_positions(
    position: &MapPosition,
    map: &Map,
) -> Vec<MapPosition> {
    let mut reachable_positions: Vec<MapPosition> = vec![];

    if can_move_left(position, map) {
        reachable_positions.push(MapPosition {
            x: position.x - 1,
            y: position.y,
        });
    }
    if can_move_right(position, map) {
        reachable_positions.push(MapPosition {
            x: position.x + 1,
            y: position.y,
        });
    }
    if can_move_up(position, map) {
        reachable_positions.push(MapPosition {
            x: position.x,
            y: position.y - 1,
        });
    }
    if can_move_down(position, map) {
        reachable_positions.push(MapPosition {
            x: position.x,
            y: position.y + 1,
        });
    }
    for position in &reachable_positions {
        println!("Map Position: x = {}, y = {}", position.x, position.y);
    }
    return reachable_positions;
}

pub fn can_move_left(pos: &MapPosition, map: &Map) -> bool {
    if pos.x > 0 {
        let index = pos.x + pos.y * map.width -1;
        let tile = map.tiles[index];
        tile.is_walkable() && !tile.is_occupied()
    } else {
        false
    }
}

pub fn can_move_right(pos: &MapPosition, map: &Map) -> bool {
    if pos.x < map.width - 1 {
        let index = pos.x + pos.y * map.width + 1;
        let tile = map.tiles[index];
        tile.is_walkable() && !tile.is_occupied()
    } else {
        false
    }
}

pub fn can_move_up(pos: &MapPosition, map: &Map) -> bool {
    if pos.y > 0 {
        let index = pos.x + (pos.y - 1) * map.width;
        let tile = map.tiles[index];
        tile.is_walkable() && !tile.is_occupied()
    } else {
        false
    }
}

pub fn can_move_down(pos: &MapPosition, map: &Map) -> bool {
    if pos.y < map.height - 1 {
        let index = pos.x + (pos.y + 1) * map.width;
        let tile = map.tiles[index];
        tile.is_walkable() && !tile.is_occupied()
    } else {
        false
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

// Helper functions for recursive division

pub fn coords_to_index(x: usize, y: usize, width: usize) -> usize {
    return x + width * y;
}

pub fn index_to_coords(index: usize, width: usize) -> (usize, usize) {
    let x = index % width;
    let y = index / width;
    return (x, y);
}

pub fn choose_orientation(width: usize, height: usize) -> char {
    if width < height {
        // HORIZONTAL
        return 'h';
    } else if height < width {
        // VERTICAL
        return 'v';
    } else {
        if rand::random::<bool>() {
            return 'h';
        } else {
            return 'v';
        }
    }
}
