pub const KEY_PLAYER_LEFT: KeyCode = KeyCode::KeyA;
pub const KEY_PLAYER_RIGHT: KeyCode = KeyCode::KeyD;
pub const KEY_PLAYER_UP: KeyCode = KeyCode::KeyW;
pub const KEY_PLAYER_DOWN: KeyCode = KeyCode::KeyS;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_player_move)
    }
}

pub fn check_player_move(player: Query<(&mut MapPosition), With<Player>, input: Res<ButtonInput<KeyCode>>) {
    if input.any_just_pressed(KEY_PLAYER_LEFT) {
        player.map_position(x: player.map_position.clone() + 1, y: player.map_position.clone())
    }
}