#![allow(warnings, unused)]
extern crate specs;
use specs::prelude::*;
extern crate specs_derive;
extern crate rltk;
use rltk::{RGB};

#[derive(Component)]
pub struct Position { pub x: i32, pub y: i32 }

#[derive(Component)]
pub struct Renderable { pub glyph: u8, pub fg: RGB, pub bg: RGB, }

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component, Debug)]
pub struct Monster {}

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles : Vec<rltk::Point>,
    pub range : i32,
    pub dirty : bool,
}
#[derive(Component, Debug)]
pub struct Name { pub name : String }

#[derive(Component, Debug)]
pub struct BlocksTile {}

#[derive(Component, Debug)]
pub struct CombatStats {
    pub max_hp : i32,
    pub hp : i32,
    pub defense : i32,
    pub power : i32,
}
