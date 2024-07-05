use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct Unit {
    pub unit_type: UnitType,
}

#[derive(Clone, Copy)]
pub enum UnitType {
    Player,
    Enemy,
}