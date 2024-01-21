use std::io::{stdout, stdin, Write};

use bevy::{prelude::*, utils::synccell::SyncCell};
use midir::{MidiOutput, MidiOutputPort, MidiOutputConnection};

use super::NoteKind;

#[derive(Resource)]
pub struct ResMidiOutputConnection(SyncCell<MidiOutputConnection>);

pub fn create_midi_output(mut commands: Commands) {
    let midi_out = MidiOutput::new("My Test Output")
        .expect("couldn't create output");

    // Get an output port (read from console if multiple are available)
    let out_ports = midi_out.ports();
    let out_port: &MidiOutputPort = match out_ports.len() {
        0 => panic!("no output port found"),
        1 => {
            println!(
                "Choosing the only available output port: {}",
                midi_out.port_name(&out_ports[0]).unwrap()
            );
            &out_ports[0]
        }
        _ => {
            println!("\nAvailable output ports:");
            for (i, p) in out_ports.iter().enumerate() {
                println!("{}: {}", i, midi_out.port_name(p).unwrap());
            }
            print!("Please select output port: ");
            stdout().flush().unwrap();
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            out_ports
                .get(input.trim().parse::<usize>().unwrap())
                .expect("invalid output port selected")
        }
    };

    let connection = midi_out.connect(out_port, "piano").unwrap();
    commands.insert_resource(ResMidiOutputConnection(SyncCell::new(connection)));
}

#[derive(Clone, Copy, Event)]
pub struct SendNote {
    pub key: u8,
    /// available range: 0..=15
    pub channel: u8,
    /// available range: 1..=127
    pub velocity: u8,
    pub kind: NoteKind
}

pub fn midi_output_play_notes(mut notes: EventReader<SendNote>, mut connection: ResMut<ResMidiOutputConnection>) {
    for &note in notes.read() {
        const NOTE_ON_MSG: u8 = 0x90;
        const NOTE_OFF_MSG: u8 = 0x80;

        let msg = match note.kind {
            NoteKind::Play => NOTE_ON_MSG,
            NoteKind::Stop => NOTE_OFF_MSG
        };

        connection.0.get().send(&[msg + note.channel, note.key, note.velocity]).expect("failed to send midi message");
    }
}
