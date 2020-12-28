use crate::STRING_COUNT;
use crate::chord::{Chord, FretID, Tuning};
use crate::note::{PitchClass, Semitones};
use std::convert::{TryFrom, TryInto};
use std::ops::{Add, Index};
use std::slice::Iter;
use std::str::FromStr;

/// A pattern of frets to press down for playing a chord.
/// Each index of the array corresponds to a ukulele string.
#[derive(Debug, Copy, Clone)]
pub struct FretPattern {
    frets: [FretID; STRING_COUNT],
}

impl FretPattern {
    pub fn iter(&self) -> Iter<'_, FretID> {
        self.frets.iter()
    }

    pub fn get_min_fret(&self) -> FretID {
        *self.iter().min().unwrap()
    }

    pub fn get_max_fret(&self) -> FretID {
        *self.iter().max().unwrap()
    }

    pub fn get_pitch_classes(&self, tuning: Tuning) -> Vec<PitchClass> {
        let roots = tuning.get_roots();
        let pitches: Vec<_> = self.iter().zip(roots.iter()).map(|(fret, note)| note.pitch_class + *fret).collect();

        pitches
    }

    pub fn get_chords(&self, tuning: Tuning) -> Vec<Chord> {
        let mut chords = vec![];
        let mut pitches = self.get_pitch_classes(tuning);

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

        chords
    }
}

impl From<[FretID; STRING_COUNT]> for FretPattern {
    fn from(frets: [FretID; STRING_COUNT]) -> Self {
        Self {
            frets
        }
    }
}

impl From<Vec<FretID>> for FretPattern {
    fn from(mut fret_vec: Vec<FretID>) -> Self {
        // Make sure the vector has the correct length. If it is too short,
        // extend by the difference and fill with zeros.
        // For example, [1, 2] will be extended to [1, 2, 0, 0].
        fret_vec.resize(STRING_COUNT, 0);

        Self {
            frets: fret_vec.try_into().unwrap()
        }
    }
}

impl FromStr for FretPattern {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Handle both patterns containing spaces such as "1 2 3 4" as well as patterns
        // without spaces such as "1234".
        let split: Vec<String> = match s.contains(" ") {
            true => s.split(" ").map(|c| c.to_string()).collect(),
            false => s.chars().map(|c| c.to_string()).collect()
        };

        let fret_res: Result<Vec<FretID>, _> = split.iter().map(|s| s.parse()).collect();

        match fret_res {
            Ok(fret_vec) => Ok(Self::from(fret_vec)),
            _ => Err("Fret pattern has wrong format")
        }
    }
}

impl Index<usize> for FretPattern {
    type Output = FretID;

    fn index(&self, i: usize) -> &Self::Output {
        &self.frets[i]
    }
}

impl Add<Semitones> for FretPattern {
    type Output = Self;

    fn add(self, n: Semitones) -> Self {
        let mut frets = self.frets;

        for f in &mut frets[..] {
            *f += n;
        }

        Self{
            frets
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use Tuning;

    #[rstest(
        s, frets,
        case("2220", [2, 2, 2, 0]),
        case("22201", [2, 2, 2, 0]),
        case("222", [2, 2, 2, 0]),
        case("", [0, 0, 0, 0]),
    )]
    fn test_from_str(s: &str, frets: [FretID; STRING_COUNT]) {
        let fret_pattern = FretPattern::from_str(s).unwrap();
        assert_eq!(fret_pattern.frets, frets);
    }

    #[rstest(
        s,
        case("Cm"),
    )]
    fn test_from_str_fail(s: &str) {
        assert!(FretPattern::from_str(s).is_err())
    }

    #[rstest(
        frets, chord_str, tuning,
        case([0, 0, 0, 3], "C", Tuning::C),
        case([0, 0, 0, 3], "D", Tuning::D),
        case([2, 2, 2, 0], "D", Tuning::C),
    )]
    fn test_get_chords(frets: [FretID; STRING_COUNT], chord_str: &str, tuning: Tuning) {
        let fret_pattern = FretPattern::from(frets);
        let chords = fret_pattern.get_chords(tuning);
        let chord = Chord::from_str(chord_str).unwrap();
        assert_eq!(chords, vec![chord]);
    }

    #[rstest(
        frets,
        case([1, 2, 3, 4]),
    )]
    fn test_get_chords_fail(frets: [FretID; STRING_COUNT]) {
        let fret_pattern = FretPattern::from(frets);
        assert!(fret_pattern.get_chords(Tuning::C).is_empty());
    }
}
