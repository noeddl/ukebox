use std::cmp::min;
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use crate::{Interval, PitchClass, PITCH_CLASS_COUNT, STRING_COUNT};

/// The type of the chord depending on the intervals it contains.
///
/// Sources used:
///
/// * <https://en.wikipedia.org/wiki/Chord_(music)>
/// * <https://github.com/hyvyys/chord-fingering/blob/master/src/CHORD_DATA.js>
/// * <https://en.wikibooks.org/wiki/Music_Theory/Complete_List_of_Chord_Patterns>
/// * <http://www.hakwright.co.uk/music/quick_crd_ref.html>
/// * <https://chords.gock.net>
/// * <https://ukulele-chords.com>
/// * <https://ukulelehelper.com>
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChordType {
    Major,
    MajorSeventh,
    MajorNinth,
    MajorEleventh,
    MajorThirteenth,
    MajorSixth,
    SixthNinth,
    DominantSeventh,
    DominantNinth,
    /// According to Wikipedia, the dominant eleventh chord is often played as a suspended
    /// chord without the third. But as this overlaps with the dominant seventh suspended
    /// fourth chord and other sources list the dominant eleventh including the third,
    /// let's also keep it around here. (See https://en.wikipedia.org/wiki/Eleventh_chord)
    DominantEleventh,
    DominantThirteenth,
    DominantSeventhFlatNinth,
    DominantSeventhSharpNinth,
    DominantSeventhFlatFifth,
    //DominantSeventhSharpFifth,
    SuspendedFourth,
    SuspendedSecond,
    DominantSeventhSuspendedFourth,
    DominantSeventhSuspendedSecond,
    Minor,
    MinorSeventh,
    MinorMajorSeventh,
    MinorSixth,
    MinorNinth,
    MinorEleventh,
    MinorThirteenth,
    Diminished,
    DiminishedSeventh,
    HalfDiminishedSeventh,
    Fifth,
    Augmented,
    AugmentedSeventh,
    AugmentedMajorSeventh,
    AddedNinth,
    AddedFourth,
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
            MajorEleventh,
            MajorThirteenth,
            MajorSixth,
            SixthNinth,
            DominantSeventh,
            DominantNinth,
            DominantEleventh,
            DominantThirteenth,
            DominantSeventhFlatNinth,
            DominantSeventhSharpNinth,
            DominantSeventhFlatFifth,
            //DominantSeventhSharpFifth,
            SuspendedFourth,
            SuspendedSecond,
            DominantSeventhSuspendedFourth,
            DominantSeventhSuspendedSecond,
            Minor,
            MinorSeventh,
            MinorMajorSeventh,
            MinorSixth,
            MinorNinth,
            MinorEleventh,
            MinorThirteenth,
            Diminished,
            DiminishedSeventh,
            HalfDiminishedSeventh,
            Fifth,
            Augmented,
            AugmentedSeventh,
            AugmentedMajorSeventh,
            AddedNinth,
            AddedFourth,
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
            MajorEleventh => vec!["P1", "M3", "P5", "M7", "M9", "P11"],
            MajorThirteenth => vec!["P1", "M3", "P5", "M7", "M9", "P11", "M13"],
            MajorSixth => vec!["P1", "M3", "P5", "M6"],
            SixthNinth => vec!["P1", "M3", "P5", "M6", "M9"],
            DominantSeventh => vec!["P1", "M3", "P5", "m7"],
            DominantNinth => vec!["P1", "M3", "P5", "m7", "M9"],
            DominantEleventh => vec!["P1", "M3", "P5", "m7", "M9", "P11"],
            DominantThirteenth => vec!["P1", "M3", "P5", "m7", "M9", "P11", "M13"],
            DominantSeventhFlatNinth => vec!["P1", "M3", "P5", "m7", "m9"],
            DominantSeventhSharpNinth => vec!["P1", "M3", "P5", "m7", "A9"],
            DominantSeventhFlatFifth => vec!["P1", "M3", "d5", "m7"],
            //DominantSeventhSharpFifth => vec!["P1", "M3", "A5", "m7"],
            SuspendedFourth => vec!["P1", "P4", "P5"],
            SuspendedSecond => vec!["P1", "M2", "P5"],
            DominantSeventhSuspendedFourth => vec!["P1", "P4", "P5", "m7"],
            DominantSeventhSuspendedSecond => vec!["P1", "M2", "P5", "m7"],
            Minor => vec!["P1", "m3", "P5"],
            MinorSeventh => vec!["P1", "m3", "P5", "m7"],
            MinorMajorSeventh => vec!["P1", "m3", "P5", "M7"],
            MinorSixth => vec!["P1", "m3", "P5", "M6"],
            MinorNinth => vec!["P1", "m3", "P5", "m7", "M9"],
            MinorEleventh => vec!["P1", "m3", "P5", "m7", "M9", "P11"],
            MinorThirteenth => vec!["P1", "m3", "P5", "m7", "M9", "P11", "M13"],
            Diminished => vec!["P1", "m3", "d5"],
            DiminishedSeventh => vec!["P1", "m3", "d5", "d7"],
            HalfDiminishedSeventh => vec!["P1", "m3", "d5", "m7"],
            Fifth => vec!["P1", "P5"],
            Augmented => vec!["P1", "M3", "A5"],
            AugmentedSeventh => vec!["P1", "M3", "A5", "m7"],
            AugmentedMajorSeventh => vec!["P1", "M3", "A5", "M7"],
            AddedNinth => vec!["P1", "M3", "P5", "M9"],
            AddedFourth => vec!["P1", "M3", "P4", "P5"],
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
            | DominantSeventhFlatNinth
            | DominantSeventhSharpNinth
            | SuspendedFourth
            | SuspendedSecond
            | DominantSeventhSuspendedFourth
            | DominantSeventhSuspendedSecond
            | MinorSeventh
            | MinorMajorSeventh
            | MinorSixth
            | MinorNinth
            | AddedNinth
            | AddedFourth => vec!["P5"],
            MajorEleventh | DominantEleventh | MinorEleventh => vec!["P5", "M9"],
            MajorThirteenth | DominantThirteenth | MinorThirteenth => vec!["P5", "M9", "P11"],
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

    pub fn symbols(self) -> Vec<String> {
        use ChordType::*;

        let symbols = match self {
            Major => &[""],
            MajorSeventh => &["maj7"],
            MajorNinth => &["maj9"],
            MajorEleventh => &["maj11"],
            MajorThirteenth => &["maj13"],
            MajorSixth => &["6"],
            SixthNinth => &["6/9"],
            DominantSeventh => &["7"],
            DominantNinth => &["9"],
            DominantEleventh => &["11"],
            DominantThirteenth => &["13"],
            DominantSeventhFlatNinth => &["7b9"],
            DominantSeventhSharpNinth => &["7#9"],
            DominantSeventhFlatFifth => &["7b5"],
            //DominantSeventhSharpFifth => "7#5",
            SuspendedFourth => &["sus4"],
            SuspendedSecond => &["sus2"],
            DominantSeventhSuspendedFourth => &["7sus4"],
            DominantSeventhSuspendedSecond => &["7sus2"],
            Minor => &["m"],
            MinorSeventh => &["m7"],
            MinorMajorSeventh => &["mMaj7"],
            MinorSixth => &["m6"],
            MinorNinth => &["m9"],
            MinorEleventh => &["m11"],
            MinorThirteenth => &["m13"],
            Diminished => &["dim"],
            DiminishedSeventh => &["dim7"],
            HalfDiminishedSeventh => &["m7b5"],
            Fifth => &["5"],
            Augmented => &["aug"],
            AugmentedSeventh => &["aug7"],
            AugmentedMajorSeventh => &["augMaj7"],
            AddedNinth => &["add9"],
            AddedFourth => &["add4"],
        };

        symbols.iter().map(|s| s.to_string()).collect()
    }

    pub fn to_symbol(self) -> String {
        self.symbols()[0].clone()
    }
}

impl fmt::Display for ChordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ChordType::*;

        let s = match self {
            Major => "major",
            MajorSeventh => "major 7th",
            MajorNinth => "major 9th",
            MajorEleventh => "major 11th",
            MajorThirteenth => "major 13th",
            MajorSixth => "major 6th",
            SixthNinth => "6th/9th",
            DominantSeventh => "dominant 7th",
            DominantNinth => "dominant 9th",
            DominantEleventh => "dominant 11th",
            DominantThirteenth => "dominant 13th",
            DominantSeventhFlatNinth => "dominant 7th flat 9th",
            DominantSeventhSharpNinth => "dominant 7th sharp 9th",
            DominantSeventhFlatFifth => "dominant 7th flat 5th",
            //DominantSeventhSharpFifth => "dominant 7th sharp 5th",
            SuspendedFourth => "suspended 4th",
            SuspendedSecond => "suspended 2nd",
            DominantSeventhSuspendedFourth => "dominant 7th suspended 4th",
            DominantSeventhSuspendedSecond => "dominant 7th suspended 2nd",
            Minor => "minor",
            MinorSeventh => "minor 7th",
            MinorMajorSeventh => "minor/major 7th",
            MinorSixth => "minor 6th",
            MinorNinth => "minor 9th",
            MinorEleventh => "minor 11th",
            MinorThirteenth => "minor 13th",
            Diminished => "diminished",
            DiminishedSeventh => "diminished 7th",
            HalfDiminishedSeventh => "half-diminished 7th",
            Fifth => "5th",
            Augmented => "augmented",
            AugmentedSeventh => "augmented 7th",
            AugmentedMajorSeventh => "augmented major 7th",
            AddedNinth => "added 9th",
            AddedFourth => "added 4th",
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
            "maj11" => Ok(MajorEleventh),
            "maj13" => Ok(MajorThirteenth),
            "6" => Ok(MajorSixth),
            "6/9" => Ok(SixthNinth),
            "7" => Ok(DominantSeventh),
            "9" => Ok(DominantNinth),
            "11" => Ok(DominantEleventh),
            "13" => Ok(DominantThirteenth),
            "7b9" => Ok(DominantSeventhFlatNinth),
            "7#9" => Ok(DominantSeventhSharpNinth),
            "7b5" => Ok(DominantSeventhFlatFifth),
            //"7#5" => Ok(DominantSeventhSharpFifth),
            "sus4" => Ok(SuspendedFourth),
            "sus2" => Ok(SuspendedSecond),
            "7sus4" => Ok(DominantSeventhSuspendedFourth),
            "7sus2" => Ok(DominantSeventhSuspendedSecond),
            "m" => Ok(Minor),
            "m7" => Ok(MinorSeventh),
            "mMaj7" => Ok(MinorMajorSeventh),
            "m6" => Ok(MinorSixth),
            "m9" => Ok(MinorNinth),
            "m11" => Ok(MinorEleventh),
            "m13" => Ok(MinorThirteenth),
            "dim" => Ok(Diminished),
            "dim7" => Ok(DiminishedSeventh),
            "m7b5" => Ok(HalfDiminishedSeventh),
            "5" => Ok(Fifth),
            "aug" => Ok(Augmented),
            "aug7" => Ok(AugmentedSeventh),
            "7#5" => Ok(AugmentedSeventh),
            "augMaj7" => Ok(AugmentedMajorSeventh),
            "add9" => Ok(AddedNinth),
            "add2" => Ok(AddedNinth),
            "add4" => Ok(AddedFourth),
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
            // If a chord has less required intervals than we have strings, add optional intervals
            // until all strings are used.
            let min_len = min(chord_type.intervals().count(), STRING_COUNT);

            if pitch_diffs.len() < min_len {
                continue;
            }

            // All the required intervals need to be there.
            let req_sems: Vec<_> = chord_type.required_intervals().map(to_semitones).collect();

            let req = pitch_diffs.iter().filter(|s| req_sems.contains(s));

            if req.count() != req_sems.len() {
                continue;
            }

            // The remaining semitones must all correspond to optional intervals from the chord.
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
        case(vec![C, E, G, B, D, F], MajorEleventh),
        case(vec![C, E, B, F], MajorEleventh),
        case(vec![C, E, G, B, D, F, A], MajorThirteenth),
        case(vec![C, E, B, A], MajorThirteenth),
        case(vec![C, E, G, A], MajorSixth),
        case(vec![C, E, G, A, D], SixthNinth),
        case(vec![C, E, A, D], SixthNinth),
        case(vec![C, E, G, ASharp], DominantSeventh),
        case(vec![C, E, G, ASharp, D], DominantNinth),
        case(vec![C, E, ASharp, D], DominantNinth),
        case(vec![C, E, G, ASharp, D, F], DominantEleventh),
        case(vec![C, E, ASharp, F], DominantEleventh),
        case(vec![C, E, G, ASharp, D, F, A], DominantThirteenth),
        case(vec![C, E, ASharp, D, A], DominantThirteenth),
        case(vec![C, E, ASharp, A], DominantThirteenth),
        case(vec![C, E, G, ASharp, CSharp], DominantSeventhFlatNinth),
        case(vec![C, E, ASharp, CSharp], DominantSeventhFlatNinth),
        case(vec![C, E, G, ASharp, DSharp], DominantSeventhSharpNinth),
        case(vec![C, E, ASharp, DSharp], DominantSeventhSharpNinth),
        case(vec![C, E, FSharp, ASharp], DominantSeventhFlatFifth),
        //case(vec![C, E, GSharp, ASharp], DominantSeventhSharpFifth),
        case(vec![C, F, G], SuspendedFourth),
        case(vec![C, D, G], SuspendedSecond),
        case(vec![C, F, G, ASharp], DominantSeventhSuspendedFourth),
        case(vec![C, D, G, ASharp], DominantSeventhSuspendedSecond),
        case(vec![C, DSharp, G], Minor),
        case(vec![C, DSharp, G, ASharp], MinorSeventh),
        case(vec![C, DSharp, G, B], MinorMajorSeventh),
        case(vec![C, DSharp, G, A], MinorSixth),
        case(vec![C, DSharp, G, ASharp, D], MinorNinth),
        case(vec![C, DSharp, ASharp, D], MinorNinth),
        case(vec![C, DSharp, G, ASharp, D, F], MinorEleventh),
        case(vec![C, DSharp, D, ASharp, F], MinorEleventh),
        case(vec![C, DSharp, G, ASharp, D, F, A], MinorThirteenth),
        case(vec![C, DSharp, D, ASharp, A], MinorThirteenth),
        case(vec![C, DSharp, FSharp], Diminished),
        case(vec![C, DSharp, FSharp, A], DiminishedSeventh),
        case(vec![C, DSharp, FSharp, ASharp], HalfDiminishedSeventh),
        case(vec![C, G], Fifth),
        case(vec![C, E, GSharp], Augmented),
        case(vec![C, E, GSharp, ASharp], AugmentedSeventh),
        case(vec![C, E, GSharp, B], AugmentedMajorSeventh),
        case(vec![C, E, G, D], AddedNinth),
        case(vec![C, D, E, G], AddedNinth),
        case(vec![C, E, F, G], AddedFourth),
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
