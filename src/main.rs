use bevy::prelude::*;
mod map;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, spawn_player))
        .add_systems(Update, player_movement)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera Setup
    commands.spawn((Camera2dBundle {
        transform: Transform::from_xyz(100.0, 200.0, 0.0),
        ..default()
    },
    MyCameraMarker,
    ));
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.;
        }

        transform.translation += direction * 2.;
    }
}

#[derive(Component)]
struct MyCameraMarker;

// Player
#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    // Component used to identify an entity part of bevy core
    name: Name,
    // A way to tag the player
    player: Player,
    // Eventually we will convert this to a spritesheet
    sprite: SpriteBundle,
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_sprite = asset_server.load("player.png");
    commands.spawn(PlayerBundle {
        player: Player,
        name: Name::new("Player"),
        sprite: SpriteBundle {
            texture: player_sprite,
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
    });
}