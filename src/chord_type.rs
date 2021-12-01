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
/// * <https://en.wikipedia.org/wiki/Chord_names_and_symbols_(popular_music)>
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

    /// Return an iterator over the symbols that can be used to denote a chord type.
    pub fn symbols(self) -> impl Iterator<Item = &'static str> + 'static {
        use ChordType::*;

        let symbols = match self {
            Major => vec!["", "maj", "M"],
            MajorSeventh => vec!["maj7", "M7"],
            MajorNinth => vec!["maj9", "M9"],
            MajorEleventh => vec!["maj11", "M11"],
            MajorThirteenth => vec!["maj13", "M13"],
            MajorSixth => vec!["6", "maj6", "M6"],
            SixthNinth => vec!["6/9", "maj6/9", "M6/9"],
            DominantSeventh => vec!["7", "dom"],
            DominantNinth => vec!["9"],
            DominantEleventh => vec!["11"],
            DominantThirteenth => vec!["13"],
            DominantSeventhFlatNinth => vec!["7b9"],
            DominantSeventhSharpNinth => vec!["7#9"],
            DominantSeventhFlatFifth => vec!["7b5", "7dim5"],
            //DominantSeventhSharpFifth => "7#5",
            SuspendedFourth => vec!["sus4", "sus"],
            SuspendedSecond => vec!["sus2"],
            DominantSeventhSuspendedFourth => vec!["7sus4", "7sus"],
            DominantSeventhSuspendedSecond => vec!["7sus2"],
            Minor => vec!["m", "min"],
            MinorSeventh => vec!["m7", "min7"],
            MinorMajorSeventh => vec!["mMaj7", "mM7", "minMaj7"],
            MinorSixth => vec!["m6", "min6"],
            MinorNinth => vec!["m9", "min9"],
            MinorEleventh => vec!["m11", "min11"],
            MinorThirteenth => vec!["m13", "min13"],
            Diminished => vec!["dim", "o"],
            DiminishedSeventh => vec!["dim7", "o7"],
            HalfDiminishedSeventh => vec!["m7b5", "ø", "ø7"],
            Fifth => vec!["5"],
            Augmented => vec!["aug", "+"],
            AugmentedSeventh => vec!["aug7", "+7", "7#5"],
            AugmentedMajorSeventh => vec!["augMaj7", "+M7"],
            AddedNinth => vec!["add9", "add2"],
            AddedFourth => vec!["add4"],
        };

        symbols.into_iter()
    }

    pub fn to_symbol(self) -> String {
        self.symbols().next().unwrap().to_string()
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
        ChordType::values()
            .find(|ct| ct.symbols().any(|sym| sym == s))
            .ok_or("no valid chord type")
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
