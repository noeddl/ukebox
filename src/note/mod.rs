#![allow(clippy::module_inception)]
mod interval;
mod note;
mod pitch_class;
mod staff_position;

pub use self::interval::Interval;
pub use self::note::Note;
pub use self::pitch_class::PitchClass;
pub use self::staff_position::StaffPosition;

/// The number of semitones (corresponds to the number of frets)
/// to move from one note or pitch class to another.
pub type Semitones = u8;

/// The number of steps in a staff to move from one staff position
/// to another.
type StaffSteps = u8;
