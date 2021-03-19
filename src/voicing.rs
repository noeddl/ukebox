use std::slice::Iter;

use crate::{Chord, FretID, Note, Semitones, UkeString, STRING_COUNT};

#[derive(Debug, Clone, Copy)]
pub struct Voicing {
    uke_strings: [UkeString; STRING_COUNT],
    max_span: Semitones,
}

impl Voicing {
    pub fn new(uke_strings: [UkeString; STRING_COUNT], max_span: Semitones) -> Self {
        Self {
            uke_strings,
            max_span,
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

    /// Return `true` if the diagram is a valid depiction of how to
    /// play the given `chord`.
    pub fn depicts(&self, chord: &Chord) -> bool {
        let notes: Vec<Note> = self.notes().collect();
        chord.consists_of(&notes)
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
}
