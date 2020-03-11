#![allow(warnings, unused)]
extern crate rltk;
use rltk::{Rltk, VirtualKeyCode, Point, console};
extern crate specs;
use specs::prelude::*;
use std::cmp::{max, min};
use super::{Position, Player, Viewshed, TileType, State, Map, RunState, CombatStats};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut position = ecs.write_storage::<Position>();
    let mut player = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let map = ecs.fetch::<Map>();

    for (_p, pos, vs) in (&mut player, &mut position, &mut viewsheds).join() {
        let dest_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

        for potential_target in map.tile_content[dest_idx].iter() {
            let target = combat_stats.get(*potential_target);
            match target {
                None => {},
                Some(t) => {
                    // attack target ...
                    console::log(&format!("From Hell's Heart, I stab thee!"));
                    return;
                }
            }
        }


        if !map.blocked[dest_idx] {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
            vs.dirty = true;
            let mut ppos = ecs.write_resource::<Point>();
            ppos.x = pos.x;
            ppos.y = pos.y;
        }
    }
}
pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => { return RunState::Paused },
        Some(key) => match key {
            VirtualKeyCode::A => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::D => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::W => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::S => try_move_player(0, 1, &mut gs.ecs),

            VirtualKeyCode::E => try_move_player(1, -1, &mut gs.ecs),
            VirtualKeyCode::Q => try_move_player(-1, -1, &mut gs.ecs),
            VirtualKeyCode::C => try_move_player(1, 1, &mut gs.ecs),
            VirtualKeyCode::X => try_move_player(-1, 1, &mut gs.ecs),
            _ => { return RunState::Paused }
        }
    }
    RunState::Running
}
