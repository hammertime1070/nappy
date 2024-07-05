use bevy::prelude::*;

#[derive(Component)]
pub struct HitPoints {
    max: usize,
    current: usize,
}