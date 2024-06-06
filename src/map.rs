use bevy::prelude::*;

use crate::tile::TileType;

// struct MapPlugin;

// impl Plugin for MapPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(Startup, setup_map);
//     }
// }

// #[derive(Bundle)]
// struct TileBundle {
//     name: Name,
//     position: Position,
//     sprite: SpriteBundle,
//     passable: Passable,
// }


// #[derive(Component)]
// struct Position {
//     x: usize,
//     y: usize,
// }

// #[derive(Component)]
// struct Passable(bool);

// #[derive(Component)]
// struct Sprite {
//     sprite: Handle<Image>
// }

// fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let bush_sprite = asset_server.load("bush.png");
//     let bush_position = Position { x: 1, y: 1 };
//     commands.spawn(TileBundle {
//         position: bush_position,
//         name: Name::new("Bush"),
//         sprite: SpriteBundle {
//             texture: bush_sprite,
//             transform: Transform::from_xyz(bush_position.x as f32, bush_position.y as f32, 1.0),
//             ..default()
//         },
//         passable: Passable(false),
//     });
// }

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