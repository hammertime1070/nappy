use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera Setup
    commands.spawn((Camera2dBundle {
        transform: Transform::from_xyz(100.0, 200.0, 0.0),
        ..default()
    },
    MyCameraMarker,
    ));
    // Load sprites
    commands.spawn(SpriteBundle {
        texture:asset_server.load("player.png"),
        ..default()
    });
}

#[derive(Component)]
struct MyCameraMarker;