extern crate invoker_game_core as core;
extern crate termion as term;

use std::io::{stdin, Stdin, stdout, Stdout, Write};

use term::*;
use term::input::TermRead;
use term::raw::{IntoRawMode, RawTerminal};

use core::game::*;

const LINE_NUM_TOP_SKILLS_AREA: u16 = 3;

fn main() {
    let stdin: Stdin = stdin();
    let mut stdout: RawTerminal<Stdout> = stdout().into_raw_mode().unwrap();

    init_display(&mut stdout);

    let mut player = Invoker::new();

    refresh_display(&mut stdout, player);

    use event::Key::*;
    for e in stdin.keys() {
        match e {
            Ok(Char('q')) => player.cast_element(Element::Quas),
            Ok(Char('w')) => player.cast_element(Element::Wex),
            Ok(Char('e')) => player.cast_element(Element::Exort),
            Ok(Char('r')) => player.invoke(),
            Ok(Ctrl('c')) => break,
            _ => {}
        }
        refresh_display(&mut stdout, player)
    }

    write!(stdout, "{}", cursor::Show).unwrap();
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

fn refresh_display(stdout: &mut RawTerminal<Stdout>, player: Invoker) {
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
