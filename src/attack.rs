use bevy::prelude::*;
use crate::resource::*;

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HitEvent>()
        add_systems(Update, handle_hits);
    }
}



#[derive(Event)]
struct HitEvent {
    pub attacker: Entity,
    pub target: Entity,
    pub damage: i32,
}

pub fn hit(attacker: Entity, target: Entity, damage: i32, mut hit_events: EventWriter<HitEvent>) {
    hit_events.send(HitEvent {
        attacker,
        target,
        damage,
    });
}

pub fn handle_hits(mut hit_events: EventReader<HitEvent>, mut query: Query<&mut HitPoints>) {
    for event in hit_events.iter() {
        if let Ok(mut hitpoints) = query.get_mut(event.target) {
            hitpoints.current -= event.damage;
            println!("Entity {:?} was hit by {:?} for {} damage.", event.target, event.attacker, event.damage);
        }
    }
}