use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use itertools::Itertools;

use crate::{Interval, PitchClass, Semitones, PITCH_CLASS_COUNT};

/// The type of the chord depending on the intervals it contains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChordType {
    Major,
    MajorSeventh,
    MajorNinth,
    MajorThirteenth,
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
    /// Iterator over the values of the ChordType enum.
    ///
    /// Unfortunately, we have to list them all and make sure to update
    /// this list if a value is added or removed.
    pub fn values() -> impl Iterator<Item = ChordType> {
        use ChordType::*;

        [
            Major,
            MajorSeventh,
            MajorNinth,
            MajorThirteenth,
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
        ]
        .iter()
        .copied()
    }

    /// Return an iterator over the chord type's intervals.
    pub fn intervals(&self) -> impl Iterator<Item = Interval> + '_ {
        use ChordType::*;

        let interval_names = match self {
            Major => vec!["P1", "M3", "P5"],
            MajorSeventh => vec!["P1", "M3", "P5", "M7"],
            MajorNinth => vec!["P1", "M3", "P5", "M7", "M9"],
            MajorThirteenth => vec!["P1", "M3", "P5", "M7", "M9", "P11", "M13"],
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

    /// Return an iterator over the chord type's optional intervals.
    pub fn optional_intervals(&self) -> impl Iterator<Item = Interval> + '_ {
        use ChordType::*;

        let interval_names = match self {
            MajorSeventh | MajorNinth | DominantSeventh | SuspendedFourth | SuspendedSecond
            | MinorSeventh | MinorMajorSeventh => vec!["P5"],
            MajorThirteenth => vec!["P5", "M9", "P11"],
            _ => vec![],
        };

        interval_names
            .into_iter()
            .map(|s| Interval::from_str(s).unwrap())
    }

    /// Return an iterator over the chord type's required intervals.
    pub fn required_intervals(&self) -> impl Iterator<Item = Interval> + '_ {
        self.intervals()
            .filter(move |&i1| self.optional_intervals().all(|i2| i2 != i1))
    }

    pub fn semitones(&self) -> impl Iterator<Item = Semitones> + '_ {
        self.intervals().map(|i| i.to_semitones()).map(|s| {
            if s >= PITCH_CLASS_COUNT {
                s - PITCH_CLASS_COUNT
            } else {
                s
            }
        })
    }

    pub fn to_symbol(self) -> String {
        use ChordType::*;

        let s = match self {
            Major => "",
            MajorSeventh => "maj7",
            MajorNinth => "maj9",
            MajorThirteenth => "maj13",
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
            MajorThirteenth => "major 13th",
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
            "maj13" => Ok(MajorThirteenth),
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
        // Subtract the root note's pitch class from all pitch classes to get the
        // difference in semitones.
        let mut pitch_diffs: Vec<_> = pitches.iter().map(|pc| *pc - pitches[0]).collect();

        pitch_diffs.sort_unstable();

        for chord_type in ChordType::values() {
            if pitch_diffs
                .iter()
                .cloned()
                .eq(chord_type.semitones().sorted())
            {
                return Ok(chord_type);
            }
        }

        Err("No matching chord type found.")
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
        case(vec![C, E, G, B, D, F, A], MajorThirteenth),
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

    #[rstest(
        chord_type, intervals,
        case(Major, vec!["P1", "M3", "P5"]),
        case(MajorSeventh, vec!["P1", "M3", "M7"]),
    )]
    fn test_required_intervals(chord_type: ChordType, intervals: Vec<&str>) {
        let req_ints: Vec<_> = chord_type.required_intervals().collect();

        let exp_ints: Vec<_> = intervals
            .iter()
            .map(|s| Interval::from_str(s).unwrap())
            .collect();

        assert_eq!(req_ints, exp_ints);
    }
}
