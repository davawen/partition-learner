fn parse_value(input: &str) -> Value {
     match input {
        "C" => Value::Carre,
        "r" => Value::Ronde,
        "b" => Value::Blanche,
        "n" => Value::Noire,
        "c" =>   Value::Croche,
        "cc" =>  Value::DoubleCroche,
        "ccc" => Value::TripleCroche,
        _ => panic!()
    }
}

fn parse_note(input: &str) -> Note {
    let offset = input.chars().fold(0, |a, c| match c {
        '+' => a + 12,
        '-' => a - 12,
        _ => a
    });

    let note = match input {
        s if s.starts_with("Do") => 0,
        s if s.starts_with("Reb") => 1,
        s if s.starts_with("Re") => 2,
        s if s.starts_with("MiB") => 3,
        s if s.starts_with("Mi") => 4,
        s if s.starts_with("Fa") => 5,
        s if s.starts_with("SolB") => 6,
        s if s.starts_with("Sol") => 7,
        s if s.starts_with("LaB") => 8,
        s if s.starts_with("La") => 9,
        s if s.starts_with("SiB") => 10,
        s if s.starts_with("Si") => 11,
        _ => panic!("unknown note: {input}")
    };

    Note {
        note: note + offset,
        dot: input.contains('.')
    }
}

fn parse_figure(input: &str) -> Figure {
    let (value, rest) = input.split_once('-').unwrap();
    let value = parse_value(value);

    if rest.starts_with('(') && rest.ends_with(')') {
        let notes = rest[1..rest.len()-1].split('_');
        Figure::Chord(value, notes.map(parse_note).collect())
    } else if rest == "X" {
        Figure::Silence(value)
    } else {
        Figure::Note(value, parse_note(rest))
    }
}

fn parse_mesure(input: &str) -> Vec<Figure> {
    let input = input.split_whitespace();

    input.map(parse_figure).collect()
}

pub fn your_best_friend() -> Partition {
    Partition {
        bpm: 125,
        clefs: vec![
            Clef {
                mesure: (4, 4),
                kind: ClefKind::Sol,
                figures: parse_mesure(
                    "c-LaB c-X c-Do+ c-X c-MiB c-X c-Do+ c-X"
                ),
                base_do: 5*12
            }
        ]
    }
}

pub fn ghost_fight() -> Partition {
    Partition {
        bpm: 113,
        clefs: vec![
            Clef {
                mesure: (4, 4),
                kind: ClefKind::Sol,
                figures: parse_mesure("
c-(Mi_Sol) c-(Sol_SiB) c-(La_Do+) c-(Mi_Sol) c-(Sol_SiB) cc-(La_Do+) cc-(SiB_ReB+) cc-X cc-(Si_Re+) c-(La_SolB+)
c-(Mi_Sol) c-(Sol_SiB) c-(La_Do+) c-(Mi_Sol) c-(Sol_SiB) cc-(SiB_ReB+) cc-(Si_Re+) cc-X cc-(Re+_Fa+) c-(Do+_La+)
c-(Mi_Sol) c-(Sol_SiB) c-(La_Do+) c-(Mi_Sol) c-(Sol_SiB) cc-(La_Do+) cc-(SiB_ReB+) cc-X cc-(Si_Re+) c-(La_SolB+)
c-(Mi_Sol) c-(Sol_SiB) c-(La_Do+) c-(Mi_Sol) cc-(Re_Fa) c-(MiB_SolB) c-(Mi_Sol) cc-X c-X
cc-(MiB_Sol) cc-(Mi_Sol) c-(Sol_SiB) c-(La_Do+) c-(Mi_Sol) c-(Sol_SiB) cc-(La_Do+) cc-(SiB_ReB+) cc-X cc-(Si_Re) c-(La_SolB+)
r-X
                "),
                base_do: 5*12
            },
            Clef {
                mesure: (4, 4),
                kind: ClefKind::Fa,
                figures: parse_mesure("
r-X r-X r-X r-X
cc-Sol cc-Sol c-SolB c-Fa c-Mi c-MiB c-SiB- c-La- c-Re-
cc-Sol cc-Sol c-SolB c-Fa c-Mi c-MiB c-Sol c-SolB c-Re
                "),
                base_do: 3*12
            }
        ]
    }
}

#[derive(bevy::prelude::Component)]
pub struct Partition {
    pub clefs: Vec<Clef>,
    pub bpm: u16
}

pub struct Clef {
    pub mesure: (u8, u8),
    // armure: ,
    pub figures: Vec<Figure>,
    pub kind: ClefKind,
    pub base_do: u8
}

pub enum ClefKind {
    Sol,
    Fa,
    Ut
}

#[derive(Clone)]
pub enum Figure {
    Note(Value, Note),
    Chord(Value, Vec<Note>),
    Silence(Value),
}

impl Figure {
    pub fn value(&self) -> Value {
        match self {
            &Figure::Note(v, _) => v,
            &Figure::Chord(v, _) => v,
            &Figure::Silence(v) => v,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Note {
    /// écart du do de la clef
    pub note: i8,
    pub dot: bool
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Value {
    Carre,
    Ronde,
    Blanche,
    Noire,
    Croche,
    DoubleCroche,
    TripleCroche
}
