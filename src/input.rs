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
            check_player_move_other.run_if(in_state(GameState::PlayerTurn)),
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

    if input.just_pressed(KEY_PLAYER_RIGHT) && can_move_right(&pos_player_old, &map)
        {
        println!("Right key pressed");
        move_right(&mut map, &mut pos_player).unwrap();
        }

    if input.just_pressed(KEY_PLAYER_LEFT) && can_move_left(&pos_player_old, &map)
        {
        println!("Left key pressed");
        move_left(&mut map, &mut pos_player).unwrap();
        }

    if input.just_pressed(KEY_PLAYER_UP) && can_move_up(&pos_player_old, &map)
        {
        println!("Up key pressed");
        move_up(&mut map, &mut pos_player).unwrap();
        }

    if input.just_pressed(KEY_PLAYER_DOWN) && can_move_down(&pos_player_old, &map)
        {
        println!("Down key pressed");
        move_down(&mut map, &mut pos_player).unwrap();
        }
    if pos_player_old != pos_player.clone() {
        println!("Going to Enemy Turn");
        next_game_state.set(GameState::EnemyTurn);
    }
}

//TODO Fix this shit it is broken
pub fn check_player_move_other(
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

    if input.just_pressed(KEY_PLAYER_RIGHT) && map.check_if_valid_move(&pos_player_old.right().unwrap())
        {
        map.move_unit(&mut pos_player_old, pos_player_old.right().unwrap());
        }
    if input.just_pressed(KEY_PLAYER_LEFT) && map.check_if_valid_move(&pos_player_old.left().unwrap())
        {
        map.move_unit(&mut pos_player_old, pos_player_old.left().unwrap());
        }
    if input.just_pressed(KEY_PLAYER_UP) && map.check_if_valid_move(&pos_player_old.up().unwrap())
        {
        map.move_unit(&mut pos_player_old, pos_player_old.up().unwrap());
        }
    if input.just_pressed(KEY_PLAYER_DOWN) && map.check_if_valid_move(&pos_player_old.down().unwrap())
        {
        map.move_unit(&mut pos_player_old, pos_player_old.down().unwrap());
        }
    if pos_player_old != pos_player.clone() {
        println!("Going to Enemy Turn");
        next_game_state.set(GameState::EnemyTurn);
    }
}