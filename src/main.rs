use midir::MidiOutput;
use wmidi::{Channel, MidiMessage, Note, U7};

fn main() {
    let midi_out = MidiOutput::new("leddy").unwrap();
    if midi_out.port_count() == 0 {
        panic!("No MIDI output ports available.");
    }

    let mut port_out = midi_out.ports()
        .into_iter()
        .find(|p| {
            let port_name = midi_out.port_name(p).unwrap();
            port_name.to_ascii_lowercase().contains("ddj")
        })
        .expect("No DDJ MIDI output port found.");

    let port_name = midi_out.port_name(&port_out).unwrap();
    println!("Connecting to port {}: {}", port_out.id(), port_name);
    let mut conn_out = midi_out.connect(&port_out, "leddy-out").unwrap();

    let ch = Channel::Ch8;
    let note = Note::CMinus1;
    let velocity = 127;

    let msg = MidiMessage::NoteOn(ch, note, U7::from_u8_lossy(velocity));
    let mut bytes = [0u8; 3];
    msg.copy_to_slice(&mut bytes).unwrap();
    conn_out.send(&bytes).unwrap();
    
    std::thread::sleep(std::time::Duration::from_secs(1));

    let msg = MidiMessage::NoteOn(ch, note, U7::from_u8_lossy(0));
    let mut bytes = [0u8; 3];
    msg.copy_to_slice(&mut bytes).unwrap();
    conn_out.send(&bytes).unwrap();
}