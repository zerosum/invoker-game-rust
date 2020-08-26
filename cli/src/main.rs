extern crate invoker_game_core as core;
extern crate termion as term;

use std::io::{stdin, Stdin, stdout, Stdout, Write};

use term::*;
use term::input::TermRead;
use term::raw::{IntoRawMode, RawTerminal};

use core::ctx;
use core::game::*;

const LINE_NUM_TOP_SKILLS_AREA: u16 = 3;

fn main() {
    let stdin: Stdin = stdin();
    let mut stdout: RawTerminal<Stdout> = stdout().into_raw_mode().unwrap();

    init_display(&mut stdout);

    refresh_display(&mut stdout);

    use event::Key::*;
    for e in stdin.keys() {
        let input = match e {
            Ok(Char('q')) => Input::E1,
            Ok(Char('w')) => Input::E2,
            Ok(Char('e')) => Input::E3,
            Ok(Char('r')) => Input::Invoke,
            // Ok(Char('d')) => Input::S1,
            // Ok(Char('f')) => Input::S2,
            Ok(Ctrl('c')) => break,
            _ => continue
        };

        ctx::cast(input).unwrap();
        refresh_display(&mut stdout)
    }

    write!(stdout, "{}{}{}",  clear::All, cursor::Show, cursor::Goto(1,1)).unwrap();
}

fn init_display(stdout: &mut RawTerminal<Stdout>) {
    write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();

    write!(stdout, "\
{}{}Invoker Game{} ('^+C': quit)\
{}-------------------\
{}[Q]: Quas\
{}[W]: Wex\
{}[E]: Exort\
{}[R]: Invoke\
{}-------------------",
           cursor::Goto(1, 1), style::Bold, style::Reset,
           cursor::Goto(1, LINE_NUM_TOP_SKILLS_AREA),
           cursor::Goto(1, LINE_NUM_TOP_SKILLS_AREA + 1),
           cursor::Goto(1, LINE_NUM_TOP_SKILLS_AREA + 2),
           cursor::Goto(1, LINE_NUM_TOP_SKILLS_AREA + 3),
           cursor::Goto(1, LINE_NUM_TOP_SKILLS_AREA + 4),
           cursor::Goto(1, LINE_NUM_TOP_SKILLS_AREA + 7),
    ).unwrap();

    stdout.flush().unwrap();
}

fn refresh_display(stdout: &mut RawTerminal<Stdout>) {
    let player: Invoker = ctx::player().unwrap();

    write!(stdout, "{}{}{}[{}][{}][{}]{}",
           cursor::Goto(1, LINE_NUM_TOP_SKILLS_AREA + 9),
           clear::CurrentLine,
           style::Bold,
           display_element(player.elements[0]),
           display_element(player.elements[1]),
           display_element(player.elements[2]),
           style::Reset,
    ).unwrap();

    write!(stdout, "{}{}[D]: {}{}{}[F]: {}",
           cursor::Goto(1, LINE_NUM_TOP_SKILLS_AREA + 5), clear::CurrentLine,
           player.spell1.map_or_else(|| { "" }, display_spell),
           cursor::Goto(1, LINE_NUM_TOP_SKILLS_AREA + 6), clear::CurrentLine,
           player.spell2.map_or_else(|| { "" }, display_spell),
    ).unwrap();

    stdout.flush().unwrap();
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
