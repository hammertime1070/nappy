use crate::map::MapPosition;
use bevy::prelude::*;
use crate::resource::HitPoints;


#[derive(Component)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub position: MapPosition,
    pub sprite: SpriteSheetBundle,
}