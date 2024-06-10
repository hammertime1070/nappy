use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Unit {
    pub unit_type: UnitType,
}

#[derive(Clone)]
pub enum UnitType {
    Player,
}