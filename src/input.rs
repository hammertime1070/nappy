pub const KEY_PLAYER_LEFT: KeyCode = KeyCode::KeyA;
pub const KEY_PLAYER_RIGHT: KeyCode = KeyCode::KeyD;
pub const KEY_PLAYER_UP: KeyCode = KeyCode::KeyW;
pub const KEY_PLAYER_DOWN: KeyCode = KeyCode::KeyS;

use crate::map::*;
use crate::player::*;
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_player_move);
    }
}

pub fn check_player_move(
    mut q_actors: Query<(&mut MapPosition), With<Player>>,
    mut q_map: Query<&mut Map>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let mut map = q_map.single_mut();

    let (mut pos_player, _) = q_actors
        .iter_mut()
        .filter(|(_, a)| a.is_player())
        .last()
        .expect("no player pos found");

    let pos_player_old = pos_player.clone();

    if input.any_just_pressed(KEY_PLAYER_RIGHT)
        {
        move_right(&mut map, &mut pos_player).unwrap();
        }

    if input.any_just_pressed(KEY_PLAYER_LEFT)
        {
        move_left(&mut map, &mut pos_player).unwrap();
        }

    if input.any_just_pressed(KEY_PLAYER_UP)
        {
        move_up(&mut map, &mut pos_player).unwrap();
        }

    if input.any_just_pressed(KEY_PLAYER_DOWN)
        {
        move_down(&mut map, &mut pos_player).unwrap();
        }
}