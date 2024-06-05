use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
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

    // Draw the Player
    let player_sprite = asset_server.load("player.png");
    commands.insert_resource(PlayerSprite(player_sprite.clone()));
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(player_sprite.into()),
        transform: Transform: Vec3::new(0.0, 0.0, 0.0),
        ..default()
    },
    ..default())
    .insert(Player);
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::W) {
            direction.y += 1.;
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction.y -= 1.;
        }
        if keyboard_input.pressed(KeyCode::A) {
            direction.x -= 1.;
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction.x += 1.;
        }

        transform.translation += direction * 2.;
    }
}

#[derive(Component)]
struct MyCameraMarker;

#[derive(Component)]
struct Player;

struct PlayerSprite(Handle<Texture>);

//TODO: Accidentally broke everything