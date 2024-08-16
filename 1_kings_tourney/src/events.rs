use bevy::prelude::*;

#[derive(Event)]
pub struct CreateMatches;

#[derive(Event)]
pub struct StartMatches;

#[derive(Event)]
pub struct AttackIntent {
    pub attacker: Entity,
    pub target: Entity,
}

#[derive(Event)]
pub struct TakeDamage {
    pub target: Entity,
    pub damage: i32,
}

#[derive(Event)]
pub struct KnockOut {
    pub target: Entity,
}

#[derive(Event)]
pub struct EndDuel {
    pub duel_ent: Entity,
    pub winner: Entity,
    pub loser: Entity,
}
