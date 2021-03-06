extern crate rltk;
use rltk::{ RGB, RandomNumberGenerator };
extern crate specs;
use specs::prelude::*;
use super::{
    components::*,
    random_table,
    map::MAPWIDTH, map::MAPHEIGHT, map::MAPCOUNT,
    rect::Rect,
};
use crate::specs::saveload::{MarkedBuilder, SimpleMarker};

const MAX_MONSTERS : i32 = 4;
const MAX_ITEMS : i32 = 2;
pub fn player(ecs: &mut World, player_x: i32, player_y: i32) -> Entity {
    ecs
        .create_entity()
        .with(Player{})
        .with(Position{ x:player_x, y:player_y })
        .with(Renderable{
            glyph : rltk::to_cp437('@'),
            fg: RGB::named(rltk::GREEN),
            bg: RGB::named(rltk::BLACK),
            render_order: 0,
        })
        .with(Viewshed{ visible_tiles : Vec::new(), range: 8, dirty: true })
        .with(Name{ name: "Player".to_string() })
        .with(CombatStats{ max_hp: 50, hp: 50, defense: 2, power: 6 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}
fn monster<S :ToString>(ecs: &mut World, x: i32, y: i32, glyph : u8, name : S) {
    ecs
        .create_entity()
        .with(Monster{})
        .with(Position{ x,y })
        .with(Renderable{
            glyph,
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
            render_order : 1,
        })
        .with(Viewshed{ visible_tiles : Vec::new(), range: 8, dirty: true })
        .with(Name{ name : name.to_string() })
        .with(BlocksTile{})
        .with(CombatStats{ max_hp: 15, hp: 15, defense: 1, power: 4 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}
fn orc(ecs: &mut World, x:i32, y:i32) { monster(ecs, x, y, rltk::to_cp437('o'), "Orc"); }
fn goblin(ecs: &mut World, x:i32, y:i32) { monster(ecs, x, y, rltk::to_cp437('g'), "Goblin"); }

pub fn random_monster(ecs: &mut World, x: i32, y: i32) {
    let roll: i32;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        roll = rng.roll_dice(1, 2);
    }
    match roll {
        1 => { orc(ecs, x, y) },
        _ => { goblin(ecs, x, y) },
    }
}
fn health_potion(ecs: &mut World, x: i32, y: i32) {
    ecs
        .create_entity()
        .with(Item{})
        .with(Potion{ heal : 25 })
        .with(Name{ name : "Red Sugar".to_string() })
        .with(Position{ x,y })
        .with(Renderable{
            glyph: rltk::to_cp437('i'),
            fg: RGB::named(rltk::MAGENTA),
            bg: RGB::named(rltk::BLACK),
            render_order : 2,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}
fn magic_missile_scroll(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{ range:10 })
        .with(InflictsDamage{ damage:8 })
        .with(Position{ x,y })
        .with(Name{ name:"Magic Missile Scroll".to_string() })
        .with(Renderable{
            glyph: rltk::to_cp437( '&' ),
            fg: RGB::named(rltk::CYAN),
            bg: RGB::named(rltk::BLACK),
            render_order : 2,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}
fn fireball_scroll(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{ range:10 })
        .with(InflictsDamage{ damage:20 })
        .with(AreaOfEffect{ radius:3 })
        .with(Position{ x,y })
        .with(Name{ name:"Fireball Scroll".to_string() })
        .with(Renderable{
            glyph: rltk::to_cp437( '&' ),
            fg: RGB::named(rltk::ORANGE),
            bg: RGB::named(rltk::BLACK),
            render_order : 2,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}
fn confusion_scroll(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{ range:10 })
        .with(Confusion{ turns:4 })
        .with(Position{ x,y })
        .with(Name{ name:"Confusion Scroll".to_string() })
        .with(Renderable{
            glyph: rltk::to_cp437( '&' ),
            fg: RGB::named(rltk::PINK),
            bg: RGB::named(rltk::BLACK),
            render_order : 2,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}
pub fn random_item(ecs: &mut World, x: i32, y: i32) {
    let roll: i32;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        roll = rng.roll_dice(1, 4);
    }
    match roll {
        1 => { health_potion(ecs, x, y) },
        2 => { fireball_scroll(ecs, x, y) },
        3 => { confusion_scroll(ecs, x, y) },
        _ => { magic_missile_scroll(ecs, x, y) },
    }
}
fn room_table() -> random_table::RandomTable {
    random_table::RandomTable::new()
        .add("Goblin", 10)
        .add("Orc", 1)
        .add("Health Potion", 7)
        .add("Fireball Scroll", 2)
        .add("Confusion Scroll", 2)
        .add("Magic Missile Scroll", 4)
}
pub fn spawn_room(ecs: &mut World, room : &Rect) {
    let mut monster_spawn_points : Vec<usize> = Vec::new();
    let mut item_spawn_points : Vec<usize> = Vec::new();
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        let num_monsters = rng.roll_dice(1, MAX_MONSTERS + 2) - 3;
        let num_items = rng.roll_dice(1, MAX_ITEMS + 2) - 3;

        for _i in 0..num_monsters {
            let mut added = false;
            while !added {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1)));
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1)));
                let idx = (y as usize * MAPWIDTH) + x as usize;
                if !monster_spawn_points.contains(&idx) {
                    monster_spawn_points.push(idx);
                    added = true;
                }
            }
        }
        for _i in 0..num_items {
            let mut added = false;
            while !added {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                let idx = (y * MAPWIDTH) + x;
                if !item_spawn_points.contains(&idx) {
                    item_spawn_points.push(idx);
                    added = true;
                }
            }
        }
    }
    for idx in monster_spawn_points.iter() {
        let x = *idx % MAPWIDTH;
        let y = *idx / MAPWIDTH;
        random_monster(ecs, x as i32, y as i32);
    }
    for idx in item_spawn_points.iter() {
        let x = *idx % MAPWIDTH;
        let y = *idx / MAPWIDTH;
        random_item(ecs, x as i32, y as i32);
    }
}




