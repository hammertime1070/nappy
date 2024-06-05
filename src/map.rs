use bevy::prelude::*;

struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_map);
    }
}

#[derive(Bundle)]
struct TileBundle {
    name: Name,
    position: Position,
    sprite: SpriteBundle,
    passable: Passable,
}


#[derive(Component)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Component)]
struct Passable(bool);

#[derive(Component)]
struct Sprite {
    sprite: Handle<Image>
}

fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bush_sprite = asset_server.load("bush.png");
    let bush_position = Position { x: 1, y: 1 };
    commands.spawn(TileBundle {
        position: bush_position,
        name: Name::new("Bush"),
        sprite: SpriteBundle {
            texture: bush_sprite,
            transform: Transform::from_xyz(bush_position.x as f32, bush_position.y as f32, 1.0),
            ..default()
        },
        passable: Passable(false),
    });
}