
use bevy::prelude::*;

use self::{input::{create_midi_input, bridge_midi_input_channel_event}, output::{create_midi_output, midi_output_play_notes}};
pub use self::{input::RecNote, output::SendNote};

mod input;
mod output;

#[derive(Clone, Copy)]
pub enum NoteKind {
    Play, Stop
}

pub struct MidiPlugin {
    pub input: bool,
    pub output: bool
}

impl Plugin for MidiPlugin {
    fn build(&self, app: &mut App) {
        if self.input {
            app
                .add_event::<RecNote>()
                .add_systems(Startup, create_midi_input)
                .add_systems(FixedUpdate, bridge_midi_input_channel_event);
        }

        if self.output {
            app
                .add_event::<SendNote>()
                .add_systems(Startup, create_midi_output)
                .add_systems(FixedUpdate, midi_output_play_notes);
        }
    }
}
