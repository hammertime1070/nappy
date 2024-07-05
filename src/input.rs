pub const KEY_PLAYER_LEFT: KeyCode = KeyCode::KeyA;
pub const KEY_PLAYER_RIGHT: KeyCode = KeyCode::KeyD;
pub const KEY_PLAYER_UP: KeyCode = KeyCode::KeyW;
pub const KEY_PLAYER_DOWN: KeyCode = KeyCode::KeyS;

use crate::map::*;
use crate::player::*;
use crate::states::*;
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app .add_systems(
            Update,
            check_player_move.run_if(in_state(GameState::PlayerTurn)),
        );
    }
}

pub fn check_player_move(
    mut q_actors: Query<&mut MapPosition, With<Player>>,
    mut q_map: Query<&mut Map>,
    input: Res<ButtonInput<KeyCode>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let mut map = q_map.single_mut();

    let mut pos_player = q_actors
        .iter_mut()
        .last()
        .expect("no player pos found");

    let pos_player_old = pos_player.clone();

    if input.just_pressed(KEY_PLAYER_RIGHT)
        {
        println!("Right key pressed");
        move_right(&mut map, &mut pos_player).unwrap();
        }

    if input.just_pressed(KEY_PLAYER_LEFT)
        {
        println!("Left key pressed");
        move_left(&mut map, &mut pos_player).unwrap();
        }

    if input.just_pressed(KEY_PLAYER_UP)
        {
        println!("Up key pressed");
        move_up(&mut map, &mut pos_player).unwrap();
        }

    if input.just_pressed(KEY_PLAYER_DOWN)
        {
        println!("Down key pressed");
        move_down(&mut map, &mut pos_player).unwrap();
        }
    if pos_player_old != pos_player.clone() {
        next_game_state.set(GameState::EnemyTurn);
    }
}