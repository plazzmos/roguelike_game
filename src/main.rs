#![allow(warnings, unused)]

use rltk::{Rltk, GameState, Console, RGB, VirtualKeyCode};
use specs::prelude::*;

use std::cmp::{max, min};

#[macro_use]
extern crate specs_derive;

#[derive(Component)]
struct Position { x: i32, y: i32 }

#[derive(Component)]
struct Renderable { glyph: u8, fg: RGB, bg: RGB, }

#[derive(Component)]
struct LeftMover {}

#[derive(Component, Debug)]
struct Player {}

struct LeftWalker {}
impl<'s> System<'s> for LeftWalker {
    type SystemData = (
        ReadStorage<'s, LeftMover>,
        WriteStorage<'s, Position>
    );
    fn run(&mut self, (lefty, mut pos) : Self::SystemData) {
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 { pos.x = 79; }
        }
    }
}

struct State { ecs: specs::World }
impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalker{};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}
impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let position = self.ecs.read_storage::<Position>();
        let render = self.ecs.read_storage::<Renderable>();

        for (p, r) in (&position, &render).join() {
            ctx.set(p.x, p.y, r.fg, r.bg, r.glyph);
        }
        ctx.print(1, 1, "Tick!");
    }
}
fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut position = ecs.write_storage::<Position>();
    let mut player = ecs.write_storage::<Player>();

    for (_p, pos) in (&mut player, &mut position).join() {
        pos.x = min(79, max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
    }
}
fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {},
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
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
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();

    gs.ecs.create_entity()
            .with(Player{})
            .with(Position {x:40, y:25})
            .with(Renderable {
                glyph: rltk::to_cp437('@'),
                fg: RGB::named(rltk::GREEN),
                bg: RGB::named(rltk::BLACK),
                })
            .build();

    for i in 0..10 {
        gs.ecs.create_entity()
                .with(LeftMover{})
                .with(Position {x: i * 7, y:20})
                .with(Renderable {
                    glyph: rltk::to_cp437('&'),
                    fg: RGB::named(rltk::YELLOW),
                    bg: RGB::named(rltk::BLACK),
                    })
                .build();
    }
    rltk::main_loop(context, gs);
}
