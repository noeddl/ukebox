use std::cmp::Ordering;
use std::convert::{TryFrom, TryInto};
use std::slice::Iter;

use itertools::Itertools;

use crate::{Chord, FretID, FretPattern, Note, PitchClass, Tuning, UkeString, STRING_COUNT};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Voicing {
    uke_strings: [UkeString; STRING_COUNT],
}

/// The voicing of a chord describes the order of the individual notes within
/// the chord. The same chord can be voiced in different ways, i.e. there are
/// several ways to play the same chord on the ukulele.
/// https://en.wikipedia.org/wiki/Voicing_(music)
impl Voicing {
    // Create a Voicing instance from a set of frets and a tuning.
    // As there is no information about a certain chord for which
    // the voicing is created, the computed `note`s in the resulting
    // `UkeString`s will by default be sharp (for notes that can be sharp
    // or flat).
    pub fn new(fret_pattern: impl Into<FretPattern>, tuning: Tuning) -> Self {
        let fret_pattern = fret_pattern.into();
        let roots = tuning.get_roots();

        let uke_strings: Vec<UkeString> = roots
            .iter()
            .zip(fret_pattern.iter())
            .map(|(&root, &fret)| (root, fret, root + fret))
            .collect();

        Self {
            uke_strings: uke_strings.try_into().unwrap(),
        }
    }

    pub fn uke_strings(&self) -> Iter<'_, UkeString> {
        self.uke_strings.iter()
    }

    pub fn roots(&self) -> impl Iterator<Item = Note> + '_ {
        self.uke_strings.iter().map(|(r, _f, _n)| *r)
    }

    pub fn frets(&self) -> impl Iterator<Item = FretID> + '_ {
        self.uke_strings.iter().map(|(_r, f, _n)| *f)
    }

    pub fn notes(&self) -> impl Iterator<Item = Note> + '_ {
        self.uke_strings.iter().map(|(_r, _f, n)| *n)
    }

    /// Return the lowest fret at which a string is pressed down.
    pub fn get_min_fret(&self) -> FretID {
        match self.frets().filter(|&x| x > 0).min() {
            Some(x) => x,
            // Special case [0, 0, 0, 0]: no string is pressed down.
            _ => 0,
        }
    }

    pub fn get_max_fret(&self) -> FretID {
        self.frets().max().unwrap()
    }

    pub fn get_span(&self) -> FretID {
        self.get_max_fret() - self.get_min_fret()
    }

    /// Return `true` if the voicing contains all the notes needed
    /// to play the given `chord`.
    pub fn spells_out(&self, chord: &Chord) -> bool {
        self.notes()
            .sorted()
            .dedup()
            .eq(chord.notes().sorted().dedup())
    }

    pub fn get_chords(&self) -> Vec<Chord> {
        let mut chords = vec![];

        let mut pitches: Vec<PitchClass> = self.notes().map(|n| n.pitch_class).collect();
        pitches.sort();
        pitches.dedup();

        // Rotate pitch class list and collect all matching chords.
        // For example, try [C, DSharp, GSharp], [DSharp, GSharp, C], [GSharp, C, FSharp].
        for _ in 0..pitches.len() {
            if let Ok(chord) = Chord::try_from(&pitches[..]) {
                chords.push(chord);
            }

            pitches.rotate_left(1);
        }

        chords.sort();
        chords
    }
}

impl PartialOrd for Voicing {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Voicing {
    fn cmp(&self, other: &Self) -> Ordering {
        // In order to iterate over frets in reversed order, we could implement
        // DoubleEndedIterator ... or use this hack.
        let frets1: Vec<FretID> = self.frets().collect();
        let frets2: Vec<FretID> = other.frets().collect();

        match self.get_min_fret().cmp(&other.get_min_fret()) {
            Ordering::Equal => frets1.iter().rev().cmp(frets2.iter().rev()),
            other => other,
        }
    }
}

impl From<&[UkeString]> for Voicing {
    fn from(uke_strings: &[UkeString]) -> Self {
        Self {
            // Let's assume that all the Vecs coming in here have the correct size.
            uke_strings: uke_strings.try_into().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rstest::rstest;

    use super::*;

    #[rstest(
        frets1, frets2,
        case([0, 0, 0, 0], [0, 0, 0, 1]),
        case([0, 3, 3, 3], [0, 0, 3, 6]),
        case([0, 0, 8, 6], [0, 7, 8, 6]),
    )]
    fn test_compare(frets1: [FretID; STRING_COUNT], frets2: [FretID; STRING_COUNT]) {
        let voicing1 = Voicing::new(frets1, Tuning::C);
        let voicing2 = Voicing::new(frets2, Tuning::C);
        assert!(voicing1 < voicing2);
    }

    #[rstest(
        frets, min_fret, max_fret, span,
        case([0, 0, 0, 0], 0, 0, 0),
        case([1, 1, 1, 1], 1, 1, 0),
        case([2, 0, 1, 3], 1, 3, 2),
        case([5, 5, 5, 6], 5, 6, 1),
        case([3, 0, 0, 12], 3, 12, 9),
    )]
    fn test_get_min_max_fret_and_span(
        frets: [FretID; STRING_COUNT],
        min_fret: FretID,
        max_fret: FretID,
        span: u8,
    ) {
        let voicing = Voicing::new(frets, Tuning::C);
        assert_eq!(voicing.get_min_fret(), min_fret);
        assert_eq!(voicing.get_max_fret(), max_fret);
        assert_eq!(voicing.get_span(), span);
    }

    #[rstest(
        frets, chord_str, spells_out,
        case([0, 0, 0, 3], "C", true), // G C E C
        case([5, 4, 3, 3], "C", true), // C E G C
        case([0, 2, 0, 3], "C", false), // G D E C
        case([0, 0, 3, 0], "C", false), // G C G C
        case([0, 3, 3, 3], "Cm", true), // G Eb G C
        case([1, 1, 1, 4], "C#", true), // G# C# F C#
    )]
    fn test_spells_out(frets: [FretID; STRING_COUNT], chord_str: &str, spells_out: bool) {
        let voicing = Voicing::new(frets, Tuning::C);
        let chord = Chord::from_str(chord_str).unwrap();
        assert_eq!(voicing.spells_out(&chord), spells_out);
    }

    #[rstest(
        frets, chord_str, tuning,
        case([0, 0, 0, 3], "C", Tuning::C),
        case([0, 0, 0, 3], "D", Tuning::D),
        case([2, 2, 2, 0], "D", Tuning::C),
    )]
    fn test_get_chords(frets: [FretID; STRING_COUNT], chord_str: &str, tuning: Tuning) {
        let voicing = Voicing::new(frets, tuning);
        let chords = voicing.get_chords();
        let chord = Chord::from_str(chord_str).unwrap();
        assert_eq!(chords, vec![chord]);
    }

    #[rstest(
        frets,
        case([1, 2, 3, 4]),
    )]
    fn test_get_chords_fail(frets: [FretID; STRING_COUNT]) {
        let voicing = Voicing::new(frets, Tuning::C);
        assert!(voicing.get_chords().is_empty());
    }
}
