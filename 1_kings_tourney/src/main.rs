mod components;
mod events;
mod resources;
mod roll;
mod settings;

use bevy::prelude::*;
use components::*;
use events::*;
use rand::Rng;
use resources::*;
use settings::DEBUG_DAMAGE;

enum AttackDefense {
    Dodge,
    Block,
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_systems(Update, combat_system)
        .add_plugins(InitializePlugin)
        .observe(on_create_matches)
        .observe(on_start_matches)
        .observe(on_attack_intent)
        .observe(on_take_damage)
        .observe(on_knock_out)
        .observe(on_end_duel)
        .run();
}

// Event Handlers ==================================================================================

fn on_create_matches(
    _trigtger: Trigger<CreateMatches>,
    characters: Query<(Entity, &components::Name, &Character)>,
    mut commands: Commands,
) {
    let mut characters_copy = characters.iter().collect::<Vec<_>>();
    let mut rng = rand::thread_rng();

    // Pluck two random characters from the list
    let index1 = rng.gen_range(0..characters_copy.len());
    let character1 = characters_copy.remove(index1);

    let index2 = rng.gen_range(0..characters_copy.len());
    let character2 = characters_copy.remove(index2);

    let (char_ent1, name1, _character1) = character1;
    let (char_ent2, name2, _character2) = character2;

    println!(
        "{} and {} are matched against each other. May the best fighter win!",
        name1.0, name2.0
    );

    let match_name = format!("Duel between {} and {}", name1.0, name2.0);

    commands.spawn((
        components::Match {
            characters: vec![char_ent1, char_ent2],
            is_active: false,
        },
        components::Name(match_name.to_string()),
    ));

    println!("");
    commands.trigger(StartMatches);
}

fn on_start_matches(
    _trigger: Trigger<StartMatches>,
    mut query: Query<(&mut Match, &components::Name)>,
) {
    for (duel, name) in query.iter_mut() {
        start_duel(duel, name);

        break;
    }
}

fn start_duel(mut duel: Mut<Match>, _name: &components::Name) {
    duel.is_active = true;
}

fn on_attack_intent(
    trigger: Trigger<AttackIntent>,
    mut commands: Commands,
    query: Query<(&components::Name, &HitPoints, &CalculatedAttributes)>,
    equpped_armour_query: Query<&EquippedArmor>,
    armor_query: Query<&Armor>,
    shield_query: Query<&Armor, With<Shield>>,
    equipped_weapon_query: Query<&EquippedWeapon>,
    weapon_query: Query<(&Weapon, &components::Name)>,
) {
    let event = trigger.event();

    let (attacker_name, attacker_hp, _) = query.get(event.attacker).unwrap();
    let (target_name, _, c_attributes) = query.get(event.target).unwrap();
    let equipped_weapon = equipped_weapon_query.get(event.attacker).unwrap();
    let weapon = weapon_query.get(equipped_weapon.weapon).unwrap();

    if attacker_hp.current <= 0 {
        return;
    }

    let mut hit_lands = true;
    let mut hit_blocked = false;

    let attack_score = 12;
    let block_score = 12;

    if determine_if_hit(attack_score) {
        // Randomly decide to dodge or block, pick from the enum
        let attack_defense = match rand::thread_rng().gen_range(0..2) {
            0 => AttackDefense::Dodge,
            1 => AttackDefense::Block,
            _ => panic!("AttackDefense enum out of bounds"),
        };

        match attack_defense {
            AttackDefense::Dodge => {
                if determine_if_dodge(c_attributes.dodge) {
                    println!(
                        "{} strikes {} with their {}, but they dodge!",
                        attacker_name.0, target_name.0, weapon.1 .0
                    );
                    hit_lands = false;
                }
            }
            AttackDefense::Block => {
                if determine_if_block(block_score) {
                    println!(
                        "{} strikes {} with their {}, and it's blocked!",
                        attacker_name.0, target_name.0, weapon.1 .0
                    );
                    hit_lands = true;
                    hit_blocked = true;
                }
            }
        }
    } else {
        println!("{} misses {}!", attacker_name.0, target_name.0);
        hit_lands = false;
    }

    if hit_lands {
        println!(
            "{} strikes {} with their {}!",
            attacker_name.0, target_name.0, weapon.1 .0
        );

        let damage = calculate_damage(
            event.attacker,
            event.target,
            equpped_armour_query,
            armor_query,
            shield_query,
            equipped_weapon_query,
            weapon_query,
            hit_blocked,
        );

        if DEBUG_DAMAGE {
            println!(
                "{} hits {} for {} damage!",
                attacker_name.0, target_name.0, damage
            );
        }

        commands.trigger(TakeDamage {
            target: event.target,
            damage,
        });
    }
}

fn determine_if_hit(attack_score: i32) -> bool {
    return roll::roll("3d6", "attack_roll") <= attack_score;
}

fn determine_if_dodge(dodge_score: i32) -> bool {
    return roll::roll("3d6", "dodge_roll") <= dodge_score;
}

fn determine_if_block(block_score: i32) -> bool {
    return roll::roll("3d6", "block_roll") <= block_score;
}

fn calculate_damage(
    _attacker: Entity,
    target: Entity,
    equpped_armour_query: Query<&EquippedArmor>,
    armor_query: Query<&Armor>,
    shield_query: Query<&Armor, With<Shield>>,
    equipped_weapon_query: Query<&EquippedWeapon>,
    weapon_query: Query<(&Weapon, &components::Name)>,
    hit_blocked: bool,
) -> i32 {
    let equipped_armor = equpped_armour_query.get(target).unwrap();
    let armor = armor_query.get(equipped_armor.armor).unwrap();
    let shield = shield_query.get(target).unwrap_or(&Armor {
        damage_reduction: 0,
    });

    let equipped_weapon = equipped_weapon_query.get(target).unwrap();
    let weapon = weapon_query.get(equipped_weapon.weapon).unwrap();

    let mut damage_reduction = armor.damage_reduction;
    if hit_blocked {
        damage_reduction += shield.damage_reduction;
    }

    let damage_roll_result = roll::roll(weapon.0.damage.as_str(), "damage_roll");

    let damage = damage_roll_result - damage_reduction;

    if settings::DEBUG_DAMAGE {
        println!(
            "Damage: {} - {} = {}",
            damage_roll_result, damage_reduction, damage
        );
    }

    damage
}

fn on_take_damage(
    trigger: Trigger<TakeDamage>,
    mut commands: Commands,
    mut query: Query<(&components::Name, &mut HitPoints)>,
) {
    let event = trigger.event();
    let target = event.target;
    let damage = event.damage;

    let (name, mut hit_points) = query.get_mut(target).unwrap();

    hit_points.current -= damage;
    hit_points.current = hit_points.current.max(0);

    if DEBUG_DAMAGE {
        println!(
            "{} takes {} damage. {} HP remaining.\n",
            name.0, damage, hit_points.current
        );
    }

    if hit_points.current <= 0 {
        commands.trigger(KnockOut { target })
    }
}

fn on_knock_out(trigger: Trigger<KnockOut>, query: Query<&components::Name>) {
    let event = trigger.event();
    let target = event.target;

    let name = query.get(target).unwrap();

    println!("");
    println!("{} has been knocked unconcious", name.0);
}

fn on_end_duel(
    trigger: Trigger<EndDuel>,
    query: Query<&components::Name>,
    mut duel_query: Query<&mut Match>,
) {
    let event = trigger.event();

    let mut duel = duel_query.get_mut(event.duel_ent).unwrap();
    let winner = query.get(event.winner).unwrap();
    let loser = query.get(event.loser).unwrap();

    println!("{} wins the duel against {}!\n", winner.0, loser.0);

    duel.is_active = false;
}

fn spawn_characters(mut commands: Commands) {
    let dexterity = 10;
    let health = 10;

    let speed = (dexterity + health) as f32 / 4.;
    let dodge = (speed + 3.).floor() as i32;

    commands.spawn((
        components::Name("Red".to_string()),
        Character,
        HitPoints { current: 10 },
        Attributes {},
        CalculatedAttributes { dodge },
    ));

    commands.spawn((
        components::Name("Green".to_string()),
        Character,
        HitPoints { current: 10 },
        CalculatedAttributes { dodge },
    ));

    commands.spawn((
        components::Name("Blue".to_string()),
        Character,
        HitPoints { current: 10 },
        CalculatedAttributes { dodge },
    ));

    commands.spawn((
        components::Name("Yellow".to_string()),
        Character,
        HitPoints { current: 10 },
        CalculatedAttributes { dodge },
    ));
}

fn spawn_equpment(mut commands: Commands) {
    commands.spawn((
        components::Name("Leather Armor".to_string()),
        Armor {
            damage_reduction: 2,
        },
    ));

    commands.spawn((
        components::Name("Long Sword".to_string()),
        Weapon {
            damage: "1d6+2".to_string(),
        },
    ));

    commands.spawn((
        components::Name("Wooden Buckler".to_string()),
        Armor {
            damage_reduction: 1,
        },
    ));
}

fn equip_characters(
    mut commands: Commands,
    armour_query: Query<(Entity, &components::Name), With<Armor>>,
    weapon_query: Query<(Entity, &components::Name), With<Weapon>>,
    character_query: Query<(Entity, &components::Name), With<Character>>,
) {
    let leather_armor = armour_query
        .iter()
        .find(|(_, name)| name.0 == "Leather Armor");

    let short_sword = weapon_query.iter().find(|(_, name)| name.0 == "Long Sword");

    let buckler = armour_query
        .iter()
        .find(|(_, name)| name.0 == "Wooden Buckler");

    for (character_entity, _) in character_query.iter() {
        let (armour_entity, _) = leather_armor.unwrap();
        let (weapon_entity, _) = short_sword.unwrap();
        let (buckler_entity, _) = buckler.unwrap();

        commands.entity(character_entity).insert(EquippedWeapon {
            weapon: weapon_entity,
        });

        commands.entity(character_entity).insert(EquippedArmor {
            armor: armour_entity,
        });

        commands.entity(character_entity).insert(EquippedArmor {
            armor: buckler_entity,
        });
    }

    println!("");
}

fn introduce_characters(
    time: Res<Time>,
    mut timer: ResMut<IntroduceTimer>,
    mut commands: Commands,
    query: Query<&components::Name, With<Character>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("{} enters the tournament.", name.0);
        }

        println!("");

        commands.trigger(CreateMatches);
    }
}

fn combat_system(
    mut commands: Commands,
    query: Query<(Entity, &Match)>,
    health_query: Query<&HitPoints>,
) {
    for (duel_ent, duel) in &query {
        if duel.is_active {
            let (ent1, ent2) = (duel.characters[0], duel.characters[1]);

            let ent1_hp = health_query.get(ent1).unwrap();
            let ent2_hp = health_query.get(ent2).unwrap();

            let has_unconscious = ent1_hp.current <= 0 || ent2_hp.current <= 0;

            // if either of the combatants are knocked out, end the duel
            if has_unconscious {
                let winner = if ent1_hp.current > 0 { ent1 } else { ent2 };
                let loser = if ent1_hp.current <= 0 { ent1 } else { ent2 };
                commands.trigger(EndDuel {
                    duel_ent,
                    winner,
                    loser,
                });

                break;
            }

            commands.trigger(AttackIntent {
                attacker: ent1,
                target: ent2,
            });

            commands.trigger(AttackIntent {
                attacker: ent2,
                target: ent1,
            });
        }
    }
}

pub struct InitializePlugin;

impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(IntroduceTimer(Timer::from_seconds(0.0, TimerMode::Once)));
        app.add_systems(
            Startup,
            (spawn_equpment, spawn_characters, equip_characters).chain(),
        );
        app.add_systems(Update, (introduce_characters).chain());
    }
}
