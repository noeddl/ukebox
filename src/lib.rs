pub mod chord;
pub mod chord_diagram;
pub mod chord_type;
pub mod fret_pattern;
pub mod interval;
pub mod note;
pub mod pitch_class;
pub mod staff_position;
pub mod tuning;

pub use chord::Chord;
pub use chord_diagram::ChordDiagram;
pub use chord_type::ChordType;
pub use fret_pattern::FretPattern;
pub use interval::Interval;
pub use note::Note;
pub use pitch_class::PitchClass;
pub use staff_position::StaffPosition;
pub use tuning::Tuning;

/// Number of strings on our string instrument.
pub const STRING_COUNT: usize = 4;

/// The ID of a fret on the fretboard. 0 corresponds to the nut,
/// 1 corresponds to the first fret, 2 to the second etc.
pub type FretID = u8;

/// The number of semitones (corresponds to the number of frets)
/// to move from one note or pitch class to another.
pub type Semitones = u8;

/// The number of steps in a staff to move from one staff position
/// to another.
type StaffSteps = u8;
