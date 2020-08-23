extern crate invoker_game_core as core;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use core::game::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn run() {
    let mut player = Invoker::new();
    player.cast_element(Element::Exort);
    player.cast_element(Element::Exort);
    player.cast_element(Element::Exort);
    player.invoke();

    log(player.spell1.map_or_else(|| "empty", display_spell))
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
