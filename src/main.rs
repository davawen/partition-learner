use bevy::prelude::*;
use midi::RecNote;

use self::midi::{MidiPlugin, SendNote, NoteKind};

mod midi;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MidiPlugin { input: true, output: false })
        .add_systems(Startup, create_camera)
        // .add_systems(Update, keyboard_play_notes)
        .add_systems(Update, (spawn_text, scroll_text))
        // .add_systems(PostUpdate, remove_invisible_text.after(VisibilitySystems::CheckVisibility))
        .run()
}

fn create_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn keyboard_play_notes(mut notes: EventWriter<SendNote>, input: Res<Input<KeyCode>>) {
    macro_rules! map {
        ($($code:expr => $octave:literal, $key:literal, $channel:literal),+ $(,)?) => {
            $(
                if input.just_pressed($code) {
                    notes.send(SendNote { octave: $octave, key: $key, channel: $channel, kind: NoteKind::Play, velocity: 0xf0 });
                } else if input.just_released($code) {
                    notes.send(SendNote { octave: $octave, key: $key, channel: $channel, kind: NoteKind::Stop, velocity: 0xf0 });
                }
            )+
        };
    };

    map! {
        KeyCode::Q => 5, 2, 0,
        KeyCode::W => 5, 0, 0,
        KeyCode::E => 4, 11, 0,
        KeyCode::R => 4, 10, 0,
        KeyCode::A => 6, 2, 0,
        KeyCode::S => 5, 9, 0,
        KeyCode::D => 5, 8, 0,
        KeyCode::F => 5, 7, 0,
        KeyCode::Z => 5, 5, 0,
        KeyCode::X => 5, 2, 0,
        KeyCode::C => 5, 5, 0,
        KeyCode::V => 5, 7, 0,

        KeyCode::J => 5, 2, 1,
        KeyCode::K => 5, 0, 1,
        KeyCode::L => 4, 11, 1,
        KeyCode::M => 4, 10, 1,
    }

}

#[derive(Component)]
struct ScrollingNote;

fn spawn_text(mut commands: Commands, mut notes: EventReader<RecNote>) {
    for note in notes.read() {
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

