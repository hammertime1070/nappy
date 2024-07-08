use crate::map::MapPosition;
use bevy::prelude::*;
use crate::resource::HitPoints;
use crate::unit::*;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub position: MapPosition,
    pub sprite: SpriteSheetBundle,
    pub unit: Unit,
}