use std::{collections::HashMap, io::Read, result, time::Instant};

use midir::{MidiOutput, MidiOutputConnection};
use wmidi::{Channel, MidiMessage, Note, U7};

use crate::common::err::{self, Error, Result};

pub mod common;

fn setup() -> Result<MidiOutputConnection> {
    let midi_out = MidiOutput::new("leddy").unwrap();
    if midi_out.port_count() == 0 {
        return Err(Error::NoMidiOutputPorts);
    }

    let port_out = midi_out.ports()
        .into_iter()
        .find(|p| {
            let port_name = midi_out.port_name(p).unwrap();
            port_name.to_ascii_lowercase().contains("ddj")
        })
        .ok_or(Error::NoMidiOutputPorts)?;

    let port_name = midi_out.port_name(&port_out).unwrap();
    println!("Connecting to port {}: {}", port_out.id(), port_name);
    Ok(midi_out.connect(&port_out, "leddy-out")?)
}

struct Actor(Vec<Channel>, Note);
impl TryFrom<csv::StringRecord> for Actor {
    type Error = err::Error;

    fn try_from(value: csv::StringRecord) -> std::result::Result<Self, Self::Error> {
        let channel = value.get(0).ok_or(Error::CsvFormat("Missing channel".into()))?;
        let note = value.get(1).ok_or(Error::CsvFormat("Missing note".into()))?;
        let channels = channel.split('/').map(|s| s.parse::<u8>())
            .collect::<result::Result<Vec<u8>, _>>()
            .map_err(|e| Error::CsvFormat(format!("Invalid channel: {}", e)))?
            .into_iter()
            .map(|n| Channel::from_index(n - 1).or(Err(Error::CsvFormat(format!("Channel out of range: {}", n)))))
            .collect::<Result<Vec<Channel>>>()?;
        let note = Note::from_u8_lossy(note.trim().parse::<u8>()?);
        Ok(Self(channels, note))
    }
}

fn load() -> Result<Vec<Actor>> {
    let mut rdr = csv::Reader::from_path("notes.csv")?;
    rdr.records()
        .map(|r| r.map_err(Error::from).and_then(|res| Actor::try_from(res)))
        .collect()
}

type State = HashMap<(Channel, Note), Instant>;

fn tick(state: &mut State) {
    let now = Instant::now();
    state.retain(|_, &mut t| now.duration_since(t).as_secs() < 1);
}

fn main() {
    let mut state: State = HashMap::new();

    let mut conn = setup().unwrap();
    let notes = load().unwrap();

    loop {
        tick(&mut state);
        for actor in &notes {
            for ch in &actor.0 {
                if !state.contains_key(&(*ch, actor.1)) {
                    let msg = MidiMessage::NoteOn(ch.clone(), actor.1.clone(), U7::MIN);
                    let mut bytes = [0u8; 3];
                    msg.copy_to_slice(&mut bytes).unwrap();
                    conn.send(&bytes).unwrap();
                } else {
                    let msg = MidiMessage::NoteOn(ch.clone(), actor.1.clone(), U7::MAX);
                    let mut bytes = [0u8; 3];
                    msg.copy_to_slice(&mut bytes).unwrap();
                    conn.send(&bytes).unwrap();
                }

                let rng = rand::random::<u8>();
                if rng < 5 { // i.e. ~2% chance
                    state.insert((*ch, actor.1), Instant::now());
                }
            }
        }
    }
}