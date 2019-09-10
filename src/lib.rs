pub mod chord;
mod note;
mod streng;
pub mod ukulele;

/// Type for the number of frets (corresponds to the number of semitones)
/// to move from one note or pitch class to another. It can take positive
/// and negative values as you can move up and down the fretboard (or the
/// scale).
type Frets = i8;
