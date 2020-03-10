#![allow(warnings, unused)]
extern crate rltk;
use rltk::{Rltk, GameState, Console, RGB, Point};
extern crate specs;
use specs::prelude::*;

#[macro_use]
extern crate specs_derive;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
use player::*;
mod rect;
pub use rect::Rect;
mod visibility_system;
use visibility_system::VisibilitySystem;
mod monster_ai_system;
use monster_ai_system::MonsterAI;
mod map_indexing_system;
use map_indexing_system::MapIndexingSystem;


pub struct State { pub ecs: specs::World, pub runstate : RunState }

#[derive(PartialEq, Copy, Clone)]
pub enum RunState { Paused, Running }

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAI{};
        mob.run_now(&self.ecs);
        let mut mapindex = MapIndexingSystem{};
        mapindex.run_now(&self.ecs);
        self.ecs.maintain();
    }
}
impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();

        if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Paused;
        } else {
            self.runstate = player_input(self, ctx);
        }

        draw_map(&self.ecs, ctx);
        let position = self.ecs.read_storage::<Position>();
        let render = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (p, r) in (&position, &render).join() {
            let idx = map.xy_idx(p.x, p.y);
            if map.visible_tiles[idx] { ctx.set(p.x, p.y, r.fg, r.bg, r.glyph); }
        }
    }
}
fn main() {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Game")
        .build();
    let mut gs = State {
        ecs: World::new(),
        runstate : RunState::Running,
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();

    let map : Map = Map::new_map_rooms_and_corridors();
    let mut rng = rltk::RandomNumberGenerator::new();
    for (i,room) in map.rooms.iter().skip(1).enumerate() {
        let (x,y) = room.center();
        let glyph : u8;
        let name : String;
        let roll = rng.roll_dice(1, 2);
        match roll {
            1 => { glyph = rltk::to_cp437('g'); name = "Goblin".to_string(); },
            _ => { glyph = rltk::to_cp437('o'); name = "Orc".to_string(); },
        }
        gs.ecs.create_entity()
            .with(Monster{})
            .with(Name{ name: format!("{} #{}", &name, i) })
            .with(Position{ x, y })
            .with(CombatStats{ max_hp:16, hp:16, defense:1, power:4 })
            .with(BlocksTile{})
            .with(Renderable{
                glyph : glyph,
                fg : RGB::named(rltk::RED),
                bg : RGB::named(rltk::BLACK),
            })
            .with(Viewshed{ visible_tiles : Vec::new(), range: 8, dirty: true })
            .build();
    }
    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));

    gs.ecs.create_entity()
            .with(Player{})
            .with(Name{ name: "Player".to_string() })
            .with(Position { x:player_x, y:player_y })
            .with(CombatStats{ max_hp:30, hp:30, defense:2, power:5 })
            .with(Renderable {
                glyph: rltk::to_cp437('@'),
                fg: RGB::named(rltk::GREEN),
                bg: RGB::named(rltk::BLACK),
                })
            .with(Viewshed{ visible_tiles: Vec::new(), range : 8, dirty: true})
            .build();

    rltk::main_loop(context, gs);
}
