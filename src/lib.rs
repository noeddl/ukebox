pub mod chord;
mod diagram;
pub mod note;

/// Type for the number of frets (corresponds to the number of semitones)
/// to move from one note or pitch class to another.
pub type Frets = u8;

/// Number of strings on our string instrument.
pub const STRING_COUNT: usize = 4;

use crate::note::Interval;
pub type IntervalPattern = [Interval; STRING_COUNT];

use crate::note::Note;
pub type NotePattern = [Note; STRING_COUNT];
