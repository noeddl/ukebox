use crate::note::Interval;
use std::fmt;
use std::str::FromStr;

/// The type of the chord depending on the intervals it contains.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChordType {
    Major,
    Minor,
    SuspendedSecond,
    SuspendedFourth,
    Augmented,
    Diminished,
    DominantSeventh,
    MinorSeventh,
    MajorSeventh,
    MinorMajorSeventh,
    AugmentedSeventh,
    AugmentedMajorSeventh,
    DiminishedSeventh,
    HalfDiminishedSeventh,
}

impl ChordType {
    pub fn get_intervals(self) -> Vec<Interval> {
        use ChordType::*;

        let interval_names = match self {
            Major => vec!["P1", "M3", "P5"],
            Minor => vec!["P1", "m3", "P5"],
            SuspendedSecond => vec!["P1", "M2", "P5"],
            SuspendedFourth => vec!["P1", "P4", "P5"],
            Augmented => vec!["P1", "M3", "A5"],
            Diminished => vec!["P1", "m3", "d5"],
            DominantSeventh => vec!["P1", "M3", "P5", "m7"],
            MinorSeventh => vec!["P1", "m3", "P5", "m7"],
            MajorSeventh => vec!["P1", "M3", "P5", "M7"],
            MinorMajorSeventh => vec!["P1", "m3", "P5", "M7"],
            AugmentedSeventh => vec!["P1", "M3", "A5", "m7"],
            AugmentedMajorSeventh => vec!["P1", "M3", "A5", "M7"],
            DiminishedSeventh => vec!["P1", "m3", "d5", "d7"],
            HalfDiminishedSeventh => vec!["P1", "m3", "d5", "m7"],
        };

        interval_names
            .iter()
            .map(|s| Interval::from_str(s).unwrap())
            .collect()
    }
}

impl fmt::Display for ChordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ChordType::*;

        let s = match self {
            Major => "major",
            Minor => "minor",
            SuspendedSecond => "suspended 2nd",
            SuspendedFourth => "suspended 4th",
            Augmented => "augmented",
            Diminished => "diminished",
            DominantSeventh => "dominant 7th",
            MinorSeventh => "minor 7th",
            MajorSeventh => "major 7th",
            MinorMajorSeventh => "minor/major 7th",
            AugmentedSeventh => "augmented 7th",
            AugmentedMajorSeventh => "augmented major 7th",
            DiminishedSeventh => "diminished 7th",
            HalfDiminishedSeventh => "half-diminished 7th",
        };

        write!(f, "{}", s)
    }
}
