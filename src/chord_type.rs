use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use crate::{Interval, PitchClass};

/// The type of the chord depending on the intervals it contains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChordType {
    Major,
    MajorSeventh,
    MajorNinth,
    DominantSeventh,
    SuspendedFourth,
    SuspendedSecond,
    Minor,
    MinorSeventh,
    MinorMajorSeventh,
    Diminished,
    DiminishedSeventh,
    HalfDiminishedSeventh,
    Augmented,
    AugmentedSeventh,
    AugmentedMajorSeventh,
}

impl ChordType {
    /// Return an iterator over the chord type's intervals.
    pub fn intervals(&self) -> impl Iterator<Item = Interval> + '_ {
        use ChordType::*;

        let interval_names = match self {
            Major => vec!["P1", "M3", "P5"],
            MajorSeventh => vec!["P1", "M3", "P5", "M7"],
            MajorNinth => vec!["P1", "M3", "P5", "M7", "M9"],
            DominantSeventh => vec!["P1", "M3", "P5", "m7"],
            SuspendedFourth => vec!["P1", "P4", "P5"],
            SuspendedSecond => vec!["P1", "M2", "P5"],
            Minor => vec!["P1", "m3", "P5"],
            MinorSeventh => vec!["P1", "m3", "P5", "m7"],
            MinorMajorSeventh => vec!["P1", "m3", "P5", "M7"],
            Diminished => vec!["P1", "m3", "d5"],
            DiminishedSeventh => vec!["P1", "m3", "d5", "d7"],
            HalfDiminishedSeventh => vec!["P1", "m3", "d5", "m7"],
            Augmented => vec!["P1", "M3", "A5"],
            AugmentedSeventh => vec!["P1", "M3", "A5", "m7"],
            AugmentedMajorSeventh => vec!["P1", "M3", "A5", "M7"],
        };

        interval_names
            .into_iter()
            .map(|s| Interval::from_str(s).unwrap())
    }

    pub fn to_symbol(self) -> String {
        use ChordType::*;

        let s = match self {
            Major => "",
            MajorSeventh => "maj7",
            MajorNinth => "maj9",
            DominantSeventh => "7",
            SuspendedFourth => "sus4",
            SuspendedSecond => "sus2",
            Minor => "m",
            MinorSeventh => "m7",
            MinorMajorSeventh => "mMaj7",
            Diminished => "dim",
            DiminishedSeventh => "dim7",
            HalfDiminishedSeventh => "m7b5",
            Augmented => "aug",
            AugmentedSeventh => "aug7",
            AugmentedMajorSeventh => "augMaj7",
        };

        s.to_string()
    }
}

impl fmt::Display for ChordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ChordType::*;

        let s = match self {
            Major => "major",
            MajorSeventh => "major 7th",
            MajorNinth => "major 9th",
            DominantSeventh => "dominant 7th",
            SuspendedFourth => "suspended 4th",
            SuspendedSecond => "suspended 2nd",
            Minor => "minor",
            MinorSeventh => "minor 7th",
            MinorMajorSeventh => "minor/major 7th",
            Diminished => "diminished",
            DiminishedSeventh => "diminished 7th",
            HalfDiminishedSeventh => "half-diminished 7th",
            Augmented => "augmented",
            AugmentedSeventh => "augmented 7th",
            AugmentedMajorSeventh => "augmented major 7th",
        };

        write!(f, "{}", s)
    }
}

impl FromStr for ChordType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ChordType::*;

        match s {
            "" => Ok(Major),
            "maj7" => Ok(MajorSeventh),
            "maj9" => Ok(MajorNinth),
            "7" => Ok(DominantSeventh),
            "sus4" => Ok(SuspendedFourth),
            "sus2" => Ok(SuspendedSecond),
            "m" => Ok(Minor),
            "m7" => Ok(MinorSeventh),
            "mMaj7" => Ok(MinorMajorSeventh),
            "dim" => Ok(Diminished),
            "dim7" => Ok(DiminishedSeventh),
            "m7b5" => Ok(HalfDiminishedSeventh),
            "aug" => Ok(Augmented),
            "aug7" => Ok(AugmentedSeventh),
            "augMaj7" => Ok(AugmentedMajorSeventh),
            _ => Err("no valid chord type"),
        }
    }
}

impl TryFrom<&[PitchClass]> for ChordType {
    type Error = &'static str;

    /// Determine the chord type from a list of pitch classes representing a chord.
    fn try_from(pitches: &[PitchClass]) -> Result<Self, Self::Error> {
        use ChordType::*;

        // Subtract the root note's pitch class from all pitch classes to get the
        // difference in semitones.
        let mut pitch_diffs: Vec<_> = pitches.iter().map(|pc| *pc - pitches[0]).collect();

        pitch_diffs.sort_unstable();

        match pitch_diffs[..] {
            [0, 4, 7] => Ok(Major),
            [0, 4, 7, 11] => Ok(MajorSeventh),
            [0, 2, 4, 7, 11] => Ok(MajorNinth),
            [0, 4, 7, 10] => Ok(DominantSeventh),
            [0, 5, 7] => Ok(SuspendedFourth),
            [0, 2, 7] => Ok(SuspendedSecond),
            [0, 3, 7] => Ok(Minor),
            [0, 3, 7, 10] => Ok(MinorSeventh),
            [0, 3, 7, 11] => Ok(MinorMajorSeventh),
            [0, 3, 6] => Ok(Diminished),
            [0, 3, 6, 9] => Ok(DiminishedSeventh),
            [0, 3, 6, 10] => Ok(HalfDiminishedSeventh),
            [0, 4, 8] => Ok(Augmented),
            [0, 4, 8, 10] => Ok(AugmentedSeventh),
            [0, 4, 8, 11] => Ok(AugmentedMajorSeventh),
            _ => Err("No matching chord type found."),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use ChordType::*;
    use PitchClass::*;

    use super::*;

    #[rstest(
        pitches, chord_type,
        // Test C-chords.
        case(vec![C, E, G], Major),
        case(vec![C, E, G, B], MajorSeventh),
        case(vec![C, E, G, B, D], MajorNinth),
        case(vec![C, E, G, ASharp], DominantSeventh),
        case(vec![C, F, G], SuspendedFourth),
        case(vec![C, D, G], SuspendedSecond),
        case(vec![C, DSharp, G], Minor),
        case(vec![C, DSharp, G, ASharp], MinorSeventh),
        case(vec![C, DSharp, G, B], MinorMajorSeventh),
        case(vec![C, DSharp, FSharp], Diminished),
        case(vec![C, DSharp, FSharp, A], DiminishedSeventh),
        case(vec![C, DSharp, FSharp, ASharp], HalfDiminishedSeventh),
        case(vec![C, E, GSharp], Augmented),
        case(vec![C, E, GSharp, ASharp], AugmentedSeventh),
        case(vec![C, E, GSharp, B], AugmentedMajorSeventh),
        // Test some chords with other root notes.
        case(vec![D, FSharp, A], Major),
        case(vec![D, F, A], Minor),
        case(vec![D, FSharp, A, C], DominantSeventh),
        case(vec![G, B, D], Major),
        // Test pitch class list in different order.
        case(vec![C, G, E], Major),
    )]
    fn test_get_chord_type(pitches: Vec<PitchClass>, chord_type: ChordType) {
        assert_eq!(ChordType::try_from(&pitches[..]).unwrap(), chord_type);
    }
}
