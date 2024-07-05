use crate::map::*;
use crate::unit::*;
use crate::states::*;
use bevy::prelude::*;
use rand::Rng;
use crate::resource::HitPoints;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::EnemyTurn), move_unit);
    }
}


#[derive(Component)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub position: MapPosition,
    pub sprite: SpriteSheetBundle,
    pub unit: Unit,
}

pub fn move_unit(
    mut q_units: Query<(&mut MapPosition, &Unit)>,
    mut q_map: Query<&mut Map>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let mut map = q_map.single_mut();
    for (mut unit_pos, unit) in q_units.iter_mut() {
        move_randomly(&mut unit_pos, &mut map);
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