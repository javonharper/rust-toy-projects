use bevy::prelude::*;

#[derive(Component)]
pub struct Character;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Match {
    pub characters: Vec<Entity>,
    pub is_active: bool,
}

#[derive(Component)]
pub struct HitPoints {
    pub current: i32,
}

#[derive(Component)]
pub struct EquippedArmor {
    pub armor: Entity,
}

#[derive(Component)]
pub struct Armor {
    pub damage_reduction: i32,
}

#[derive(Component)]
pub struct Shield {}

#[derive(Component)]
pub struct EquippedWeapon {
    pub weapon: Entity,
}

#[derive(Component)]
pub struct Weapon {
    pub damage: String,
}


#[derive(Component)]
pub struct CalculatedAttributes {
    // pub speed: f32,
    pub dodge: i32,
}
