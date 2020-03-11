#![allow(warnings, unused)]
extern crate rltk;
use rltk::{Rltk, GameState, Console, RGB, Point};
extern crate specs;
use specs::prelude::*;

#[macro_use]
extern crate specs_derive;

mod components;
mod map;
mod player;
mod rect;
mod gamelog;

mod visibility_system;
mod monster_ai_system;
mod map_indexing_system;
mod melee_combat_system;
mod damage_system;
mod gui;

pub use components::*;
pub use map::*;
use player::*;
use gamelog::GameLog;

pub use rect::Rect;use visibility_system::VisibilitySystem;
use monster_ai_system::MonsterAI;
use map_indexing_system::MapIndexingSystem;
use melee_combat_system::MeleeCombatSystem;
use damage_system::DamageSystem;


#[derive(PartialEq, Copy, Clone)]
pub enum RunState { AwaitingInput, PreRun, PlayerTurn, MonsterTurn }

pub struct State { pub ecs: specs::World }

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAI{};
        mob.run_now(&self.ecs);
        let mut mapindex = MapIndexingSystem{};
        mapindex.run_now(&self.ecs);
        let mut melee = MeleeCombatSystem{};
        melee.run_now(&self.ecs);
        let mut damage = DamageSystem{};
        damage.run_now(&self.ecs);
        self.ecs.maintain();
    }
}
impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }
        match newrunstate {
            RunState::PreRun => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            },
            RunState::AwaitingInput => {
                newrunstate = player_input(self, ctx);
            },
            RunState::PlayerTurn => {
                self.run_systems();
                newrunstate = RunState::MonsterTurn;
            },
            RunState::MonsterTurn => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            },
        }
        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = newrunstate;
        }
        damage_system::delete_the_dead(&mut self.ecs);

        draw_map(&self.ecs, ctx);

        let position = self.ecs.read_storage::<Position>();
        let render = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (p, r) in (&position, &render).join() {
            let idx = map.xy_idx(p.x, p.y);
            if map.visible_tiles[idx] { ctx.set(p.x, p.y, r.fg, r.bg, r.glyph); }
        }
        gui::draw_ui(&self.ecs, ctx);
    }
}
fn main() {
    use rltk::RltkBuilder;
        let context = RltkBuilder::simple80x50()
            .with_title("Roguelike Game")
            .build();
//    context.with_post_scanlines(true);
    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<SufferDamage>();

    let map : Map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    let player_entity = gs.ecs
        .create_entity()
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
    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));
    gs.ecs.insert(RunState::PreRun);
    gs.ecs.insert(player_entity);
    gs.ecs.insert(gamelog::GameLog{ entries : vec!["Welcome to Rusty Roguelike".to_string()]});

    rltk::main_loop(context, gs);
}
