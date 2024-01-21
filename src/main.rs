use std::time::{Duration, Instant};

use bevy::prelude::*;
use midi::RecNote;
use partition::{Partition, Figure, Value};

use crate::partition::{Note, Clef};

use self::midi::{MidiPlugin, SendNote, NoteKind};

mod midi;
mod partition;

fn main() {
    partition::ghost_fight();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MidiPlugin { input: false, output: true })
        .add_systems(Startup, (create_camera, spawn_partition))
        // .add_systems(Update, keyboard_play_notes)
        .add_systems(Update, play_part)
        // .add_systems(Update, (spawn_text, send_timed, scroll_text))
        // .add_systems(PostUpdate, remove_invisible_text.after(VisibilitySystems::CheckVisibility))
        .run()
}

fn create_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct Player(Vec<(Timer, usize)>);

fn spawn_partition(mut commands: Commands) {
    let p = partition::ghost_fight();

    commands.spawn((Player(vec![
        (Timer::from_seconds(0.1, TimerMode::Once), 0),
        (Timer::from_seconds(0.1, TimerMode::Once), 0),
    ]), p));
}

fn play_part(mut player: Query<(&Partition, &mut Player)>, time: Res<Time<Real>>, mut notes: EventWriter<SendNote>) {
    fn get_note(clef: &Clef, note: &Note, kind: NoteKind) -> SendNote {
        SendNote {
            channel: 0,
            key: clef.base_do.saturating_add_signed(note.note),
            kind,
            velocity: 120
        }
    }

    for (partition, mut player) in &mut player {
        for (clef, (timer, index)) in partition.clefs.iter().zip(player.0.iter_mut()) {
            if *index >= clef.figures.len() {
                *index = 0;
            }

            timer.tick(time.delta());
            if timer.finished() {
                // stop previous notes
                if *index > 0 {
                    let figure = &clef.figures[*index-1];

                    match figure {
                        Figure::Note(_, note) => {
                            notes.send(get_note(clef, note, NoteKind::Stop))
                        }
                        Figure::Silence(_) => (),
                        Figure::Chord(_, ns) => {
                            for note in ns {
                                notes.send(get_note(clef, note, NoteKind::Stop))
                            }
                        }
                    }
                }

                let figure = &clef.figures[*index];
                let value = figure.value();
                match figure {
                    Figure::Note(_, note) => {
                        notes.send(get_note(clef, note, NoteKind::Play))
                    }
                    Figure::Silence(_) => (),
                    Figure::Chord(_, ns) => {
                        for note in ns {
                            notes.send(get_note(clef, note, NoteKind::Play))
                        }
                    }
                }

                let multiplier = match value {
                    Value::Carre => 8.0,
                    Value::Ronde => 4.0,
                    Value::Blanche => 2.0,
                    Value::Noire => 1.0,
                    Value::Croche => 0.5,
                    Value::DoubleCroche => 0.25,
                    Value::TripleCroche => 0.125
                };

                // x bpm, x/60 bps, 1/(x/60) s, 60/x s
                *timer = Timer::from_seconds(multiplier * 60.0 / (partition.bpm as f32), TimerMode::Once);
                *index += 1;
            }
        }
    }
}

#[derive(Component)]
struct ScrollingNote;

#[derive(Component)]
struct TimedSend(Timer, SendNote);

fn send_timed(mut commands: Commands, mut timed: Query<(Entity, &mut TimedSend)>, time: Res<Time>, mut out: EventWriter<SendNote>) {
    for (entity, mut timed) in &mut timed {
        timed.0.tick(time.delta());

        if timed.0.just_finished() {
            out.send(timed.1);
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_text(mut commands: Commands, mut notes: EventReader<RecNote>) {
    for note in notes.read() {
        commands.spawn(TimedSend(Timer::new(Duration::from_millis(1000), TimerMode::Once), SendNote {
            kind: note.kind,
            key: note.key,
            channel: note.channel,
            velocity: note.velocity
        }));

        if matches!(note.kind, NoteKind::Stop) { continue }
        
        const NOTES: [&str; 12] = [
            "Do", "Do#", "Re", "Re#", "Mi", "Fa", "Fa#", "Sol", "Sol#", "La", "La#", "Si"
        ];

        let text = format!("{} {}", NOTES[note.key as usize], note.octave);

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(text, TextStyle { color: Color::WHITE, font_size: 30.0, ..Default::default() }),
                transform: Transform::from_xyz(note.channel as f32 * 100.0, -500.0, 0.0),
                ..Default::default()
            },
            ScrollingNote
        ));
    }
}

fn scroll_text(mut notes: Query<&mut Transform, With<ScrollingNote>>, time: Res<Time>) {
    for mut note in notes.iter_mut() {
        note.translation.y += 400.0 * time.delta_seconds();
    }
}

fn remove_invisible_text(mut commands: Commands, notes: Query<(Entity, &ViewVisibility), With<ScrollingNote>>) {
    for (entity, visible) in notes.iter() {
        if visible.get() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

