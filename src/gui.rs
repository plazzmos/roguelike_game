extern crate rltk;
use rltk::{ RGB, Rltk, Console, Point };
extern crate specs;
use specs::prelude::*;
use super::{CombatStats, Player, Name, Position, Map, GameLog};

fn draw_tooltips(ecs: &World, ctx : &mut Rltk) {
    let map = ecs.fetch::<Map>();
    let names = ecs.read_storage::<Name>();
    let positions = ecs.read_storage::<Position>();

    let mouse_pos = ctx.mouse_pos();
    if mouse_pos.0 >= map.width || mouse_pos.1 >= map.height { return; }
    let mut tooltip : Vec<String> = Vec::new();
    for (n, pos) in (&names, &positions).join() {
        if pos.x == mouse_pos.0 && pos.y == mouse_pos.1 {
            tooltip.push(n.name.to_string());
        }
    }
    if !tooltip.is_empty() {
        let mut width: i32 = 0;
        for s in tooltip.iter() {
            if width < s.len() as i32 { width = s.len() as i32; }
        }
        width += 3;

        if mouse_pos.0 > 40 {
            let arrow_pos = Point::new(mouse_pos.0 - 2, mouse_pos.1);
            let left_x = mouse_pos.0 - width;
            let mut y = mouse_pos.1;
            for s in tooltip.iter() {
                ctx.print_color(
                    left_x, y,
                    RGB::named(rltk::WHITE),
                    RGB::from_f32(0.2, 0.2, 0.2),
                    s,
                );
                let padding = (width - s.len() as i32)-1;
                for i in 0..padding {
                    ctx.print_color(
                        arrow_pos.x - i, y,
                        RGB::named(rltk::WHITE),
                        RGB::from_f32(0.2, 0.2, 0.2),
                        &" ".to_string(),
                    );
                }
                y += 1;
            }
            ctx.print_color(
                arrow_pos.x, arrow_pos.y,
                RGB::named(rltk::WHITE),
                RGB::from_f32(0.2, 0.2, 0.2),
                &"->".to_string(),
            );
        } else {
            let arrow_pos = Point::new(mouse_pos.0 + 1, mouse_pos.1);
            let left_x = mouse_pos.0 +3;
            let mut y = mouse_pos.1;
            for s in tooltip.iter() {
                ctx.print_color(
                    left_x+1, y,
                    RGB::named(rltk::WHITE),
                    RGB::from_f32(0.2, 0.2, 0.2),
                    s,
                );
                let padding = (width - s.len() as i32)-1;
                for i in 0..padding {
                    ctx.print_color(
                        arrow_pos.x + 1 + i, y,
                        RGB::named(rltk::WHITE),
                        RGB::from_f32(0.2, 0.2, 0.2),
                        &" ".to_string(),
                    );
                }
                y += 1;
            }
            ctx.print_color(
                arrow_pos.x, arrow_pos.y,
                RGB::named(rltk::WHITE),
                RGB::from_f32(0.2, 0.2, 0.2),
                &"<-".to_string(),
            );
        }
    }
}

pub fn draw_ui(ecs: &World, ctx : &mut Rltk) {
    ctx.draw_box(
        0, 43, 79, 6,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK)
    );
    let combat_stats = ecs.read_storage::<CombatStats>();
    let players = ecs.read_storage::<Player>();
    let log = ecs.fetch::<GameLog>();

    let mut y = 45;
    for s in log.entries.iter().rev() {
        if y < 49 { ctx.print(2, y, s); }
        y += 1;
    }

    for (_player, stats) in (&players, &combat_stats).join() {
        let health = format!(" HP: {} / {}", stats.hp, stats.max_hp);
        ctx.print_color(
            12, 43,
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
            &health
        );
        ctx.draw_bar_horizontal(
            28, 43, 51,
            stats.hp,
            stats.max_hp,
            RGB::named(rltk::RED),
            RGB::named(rltk::BLACK),
        );
        let mouse_pos = ctx.mouse_pos();
        ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(rltk::MAGENTA));
    }
    draw_tooltips(ecs, ctx);
}
