mod chord_diagram;
mod fret_pattern;

pub use self::chord_diagram::ChordDiagram;
pub use self::fret_pattern::FretPattern;

use crate::note::Semitones;

/// Number of frets shown on the fretboard chart.
pub const CHART_WIDTH: Semitones = 4;
