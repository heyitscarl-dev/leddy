use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("MIDI initialization error: {0}")]
    MidiInit(#[from] midir::InitError),

    #[error("MIDI connection error: {0}")]
    MidiConnectIo(#[from] midir::ConnectError<std::io::Error>),

    #[error("MIDI connection error: {0}")]
    MidiConnectOutput(#[from] midir::ConnectError<midir::MidiOutput>),

    #[error("MIDI send error: {0}")]
    MidiSend(#[from] midir::SendError),

    #[error("No MIDI output ports available")]
    NoMidiOutputPorts,

    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),

    #[error("CSV format error: {0}")]
    CsvFormat(String),

    #[error("Number parsing error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
}

pub type Result<T> = std::result::Result<T, Error>;