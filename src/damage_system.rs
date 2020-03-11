extern crate specs;
use specs::prelude::*;
use super::{CombatStats, SufferDamage, Player, Name, gamelog::GameLog};
use rltk::console;

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
    );
    fn run(&mut self, data : Self::SystemData) {
        let (mut combat_stats, mut damage) = data;

        for (mut cs, d) in (&mut combat_stats, &damage).join() {
            cs.hp -= d.amount.iter().sum::<i32>();
        }
        damage.clear();
    }
}
pub fn delete_the_dead(ecs : &mut World) {
    let mut dead : Vec<Entity> = Vec::new();
    {
        let combat_stats = ecs.read_storage::<CombatStats>();
        let players = ecs.read_storage::<Player>();
        let names = ecs.read_storage::<Name>();
        let entities = ecs.entities();
        let mut log = ecs.write_resource::<GameLog>();
        for (entity, stats) in (&entities, &combat_stats).join() {
            if stats.hp < 1 {
                let player = players.get(entity);
                match player {
                    None => {
                        let victim_name = names.get(entity);
                        if let Some(victim_name) = victim_name {
                            log.entries.push(format!("{} became tame!", &victim_name.name));
                        }
                        dead.push(entity);
                    }
                    Some(_) => console::log("You are dead")
                }
            }
        }
    }
    for victim in dead {
        ecs.delete_entity(victim).expect("Unable to delete");
    }
}
