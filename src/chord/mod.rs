#![allow(clippy::module_inception)]
mod chord;
mod chord_shape;
mod chord_type;
mod tuning;

pub use self::chord::Chord;
pub use self::chord_shape::ChordShape;
pub use self::chord_shape::ChordShapeSet;
pub use self::chord_type::ChordType;
pub use self::tuning::Tuning;

use crate::STRING_COUNT;

/// The ID of a fret on the fretboard. 0 corresponds to the nut,
/// 1 corresponds to the first fret, 2 to the second etc.
pub type FretID = u8;

/// A pattern of frets to push down for playing a chord.
/// Each index of the array corresponds to a ukulele string.
pub type FretPattern = [FretID; STRING_COUNT];
