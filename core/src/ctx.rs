use std::ops::Deref;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use game::*;

lazy_static! {
    pub static ref PLAYER: RwLock<Invoker> = RwLock::new(Invoker::new());
}

pub fn init() -> Result<(), String> {
    let mut p: RwLockWriteGuard<Invoker> = PLAYER.write()
        .map_err(|e| e.to_string())?;

    p.clear();

    Ok(())
}

pub fn player() -> Result<Invoker, String> {
    let p: RwLockReadGuard<Invoker> = PLAYER.read()
        .map_err(|e| e.to_string())?;

    Ok(*p.deref())
}

pub fn cast(input: Input) -> Result<(), String> {
    let mut p: RwLockWriteGuard<Invoker> = PLAYER.write()
        .map_err(|e| e.to_string())?;

    match input {
        Input::E1 => p.cast_element(Element::Quas),
        Input::E2 => p.cast_element(Element::Wex),
        Input::E3 => p.cast_element(Element::Exort),
        Input::Invoke => p.invoke(),
        Input::S1 => {}
        Input::S2 => {}
    }

    Ok(())
}
