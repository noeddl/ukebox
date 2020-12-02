use crate::STRING_COUNT;
use crate::chord::{FretID, Tuning};
use crate::note::{PitchClass, Semitones};
use std::convert::TryInto;
use std::ops::{Add, Index};
use std::slice::Iter;

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

    pub fn get_pitch_classes(&self, tuning: Tuning) -> [PitchClass; STRING_COUNT] {
        let roots = tuning.get_roots();
        let pitches: Vec<_> = self.iter().zip(roots.iter()).map(|(fret, note)| note.pitch_class + *fret).collect();

        pitches.try_into().unwrap()
    }
}

impl From<[FretID; STRING_COUNT]> for FretPattern {
    fn from(frets: [FretID; STRING_COUNT]) -> Self {
        Self {
            frets
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
