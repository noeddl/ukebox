use crate::note::Interval;
use crate::note::PitchClass;
use std::convert::TryFrom;
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

impl TryFrom<&[PitchClass]> for ChordType {
    type Error = &'static str;

    /// Determine the chord type from a list of pitch classes representing a chord.
    fn try_from(pitches: &[PitchClass]) -> Result<Self, Self::Error> {
        use ChordType::*;

        // Subtract the root note's pitch class from all pitch classes to get the
        // difference in semitones.
        let mut pitch_diffs: Vec<_> = pitches.iter().map(|pc| *pc - pitches[0]).collect();

        pitch_diffs.sort();

        match pitch_diffs[..] {
            [0, 4, 7] => Ok(Major),
            [0, 3, 7] => Ok(Minor),
            [0, 2, 7] => Ok(SuspendedSecond),
            [0, 5, 7] => Ok(SuspendedFourth),
            [0, 4, 8] => Ok(Augmented),
            [0, 3, 6] => Ok(Diminished),
            [0, 4, 7, 10] => Ok(DominantSeventh),
            [0, 3, 7, 10] => Ok(MinorSeventh),
            [0, 4, 7, 11] => Ok(MajorSeventh),
            [0, 3, 7, 11] => Ok(MinorMajorSeventh),
            [0, 4, 8, 10] => Ok(AugmentedSeventh),
            [0, 4, 8, 11] => Ok(AugmentedMajorSeventh),
            [0, 3, 6, 9] => Ok(DiminishedSeventh),
            [0, 3, 6, 10] => Ok(HalfDiminishedSeventh),
            _ => Err("No matching chord type found."),
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::many_single_char_names)]
    use super::*;
    use rstest::rstest;
    use ChordType::*;
    use PitchClass::*;

    #[rstest(
        pitches, chord_type,
        // Test C-chords.
        case(vec![C, E, G], Major),
        case(vec![C, DSharp, G], Minor),
        case(vec![C, D, G], SuspendedSecond),
        case(vec![C, F, G], SuspendedFourth),
        case(vec![C, E, GSharp], Augmented),
        case(vec![C, DSharp, FSharp], Diminished),
        case(vec![C, E, G, ASharp], DominantSeventh),
        case(vec![C, DSharp, G, ASharp], MinorSeventh),
        case(vec![C, E, G, B], MajorSeventh),
        case(vec![C, DSharp, G, B], MinorMajorSeventh),
        case(vec![C, E, GSharp, ASharp], AugmentedSeventh),
        case(vec![C, E, GSharp, B], AugmentedMajorSeventh),
        case(vec![C, DSharp, FSharp, A], DiminishedSeventh),
        case(vec![C, DSharp, FSharp, ASharp], HalfDiminishedSeventh),
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
