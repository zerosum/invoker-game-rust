extern crate invoker_game_core as core;
#[macro_use]
extern crate lazy_static;
extern crate serde;
extern crate wasm_bindgen;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use core::game::*;

mod ctx {
    use std::sync::RwLock;

    use core::game::*;

    lazy_static! {
        pub static ref PLAYER: RwLock<Invoker> = {
            let p = Invoker::new();
            RwLock::new(p)
        };
    }
}

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub e1: char,
    pub e2: char,
    pub e3: char,
    pub s1: &'static str,
    pub s2: &'static str,
}

#[wasm_bindgen]
pub fn fetch_status() -> JsValue {
    let p = ctx::PLAYER.read().unwrap();

    let _player = Player {
        e1: display_element(p.elements[0]),
        e2: display_element(p.elements[1]),
        e3: display_element(p.elements[2]),
        s1: p.spell1.map_or_else(|| "", display_spell),
        s2: p.spell2.map_or_else(|| "", display_spell),
    };

    JsValue::from_serde(&_player).unwrap()
}

#[wasm_bindgen]
pub fn update(key: char) {
    let mut p = ctx::PLAYER.write().unwrap();
    match key {
        'q' => p.cast_element(Element::Quas),
        'w' => p.cast_element(Element::Wex),
        'e' => p.cast_element(Element::Exort),
        'r' => p.invoke(),
        // 'd' => {}
        // 'f' => {}
        _ => {}
    }
}

fn display_element(e: Element) -> char {
    match e {
        Element::Quas => 'Q',
        Element::Wex => 'W',
        Element::Exort => 'E',
        Element::Void => ' ',
    }
}

fn display_spell(s: Spell) -> &'static str {
    match s {
        Spell::ColdSnap => "Cold Snap",
        Spell::GhostWalk => "Ghost Walk",
        Spell::IceWall => "Ice Wall",
        Spell::Tornado => "Tornado",
        Spell::DeafeningBlast => "Deafening Blast",
        Spell::ForgeSpirit => "Forge Spirit",
        Spell::EMP => "E.M.P",
        Spell::Alacrity => "Alacrity",
        Spell::ChaosMeteor => "Chaos Meteor",
        Spell::SunStrike => "Sun Strike",
    }
}
