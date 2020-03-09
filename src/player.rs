#![allow(warnings, unused)]
extern crate rltk;
use rltk::{Rltk, VirtualKeyCode};
extern crate specs;
use specs::prelude::*;
use std::cmp::{max, min};
use super::{Position, Player, Viewshed, TileType, State, Map};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut position = ecs.write_storage::<Position>();
    let mut player = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();

    for (_p, pos, vs) in (&mut player, &mut position, &mut viewsheds).join() {
        let dest_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map.tiles[dest_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
            vs.dirty = true;
        }
    }
}
pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {},
        Some(key) => match key {
            VirtualKeyCode::A => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::D => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::W => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::S => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        }
    }
}
