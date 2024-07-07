use crate::map::*;
use crate::unit::*;
use crate::states::*;
use bevy::prelude::*;
use rand::Rng;
use crate::resource::HitPoints;
use crate::player::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::EnemyTurn), move_unit);
    }
}


#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct MovementStrategy {
    pub strategy: ConcreteMovementStrategy,
}

pub enum ConcreteMovementStrategy {
    MoveRandomly,
    MoveGreedily,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub movement_strategy:MovementStrategy,
    pub position: MapPosition,
    pub sprite: SpriteSheetBundle,
    pub unit: Unit,
}

pub fn move_unit(
    mut q_units: Query<(&mut MapPosition, &MovementStrategy), Without<Player>>,
    mut q_player_pos: Query<(&MapPosition), With<Player>>,
    mut q_map: Query<&mut Map>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let mut map = q_map.single_mut();
    let player_pos = q_player_pos.single();
    for (mut unit_pos, movement_strategy) in q_units.iter_mut() {
        match movement_strategy.strategy {
            ConcreteMovementStrategy::MoveRandomly => {
                move_randomly(&mut unit_pos, &mut map);
            },
            ConcreteMovementStrategy::MoveGreedily => {
                move_greedily(&mut unit_pos, &mut map, &player_pos);
            }
            // Add more cases here as you add more strategies
        }
    }
    println!("Going to Player Turn");
    next_game_state.set(GameState::PlayerTurn);
}

pub fn move_randomly(mut pos_unit: &mut MapPosition, map: &mut Map) {
    println!("attempting to move randomly");
    let pos_reachable = enumerate_reachable_positions(&pos_unit.clone(), &map);
    if !pos_reachable.is_empty() {
        let pos_random = 
            pos_reachable[rand::thread_rng().gen_range(0..pos_reachable.len())];
        map.move_unit(&mut pos_unit, &pos_random).unwrap();
    }
}

pub fn move_greedily(mut pos_unit: &mut MapPosition, map: &mut Map, pos_player: &MapPosition) {
    println!("attempting to move greedily");
    let mut pos_reachable = enumerate_reachable_positions(&pos_unit.clone(), &map);
    // Sorting reachable positions by distance to player
    pos_reachable.sort_by_key(|pos| pos.distance(pos_player));
    // Move towards player
    if let Some(new_pos) = pos_reachable.first() {
        map.move_unit(&mut pos_unit, new_pos).unwrap();
    }
}