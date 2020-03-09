#![allow(warnings, unused)]

use rltk::{Rltk, GameState, Console, RGB, VirtualKeyCode};
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


pub struct State { ecs: specs::World }
impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}
impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        draw_map(&self.ecs, ctx);
        let position = self.ecs.read_storage::<Position>();
        let render = self.ecs.read_storage::<Renderable>();

        for (p, r) in (&position, &render).join() {
            ctx.set(p.x, p.y, r.fg, r.bg, r.glyph);
        }
    }
}
fn main() {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Game")
        .build();
    let mut gs = State { ecs: specs::World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    let map : Map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs.insert(map);

    gs.ecs.create_entity()
            .with(Player{})
            .with(Position {x:player_x, y:player_y})
            .with(Renderable {
                glyph: rltk::to_cp437('@'),
                fg: RGB::named(rltk::GREEN),
                bg: RGB::named(rltk::BLACK),
                })
            .with(Viewshed{ visible_tiles: Vec::new(), range : 8, dirty: true})
            .build();

    rltk::main_loop(context, gs);
}
