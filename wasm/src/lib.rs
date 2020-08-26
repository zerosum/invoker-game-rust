extern crate invoker_game_core as core;
extern crate serde;
extern crate wasm_bindgen;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use core::game::*;
use core::ctx;

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
    let p = ctx::player().unwrap();

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
    let input: Input = match key {
        'q' => Input::E1,
        'w' => Input::E2,
        'e' => Input::E3,
        'r' => Input::Invoke,
        'd' => Input::S1,
        'f' => Input::S2,
        _ => return
    };

    ctx::cast(input).unwrap()
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
