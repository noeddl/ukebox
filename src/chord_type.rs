use std::cmp::min;
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use crate::{Interval, PitchClass, PITCH_CLASS_COUNT, STRING_COUNT};

/// The type of the chord depending on the intervals it contains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChordType {
    Major,
    MajorSeventh,
    MajorNinth,
    MajorThirteenth,
    MajorSixth,
    SixthNinth,
    DominantSeventh,
    DominantNinth,
    DominantThirteenth,
    SuspendedFourth,
    SuspendedSecond,
    DominantSeventhSuspendedFourth,
    DominantSeventhSuspendedSecond,
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
            MajorSixth,
            SixthNinth,
            DominantSeventh,
            DominantNinth,
            DominantThirteenth,
            SuspendedFourth,
            SuspendedSecond,
            DominantSeventhSuspendedFourth,
            DominantSeventhSuspendedSecond,
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
            MajorSixth => vec!["P1", "M3", "P5", "M6"],
            SixthNinth => vec!["P1", "M3", "P5", "M6", "M9"],
            DominantSeventh => vec!["P1", "M3", "P5", "m7"],
            DominantNinth => vec!["P1", "M3", "P5", "m7", "M9"],
            DominantThirteenth => vec!["P1", "M3", "P5", "m7", "M9", "P11", "M13"],
            SuspendedFourth => vec!["P1", "P4", "P5"],
            SuspendedSecond => vec!["P1", "M2", "P5"],
            DominantSeventhSuspendedFourth => vec!["P1", "P4", "P5", "m7"],
            DominantSeventhSuspendedSecond => vec!["P1", "M2", "P5", "m7"],
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
            MajorSeventh
            | MajorNinth
            | MajorSixth
            | SixthNinth
            | DominantSeventh
            | DominantNinth
            | SuspendedFourth
            | SuspendedSecond
            | DominantSeventhSuspendedFourth
            | DominantSeventhSuspendedSecond
            | MinorSeventh
            | MinorMajorSeventh => vec!["P5"],
            MajorThirteenth | DominantThirteenth => vec!["P5", "M9", "P11"],
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

    pub fn to_symbol(self) -> String {
        use ChordType::*;

        let s = match self {
            Major => "",
            MajorSeventh => "maj7",
            MajorNinth => "maj9",
            MajorThirteenth => "maj13",
            MajorSixth => "6",
            SixthNinth => "6/9",
            DominantSeventh => "7",
            DominantNinth => "9",
            DominantThirteenth => "13",
            SuspendedFourth => "sus4",
            SuspendedSecond => "sus2",
            DominantSeventhSuspendedFourth => "7sus4",
            DominantSeventhSuspendedSecond => "7sus2",
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
            MajorSixth => "major 6th",
            SixthNinth => "sixth/ninth",
            DominantSeventh => "dominant 7th",
            DominantNinth => "dominant 9th",
            DominantThirteenth => "dominant 13th",
            SuspendedFourth => "suspended 4th",
            SuspendedSecond => "suspended 2nd",
            DominantSeventhSuspendedFourth => "dominant 7th suspended 4th",
            DominantSeventhSuspendedSecond => "dominant 7th suspended 2nd",
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
            "6" => Ok(MajorSixth),
            "6/9" => Ok(SixthNinth),
            "7" => Ok(DominantSeventh),
            "9" => Ok(DominantNinth),
            "13" => Ok(DominantThirteenth),
            "sus4" => Ok(SuspendedFourth),
            "sus2" => Ok(SuspendedSecond),
            "7sus4" => Ok(DominantSeventhSuspendedFourth),
            "7sus2" => Ok(DominantSeventhSuspendedSecond),
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

        let to_semitones = |i: Interval| {
            let s = i.to_semitones();
            if s >= PITCH_CLASS_COUNT {
                s - PITCH_CLASS_COUNT
            } else {
                s
            }
        };

        for chord_type in ChordType::values() {
            // We need at least all the required intervals to determine the chord type.
            // But if we can fit more notes because we have enough strings, we should do it.
            let min_len = min(chord_type.intervals().count(), STRING_COUNT);

            if pitch_diffs.len() < min_len {
                continue;
            }

            let req_sems: Vec<_> = chord_type.required_intervals().map(to_semitones).collect();

            let req = pitch_diffs.iter().filter(|s| req_sems.contains(s));

            if req.count() != req_sems.len() {
                continue;
            }

            let opt_sems: Vec<_> = chord_type.optional_intervals().map(to_semitones).collect();

            let mut opt = pitch_diffs.iter().filter(|s| !req_sems.contains(s));

            if opt.all(|s| opt_sems.contains(s)) {
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
        case(vec![C, E, B, D], MajorNinth),
        case(vec![C, E, G, B, D, F, A], MajorThirteenth),
        case(vec![C, E, B, A], MajorThirteenth),
        case(vec![C, E, G, A], MajorSixth),
        case(vec![C, E, G, A, D], SixthNinth),
        case(vec![C, E, A, D], SixthNinth),
        case(vec![C, E, G, ASharp], DominantSeventh),
        case(vec![C, E, G, ASharp, D], DominantNinth),
        case(vec![C, E, ASharp, D], DominantNinth),
        case(vec![C, E, G, ASharp, D, A], DominantThirteenth),
        case(vec![C, E, ASharp, A], DominantThirteenth),
        case(vec![C, F, G], SuspendedFourth),
        case(vec![C, D, G], SuspendedSecond),
        case(vec![C, F, G, ASharp], DominantSeventhSuspendedFourth),
        case(vec![C, D, G, ASharp], DominantSeventhSuspendedSecond),
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
        pitches,
        case(vec![C, E]),
        case(vec![D]),
        case(vec![C, F, ASharp]), // missing fifth
    )]
    fn test_get_chord_type_error(pitches: Vec<PitchClass>) {
        assert!(ChordType::try_from(&pitches[..]).is_err());
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
