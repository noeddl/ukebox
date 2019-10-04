#![allow(clippy::module_inception)]
mod chord;
mod chord_shape;

pub use self::chord::Chord;
pub use self::chord::ChordQuality;
pub use self::chord_shape::ChordShape;
pub use self::chord_shape::ChordShapeSet;

use crate::STRING_COUNT;

/// Type for the number of frets (corresponds to the number of semitones)
/// to move from one note or pitch class to another.
pub type FretID = u8;
pub type FretPattern = [FretID; STRING_COUNT];
