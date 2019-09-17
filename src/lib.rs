pub mod chord;
mod note;
pub mod ukulele;

/// Type for the number of frets (corresponds to the number of semitones)
/// to move from one note or pitch class to another.
pub type Frets = u8;

/// Number of strings on our string instrument.
pub const STRING_COUNT: usize = 4;
