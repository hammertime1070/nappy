use bevy::prelude::*;

#[derive(Component)]
pub struct Unit {
    pub unit_type: UnitType,
}

pub enum UnitType {
    Player,
}