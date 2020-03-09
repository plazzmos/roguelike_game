extern crate specs;
use specs::prelude::*;
use super::{Viewshed, Position, Map, Player};
extern crate rltk;
use rltk::{field_of_view, Point};

pub struct VisibilitySystem {}

impl<'s> System<'s> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'s, Map>,
        Entities<'s>,
        WriteStorage<'s, Viewshed>,
        WriteStorage<'s, Position>,
        ReadStorage<'s, Player>
    );
    fn run(&mut self, data : Self::SystemData) {
        let (mut map, entities, mut viewshed, position, player) = data;

        for (ent, vs, pos) in (&entities, &mut viewshed, &position).join() {
            vs.visible_tiles.clear();
            vs.visible_tiles = field_of_view(
                Point::new(pos.x, pos.y),
                vs.range,
                &*map,
            );
            vs.visible_tiles.retain(|p|
                p.x > 0 && p.x < map.width-1 &&
                p.y > 0 && p.y < map.height-1
            );
            let p : Option<&Player> = player.get(ent);
            if let Some(p) = p {
                for vis in vs.visible_tiles.iter() {
                    let idx = map.xy_idx(vis.x, vis.y);
                    map.revealed_tiles[idx] = true;
                }
            }
        }
    }
}
