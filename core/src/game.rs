#[derive(Ord, Eq, PartialOrd, PartialEq, Copy, Clone)]
pub enum Element {
    Void,
    Quas,
    Wex,
    Exort,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Spell {
    ColdSnap,
    GhostWalk,
    IceWall,
    Tornado,
    DeafeningBlast,
    ForgeSpirit,
    EMP,
    Alacrity,
    ChaosMeteor,
    SunStrike,
}

#[derive(Copy, Clone)]
pub struct Invoker {
    pub elements: [Element; 3],
    pub spell1: Option<Spell>,
    pub spell2: Option<Spell>,
}

#[derive(Eq, PartialEq)]
pub enum Input {
    E1,
    E2,
    E3,
    Invoke,
    S1,
    S2,
}

impl Invoker {
    pub fn new() -> Invoker {
        Invoker {
            elements: [Element::Void; 3],
            spell1: None,
            spell2: None,
        }
    }

    pub fn clear(&mut self) {
        self.elements = [Element::Void; 3];
        self.spell1 = None;
        self.spell2 = None;
    }

    pub fn cast_element(&mut self, e: Element) {
        self.elements = [self.elements[1], self.elements[2], e];
    }

    pub fn invoke(&mut self) {
        use crate::game::{Element::*, Spell::*};

        let mut tmp: [Element; 3] = self.elements;
        tmp.sort();
        let maybe_invoked: Option<Spell> = match tmp {
            [Quas, Quas, Quas] => Some(ColdSnap),
            [Quas, Quas, Wex] => Some(GhostWalk),
            [Quas, Quas, Exort] => Some(IceWall),
            [Quas, Wex, Wex] => Some(Tornado),
            [Quas, Wex, Exort] => Some(DeafeningBlast),
            [Quas, Exort, Exort] => Some(ForgeSpirit),
            [Wex, Wex, Wex] => Some(EMP),
            [Wex, Wex, Exort] => Some(Alacrity),
            [Wex, Exort, Exort] => Some(ChaosMeteor),
            [Exort, Exort, Exort] => Some(SunStrike),
            _ => None,
        };

        match (maybe_invoked, self.spell1) {
            (None, _) => {}
            (Some(invoked), Some(s1)) if invoked == s1 => {}
            _ => {
                self.spell2 = self.spell1;
                self.spell1 = maybe_invoked;
            }
        }
    }

    pub fn cast_spell1(self) -> Option<Spell> {
        self.spell1
    }

    pub fn cast_spell2(self) -> Option<Spell> {
        self.spell2
    }
}
