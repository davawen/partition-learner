use std::{io::{stdout, stdin, Write}, sync::mpsc};

use bevy::{prelude::*, utils::synccell::SyncCell};
use midir::{MidiInput, MidiInputConnection};

use super::NoteKind;

#[derive(Resource)]
pub struct ResMidiInputConnection {
    #[allow(unused)]
    /// used to keep the conneciton alive
    conn: SyncCell<MidiInputConnection<mpsc::Sender<RecNote>>>,
    reciever: SyncCell<mpsc::Receiver<RecNote>>
}

pub fn create_midi_input(mut commands: Commands) {
    let midi_in = MidiInput::new("My Test Output")
        .expect("couldn't create output");

    // Get an output port (read from console if multiple are available)
    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
        0 => panic!("no input port found"),
        1 => {
            println!(
                "Choosing the only available input port: {}",
                midi_in.port_name(&in_ports[0]).unwrap()
            );
            &in_ports[0]
        }
        _ => {
            println!("\nAvailable input ports:");
            for (i, p) in in_ports.iter().enumerate() {
                println!("{}: {}", i, midi_in.port_name(p).unwrap());
            }
            print!("Please select input port: ");
            stdout().flush().unwrap();
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            in_ports
                .get(input.trim().parse::<usize>().unwrap())
                .expect("invalid input port selected")
        }
    };

    let (sender, reciever) = mpsc::channel();

    let connection = midi_in.connect(in_port, "piano", |time, data, sender| {
        if data.len() == 3 { // channel voice message
            let kind = match data {
                [ 0x80..=0x8F, _, _ ] | [ 0x90..=0x9F, _, 0 ] => NoteKind::Stop,
                [ 0x90..=0x9F, _, _ ] => NoteKind::Play,
                _ => return
            };

            let &[status, key, velocity] = data else { unreachable!() };

            let channel = status & 0xf; // get the first nibble
            let octave = key / 12;
            let key = key % 12;

            sender.send(RecNote {
                channel, octave, key, velocity, kind
            }).unwrap();
        }
    }, sender).unwrap();

    commands.insert_resource(ResMidiInputConnection {
        conn: SyncCell::new(connection),
        reciever: SyncCell::new(reciever)
    });
}

#[derive(Clone, Copy, Event)]
pub struct RecNote {
    /// available range: 0..=15
    pub channel: u8,
    pub octave: u8,
    /// available range: 0..=11
    pub key: u8,
    /// available range: 1..=127
    pub velocity: u8,
    pub kind: NoteKind
}

pub fn bridge_midi_input_channel_event(mut input: ResMut<ResMidiInputConnection>, mut notes: EventWriter<RecNote>) {
    let reciever = input.reciever.get();
    while let Ok(note) = reciever.try_recv() {
        notes.send(note);
    }
}
