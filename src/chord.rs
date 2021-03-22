use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;

use itertools::Itertools;
use regex::Regex;

use crate::{ChordType, Note, PitchClass, Semitones, Tuning, UkeString, Voicing};

/// Custom error for strings that cannot be parsed into chords.
#[derive(Debug)]
pub struct ParseChordError {
    name: String,
}

impl Error for ParseChordError {}

impl fmt::Display for ParseChordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse chord name \"{}\"", self.name)
    }
}

/// A chord such as C, Cm and so on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Chord {
    pub root: Note,
    pub chord_type: ChordType,
}

impl Chord {
    pub fn new(root: Note, chord_type: ChordType) -> Self {
        Self { root, chord_type }
    }

    /// Return an iterator over the chord's notes.
    pub fn notes(&self) -> impl Iterator<Item = Note> + '_ {
        self.chord_type.intervals().map(move |i| self.root + i)
    }

    pub fn voicings(&self, tuning: Tuning) -> impl Iterator<Item = Voicing> + '_ {
        tuning
            .roots()
            // For each ukulele string, keep track of all the frets that when pressed down
            // while playing the string result in a note of the chord.
            .map(|root| {
                self.notes()
                    // Allow each note to be checked twice on the fretboard.
                    .cartesian_product(vec![0, 12])
                    // Determine the fret on which `note` is played.
                    .map(|(note, st)| (root, (note.pitch_class - root.pitch_class) + st, note))
                    .collect::<Vec<UkeString>>()
            })
            // At this point, we have collected all possible positions of the notes in the chord
            // on each ukulele string. Now let's check all combinations and determine the ones
            // that result in a valid voicing of the chord.
            .multi_cartesian_product()
            // Create voicing from the UkeString vec.
            .map(|us_vec| Voicing::from(&us_vec[..]))
            // Only keep valid voicings.
            .filter(|voicing| voicing.spells_out(self))
            .sorted()
    }
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = format!("{}{}", self.root, self.chord_type.to_symbol());
        write!(f, "{} - {} {}", name, self.root, self.chord_type)
    }
}

impl FromStr for Chord {
    type Err = ParseChordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = s.to_string();

        // Regular expression for chord names.
        let re = Regex::new(
            r"(?x)
                ^                               # match full string
                (?P<root>[CDEFGAB][\#b]?)       # root note including accidentals
                (?P<type>                       # chord type
                      sus(?:2|4)                # suspended chords
                    | aug(?:Maj)?7?             # augmented chords
                    | dim7?                     # diminished chords
                    | maj7                      # chords with a major 7th
                    | m?(?:(?:Maj)?7(?:b5)?)?)  # minor chords + alterations
                $                               # match full string
            ",
        )
        .unwrap();

        // Match regex.
        if let Some(caps) = re.captures(s) {
            // Get root note.
            if let Ok(root) = Note::from_str(&caps["root"]) {
                // Get chord type.
                if let Ok(chord_type) = ChordType::from_str(&caps["type"]) {
                    return Ok(Self::new(root, chord_type));
                };
            };
        };

        Err(ParseChordError { name })
    }
}

impl TryFrom<&[PitchClass]> for Chord {
    type Error = &'static str;

    /// Determine the chord that is represented by a list of pitch classes.
    fn try_from(pitches: &[PitchClass]) -> Result<Self, Self::Error> {
        let chord_type = ChordType::try_from(pitches)?;
        let root = Note::from(pitches[0]);

        Ok(Self::new(root, chord_type))
    }
}

impl Add<Semitones> for Chord {
    type Output = Self;

    fn add(self, n: Semitones) -> Self {
        Self::new(self.root + n, self.chord_type)
    }
}

impl Sub<Semitones> for Chord {
    type Output = Self;

    fn sub(self, n: Semitones) -> Self {
        Self::new(self.root - n, self.chord_type)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::many_single_char_names)]
    use rstest::rstest;
    use PitchClass::*;

    use super::*;

    #[rstest(
        chord,
        case("Z"),
        case("c"),
        case("ABC"),
        case("C7b5"),
        case("C#mb5"),
        case("C#mbla"),
        case("CmMaj"),
        case("CmMaj7b5")
    )]
    fn test_from_str_fail(chord: &str) {
        assert!(Chord::from_str(chord).is_err())
    }

    #[rstest(
        chord,
        root,
        third,
        fifth,
        case("C", "C", "E", "G"),
        case("C#", "C#", "F", "G#"),
        case("Db", "Db", "F", "Ab"),
        case("D", "D", "F#", "A"),
        case("D#", "D#", "G", "A#"),
        case("Eb", "Eb", "G", "Bb"),
        case("E", "E", "G#", "B"),
        case("F", "F", "A", "C"),
        case("F#", "F#", "A#", "C#"),
        case("Gb", "Gb", "Bb", "Db"),
        case("G", "G", "B", "D"),
        case("G#", "G#", "C", "D#"),
        case("Ab", "Ab", "C", "Eb"),
        case("A", "A", "C#", "E"),
        case("A#", "A#", "D", "F"),
        case("Bb", "Bb", "D", "F"),
        case("B", "B", "D#", "F#")
    )]
    fn test_from_str_major(chord: &str, root: &str, third: &str, fifth: &str) {
        let c = Chord::from_str(chord).unwrap();
        let r = Note::from_str(root).unwrap();
        let t = Note::from_str(third).unwrap();
        let f = Note::from_str(fifth).unwrap();
        let notes: Vec<Note> = c.notes().collect();
        assert_eq!(notes, vec![r, t, f]);
        assert_eq!(c.chord_type, ChordType::Major);
    }

    #[rstest(
        chord,
        root,
        third,
        fifth,
        case("Cm", "C", "Eb", "G"),
        case("C#m", "C#", "E", "G#"),
        case("Dbm", "Db", "E", "Ab"),
        case("Dm", "D", "F", "A"),
        case("D#m", "D#", "F#", "A#"),
        case("Ebm", "Eb", "Gb", "Bb"),
        case("Em", "E", "G", "B"),
        case("Fm", "F", "Ab", "C"),
        case("F#m", "F#", "A", "C#"),
        case("Gbm", "Gb", "A", "Db"),
        case("Gm", "G", "Bb", "D"),
        case("G#m", "G#", "B", "D#"),
        case("Abm", "Ab", "B", "Eb"),
        case("Am", "A", "C", "E"),
        case("A#m", "A#", "C#", "F"),
        case("Bbm", "Bb", "Db", "F"),
        case("Bm", "B", "D", "F#")
    )]
    fn test_from_str_minor(chord: &str, root: &str, third: &str, fifth: &str) {
        let c = Chord::from_str(chord).unwrap();
        let r = Note::from_str(root).unwrap();
        let t = Note::from_str(third).unwrap();
        let f = Note::from_str(fifth).unwrap();
        let notes: Vec<Note> = c.notes().collect();
        assert_eq!(notes, vec![r, t, f]);
        assert_eq!(c.chord_type, ChordType::Minor);
    }

    #[rstest(
        chord,
        root,
        third,
        fifth,
        case("Csus2", "C", "D", "G"),
        case("C#sus2", "C#", "D#", "G#"),
        case("Dbsus2", "Db", "Eb", "Ab"),
        case("Dsus2", "D", "E", "A"),
        case("D#sus2", "D#", "F", "A#"),
        case("Ebsus2", "Eb", "F", "Bb"),
        case("Esus2", "E", "F#", "B"),
        case("Fsus2", "F", "G", "C"),
        case("F#sus2", "F#", "G#", "C#"),
        case("Gbsus2", "Gb", "Ab", "Db"),
        case("Gsus2", "G", "A", "D"),
        case("G#sus2", "G#", "A#", "D#"),
        case("Absus2", "Ab", "Bb", "Eb"),
        case("Asus2", "A", "B", "E"),
        case("A#sus2", "A#", "C", "F"),
        case("Bbsus2", "Bb", "C", "F"),
        case("Bsus2", "B", "C#", "F#")
    )]
    fn test_from_str_suspended_second(chord: &str, root: &str, third: &str, fifth: &str) {
        let c = Chord::from_str(chord).unwrap();
        let r = Note::from_str(root).unwrap();
        let t = Note::from_str(third).unwrap();
        let f = Note::from_str(fifth).unwrap();
        let notes: Vec<Note> = c.notes().collect();
        assert_eq!(notes, vec![r, t, f]);
        assert_eq!(c.chord_type, ChordType::SuspendedSecond);
    }

    #[rstest(
        chord,
        root,
        third,
        fifth,
        case("Csus4", "C", "F", "G"),
        case("C#sus4", "C#", "F#", "G#"),
        case("Dbsus4", "Db", "Gb", "Ab"),
        case("Dsus4", "D", "G", "A"),
        case("D#sus4", "D#", "G#", "A#"),
        case("Ebsus4", "Eb", "Ab", "Bb"),
        case("Esus4", "E", "A", "B"),
        case("Fsus4", "F", "Bb", "C"),
        case("F#sus4", "F#", "B", "C#"),
        case("Gbsus4", "Gb", "B", "Db"),
        case("Gsus4", "G", "C", "D"),
        case("G#sus4", "G#", "C#", "D#"),
        case("Absus4", "Ab", "Db", "Eb"),
        case("Asus4", "A", "D", "E"),
        case("A#sus4", "A#", "D#", "F"),
        case("Bbsus4", "Bb", "Eb", "F"),
        case("Bsus4", "B", "E", "F#")
    )]
    fn test_from_str_suspended_fourth(chord: &str, root: &str, third: &str, fifth: &str) {
        let c = Chord::from_str(chord).unwrap();
        let r = Note::from_str(root).unwrap();
        let t = Note::from_str(third).unwrap();
        let f = Note::from_str(fifth).unwrap();
        let notes: Vec<Note> = c.notes().collect();
        assert_eq!(notes, vec![r, t, f]);
        assert_eq!(c.chord_type, ChordType::SuspendedFourth);
    }

    #[rstest(
        chord,
        root,
        third,
        fifth,
        case("Caug", "C", "E", "G#"),
        case("C#aug", "C#", "F", "A"),
        case("Dbaug", "Db", "F", "A"),
        case("Daug", "D", "F#", "A#"),
        case("D#aug", "D#", "G", "B"),
        case("Ebaug", "Eb", "G", "B"),
        case("Eaug", "E", "G#", "C"),
        case("Faug", "F", "A", "C#"),
        case("F#aug", "F#", "A#", "D"),
        case("Gbaug", "Gb", "Bb", "D"),
        case("Gaug", "G", "B", "D#"),
        case("G#aug", "G#", "C", "E"),
        case("Abaug", "Ab", "C", "E"),
        case("Aaug", "A", "C#", "F"),
        case("A#aug", "A#", "D", "F#"),
        case("Bbaug", "Bb", "D", "F#"),
        case("Baug", "B", "D#", "G")
    )]
    fn test_from_str_augmented(chord: &str, root: &str, third: &str, fifth: &str) {
        let c = Chord::from_str(chord).unwrap();
        let r = Note::from_str(root).unwrap();
        let t = Note::from_str(third).unwrap();
        let f = Note::from_str(fifth).unwrap();
        let notes: Vec<Note> = c.notes().collect();
        assert_eq!(notes, vec![r, t, f]);
        assert_eq!(c.chord_type, ChordType::Augmented);
    }

    #[rstest(
        chord,
        root,
        third,
        fifth,
        case("Cdim", "C", "Eb", "Gb"),
        case("C#dim", "C#", "E", "G"),
        case("Dbdim", "Db", "E", "G"),
        case("Ddim", "D", "F", "Ab"),
        case("D#dim", "D#", "F#", "A"),
        case("Ebdim", "Eb", "Gb", "A"),
        case("Edim", "E", "G", "Bb"),
        case("Fdim", "F", "Ab", "B"),
        case("F#dim", "F#", "A", "C"),
        case("Gbdim", "Gb", "A", "C"),
        case("Gdim", "G", "Bb", "Db"),
        case("G#dim", "G#", "B", "D"),
        case("Abdim", "Ab", "B", "D"),
        case("Adim", "A", "C", "Eb"),
        case("A#dim", "A#", "C#", "E"),
        case("Bbdim", "Bb", "Db", "E"),
        case("Bdim", "B", "D", "F")
    )]
    fn test_from_str_diminished(chord: &str, root: &str, third: &str, fifth: &str) {
        let c = Chord::from_str(chord).unwrap();
        let r = Note::from_str(root).unwrap();
        let t = Note::from_str(third).unwrap();
        let f = Note::from_str(fifth).unwrap();
        let notes: Vec<Note> = c.notes().collect();
        assert_eq!(notes, vec![r, t, f]);
        assert_eq!(c.chord_type, ChordType::Diminished);
    }

    #[rstest(
        chord,
        root,
        third,
        fifth,
        seventh,
        case("C7", "C", "E", "G", "Bb"),
        case("C#7", "C#", "F", "G#", "B"),
        case("Db7", "Db", "F", "Ab", "B"),
        case("D7", "D", "F#", "A", "C"),
        case("D#7", "D#", "G", "A#", "C#"),
        case("Eb7", "Eb", "G", "Bb", "Db"),
        case("E7", "E", "G#", "B", "D"),
        case("F7", "F", "A", "C", "Eb"),
        case("F#7", "F#", "A#", "C#", "E"),
        case("Gb7", "Gb", "Bb", "Db", "E"),
        case("G7", "G", "B", "D", "F"),
        case("G#7", "G#", "C", "D#", "F#"),
        case("Ab7", "Ab", "C", "Eb", "Gb"),
        case("A7", "A", "C#", "E", "G"),
        case("A#7", "A#", "D", "F", "G#"),
        case("Bb7", "Bb", "D", "F", "Ab"),
        case("B7", "B", "D#", "F#", "A")
    )]
    fn test_from_str_dominant_seventh(
        chord: &str,
        root: &str,
        third: &str,
        fifth: &str,
        seventh: &str,
    ) {
        let c = Chord::from_str(chord).unwrap();
        let r = Note::from_str(root).unwrap();
        let t = Note::from_str(third).unwrap();
        let f = Note::from_str(fifth).unwrap();
        let s = Note::from_str(seventh).unwrap();
        let notes: Vec<Note> = c.notes().collect();
        assert_eq!(notes, vec![r, t, f, s]);
        assert_eq!(c.chord_type, ChordType::DominantSeventh);
    }

    #[rstest(
        chord,
        root,
        third,
        fifth,
        seventh,
        case("Cm7", "C", "Eb", "G", "Bb"),
        case("C#m7", "C#", "E", "G#", "B"),
        case("Dbm7", "Db", "E", "Ab", "B"),
        case("Dm7", "D", "F", "A", "C"),
        case("D#m7", "D#", "F#", "A#", "C#"),
        case("Ebm7", "Eb", "Gb", "Bb", "Db"),
        case("Em7", "E", "G", "B", "D"),
        case("Fm7", "F", "Ab", "C", "Eb"),
        case("F#m7", "F#", "A", "C#", "E"),
        case("Gbm7", "Gb", "A", "Db", "E"),
        case("Gm7", "G", "Bb", "D", "F"),
        case("G#m7", "G#", "B", "D#", "F#"),
        case("Abm7", "Ab", "B", "Eb", "Gb"),
        case("Am7", "A", "C", "E", "G"),
        case("A#m7", "A#", "C#", "F", "G#"),
        case("Bbm7", "Bb", "Db", "F", "Ab"),
        case("Bm7", "B", "D", "F#", "A")
    )]
    fn test_from_str_minor_seventh(
        chord: &str,
        root: &str,
        third: &str,
        fifth: &str,
        seventh: &str,
    ) {
        let c = Chord::from_str(chord).unwrap();
        let r = Note::from_str(root).unwrap();
        let t = Note::from_str(third).unwrap();
        let f = Note::from_str(fifth).unwrap();
        let s = Note::from_str(seventh).unwrap();
        let notes: Vec<Note> = c.notes().collect();
        assert_eq!(notes, vec![r, t, f, s]);
        assert_eq!(c.chord_type, ChordType::MinorSeventh);
    }

    #[rstest(
        chord,
        root,
        third,
        fifth,
        seventh,
        case("Cmaj7", "C", "E", "G", "B"),
        case("C#maj7", "C#", "F", "G#", "C"),
        case("Dbmaj7", "Db", "F", "Ab", "C"),
        case("Dmaj7", "D", "F#", "A", "C#"),
        case("D#maj7", "D#", "G", "A#", "D"),
        case("Ebmaj7", "Eb", "G", "Bb", "D"),
        case("Emaj7", "E", "G#", "B", "D#"),
        case("Fmaj7", "F", "A", "C", "E"),
        case("F#maj7", "F#", "A#", "C#", "F"),
        case("Gbmaj7", "Gb", "Bb", "Db", "F"),
        case("Gmaj7", "G", "B", "D", "F#"),
        case("G#maj7", "G#", "C", "D#", "G"),
        case("Abmaj7", "Ab", "C", "Eb", "G"),
        case("Amaj7", "A", "C#", "E", "G#"),
        case("A#maj7", "A#", "D", "F", "A"),
        case("Bbmaj7", "Bb", "D", "F", "A"),
        case("Bmaj7", "B", "D#", "F#", "A#")
    )]
    fn test_from_str_major_seventh(
        chord: &str,
        root: &str,
        third: &str,
        fifth: &str,
        seventh: &str,
    ) {
        let c = Chord::from_str(chord).unwrap();
        let r = Note::from_str(root).unwrap();
        let t = Note::from_str(third).unwrap();
        let f = Note::from_str(fifth).unwrap();
        let s = Note::from_str(seventh).unwrap();
        let notes: Vec<Note> = c.notes().collect();
        assert_eq!(notes, vec![r, t, f, s]);
        assert_eq!(c.chord_type, ChordType::MajorSeventh);
    }

    #[rstest(
        chord,
        root,
        third,
        fifth,
        seventh,
        case("CmMaj7", "C", "Eb", "G", "B"),
        case("C#mMaj7", "C#", "E", "G#", "C"),
        case("DbmMaj7", "Db", "E", "Ab", "C"),
        case("DmMaj7", "D", "F", "A", "C#"),
        case("D#mMaj7", "D#", "F#", "A#", "D"),
        case("EbmMaj7", "Eb", "Gb", "Bb", "D"),
        case("EmMaj7", "E", "G", "B", "D#"),
        case("FmMaj7", "F", "Ab", "C", "E"),
        case("F#mMaj7", "F#", "A", "C#", "F"),
        case("GbmMaj7", "Gb", "A", "Db", "F"),
        case("GmMaj7", "G", "Bb", "D", "F#"),
        case("G#mMaj7", "G#", "B", "D#", "G"),
        case("AbmMaj7", "Ab", "B", "Eb", "G"),
        case("AmMaj7", "A", "C", "E", "G#"),
        case("A#mMaj7", "A#", "C#", "F", "A"),
        case("BbmMaj7", "Bb", "Db", "F", "A"),
        case("BmMaj7", "B", "D", "F#", "A#")
    )]
    fn test_from_str_minor_major_seventh(
        chord: &str,
        root: &str,
        third: &str,
        fifth: &str,
        seventh: &str,
    ) {
        let c = Chord::from_str(chord).unwrap();
        let r = Note::from_str(root).unwrap();
        let t = Note::from_str(third).unwrap();
        let f = Note::from_str(fifth).unwrap();
        let s = Note::from_str(seventh).unwrap();
        let notes: Vec<Note> = c.notes().collect();
        assert_eq!(notes, vec![r, t, f, s]);
        assert_eq!(c.chord_type, ChordType::MinorMajorSeventh);
    }

    #[rstest(
        chord,
        root,
        third,
        fifth,
        seventh,
        case("Caug7", "C", "E", "G#", "Bb"),
        case("C#aug7", "C#", "F", "A", "B"),
        case("Dbaug7", "Db", "F", "A", "B"),
        case("Daug7", "D", "F#", "A#", "C"),
        case("D#aug7", "D#", "G", "B", "C#"),
        case("Ebaug7", "Eb", "G", "B", "Db"),
        case("Eaug7", "E", "G#", "C", "D"),
        case("Faug7", "F", "A", "C#", "Eb"),
        case("F#aug7", "F#", "A#", "D", "E"),
        case("Gbaug7", "Gb", "Bb", "D", "E"),
        case("Gaug7", "G", "B", "D#", "F"),
        case("G#aug7", "G#", "C", "E", "F#"),
        case("Abaug7", "Ab", "C", "E", "Gb"),
        case("Aaug7", "A", "C#", "F", "G"),
        case("A#aug7", "A#", "D", "F#", "G#"),
        case("Bbaug7", "Bb", "D", "F#", "Ab"),
        case("Baug7", "B", "D#", "G", "A")
    )]
    fn test_from_str_augmented_seventh(
        chord: &str,
        root: &str,
        third: &str,
        fifth: &str,
        seventh: &str,
    ) {
        let c = Chord::from_str(chord).unwrap();
        let r = Note::from_str(root).unwrap();
        let t = Note::from_str(third).unwrap();
        let f = Note::from_str(fifth).unwrap();
        let s = Note::from_str(seventh).unwrap();
        let notes: Vec<Note> = c.notes().collect();
        assert_eq!(notes, vec![r, t, f, s]);
        assert_eq!(c.chord_type, ChordType::AugmentedSeventh);
    }

    #[rstest(
        chord,
        root,
        third,
        fifth,
        seventh,
        case("CaugMaj7", "C", "E", "G#", "B"),
        case("C#augMaj7", "C#", "F", "A", "C"),
        case("DbaugMaj7", "Db", "F", "A", "C"),
        case("DaugMaj7", "D", "F#", "A#", "C#"),
        case("D#augMaj7", "D#", "G", "B", "D"),
        case("EbaugMaj7", "Eb", "G", "B", "D"),
        case("EaugMaj7", "E", "G#", "C", "D#"),
        case("FaugMaj7", "F", "A", "C#", "E"),
        case("F#augMaj7", "F#", "A#", "D", "F"),
        case("GbaugMaj7", "Gb", "Bb", "D", "F"),
        case("GaugMaj7", "G", "B", "D#", "F#"),
        case("G#augMaj7", "G#", "C", "E", "G"),
        case("AbaugMaj7", "Ab", "C", "E", "G"),
        case("AaugMaj7", "A", "C#", "F", "G#"),
        case("A#augMaj7", "A#", "D", "F#", "A"),
        case("BbaugMaj7", "Bb", "D", "F#", "A"),
        case("BaugMaj7", "B", "D#", "G", "A#")
    )]
    fn test_from_str_augmented_major_seventh(
        chord: &str,
        root: &str,
        third: &str,
        fifth: &str,
        seventh: &str,
    ) {
        let c = Chord::from_str(chord).unwrap();
        let r = Note::from_str(root).unwrap();
        let t = Note::from_str(third).unwrap();
        let f = Note::from_str(fifth).unwrap();
        let s = Note::from_str(seventh).unwrap();
        let notes: Vec<Note> = c.notes().collect();
        assert_eq!(notes, vec![r, t, f, s]);
        assert_eq!(c.chord_type, ChordType::AugmentedMajorSeventh);
    }

    #[rstest(
        chord,
        root,
        third,
        fifth,
        seventh,
        case("Cdim7", "C", "Eb", "Gb", "A"),
        case("C#dim7", "C#", "E", "G", "Bb"),
        case("Dbdim7", "Db", "E", "G", "Bb"),
        case("Ddim7", "D", "F", "Ab", "B"),
        case("D#dim7", "D#", "F#", "A", "C"),
        case("Ebdim7", "Eb", "Gb", "A", "C"),
        case("Edim7", "E", "G", "Bb", "Db"),
        case("Fdim7", "F", "Ab", "B", "D"),
        case("F#dim7", "F#", "A", "C", "Eb"),
        case("Gbdim7", "Gb", "A", "C", "Eb"),
        case("Gdim7", "G", "Bb", "Db", "E"),
        case("G#dim7", "G#", "B", "D", "F"),
        case("Abdim7", "Ab", "B", "D", "F"),
        case("Adim7", "A", "C", "Eb", "Gb"),
        case("A#dim7", "A#", "C#", "E", "G"),
        case("Bbdim7", "Bb", "Db", "E", "G"),
        case("Bdim7", "B", "D", "F", "Ab")
    )]
    fn test_from_str_diminished_seventh(
        chord: &str,
        root: &str,
        third: &str,
        fifth: &str,
        seventh: &str,
    ) {
        let c = Chord::from_str(chord).unwrap();
        let r = Note::from_str(root).unwrap();
        let t = Note::from_str(third).unwrap();
        let f = Note::from_str(fifth).unwrap();
        let s = Note::from_str(seventh).unwrap();
        let notes: Vec<Note> = c.notes().collect();
        assert_eq!(notes, vec![r, t, f, s]);
        assert_eq!(c.chord_type, ChordType::DiminishedSeventh);
    }

    #[rstest(
        chord,
        root,
        third,
        fifth,
        seventh,
        case("Cm7b5", "C", "Eb", "Gb", "Bb"),
        case("C#m7b5", "C#", "E", "G", "B"),
        case("Dbm7b5", "Db", "E", "G", "B"),
        case("Dm7b5", "D", "F", "Ab", "C"),
        case("D#m7b5", "D#", "F#", "A", "C#"),
        case("Ebm7b5", "Eb", "Gb", "A", "Db"),
        case("Em7b5", "E", "G", "Bb", "D"),
        case("Fm7b5", "F", "Ab", "B", "Eb"),
        case("F#m7b5", "F#", "A", "C", "E"),
        case("Gbm7b5", "Gb", "A", "C", "E"),
        case("Gm7b5", "G", "Bb", "Db", "F"),
        case("G#m7b5", "G#", "B", "D", "F#"),
        case("Abm7b5", "Ab", "B", "D", "Gb"),
        case("Am7b5", "A", "C", "Eb", "G"),
        case("A#m7b5", "A#", "C#", "E", "G#"),
        case("Bbm7b5", "Bb", "Db", "E", "Ab"),
        case("Bm7b5", "B", "D", "F", "A")
    )]
    fn test_from_str_half_diminished_seventh(
        chord: &str,
        root: &str,
        third: &str,
        fifth: &str,
        seventh: &str,
    ) {
        let c = Chord::from_str(chord).unwrap();
        let r = Note::from_str(root).unwrap();
        let t = Note::from_str(third).unwrap();
        let f = Note::from_str(fifth).unwrap();
        let s = Note::from_str(seventh).unwrap();
        let notes: Vec<Note> = c.notes().collect();
        assert_eq!(notes, vec![r, t, f, s]);
        assert_eq!(c.chord_type, ChordType::HalfDiminishedSeventh);
    }

    #[rstest(
        pitches,
        chord_name,
        // Test C-chords.
        case(vec![C, E, G], "C"),
        case(vec![C, DSharp, G], "Cm"),
        case(vec![C, D, G], "Csus2"),
        case(vec![C, F, G], "Csus4"),
        case(vec![C, E, GSharp], "Caug"),
        case(vec![C, DSharp, FSharp], "Cdim"),
        case(vec![C, E, G, ASharp], "C7"),
        case(vec![C, DSharp, G, ASharp], "Cm7"),
        case(vec![C, E, G, B], "Cmaj7"),
        case(vec![C, DSharp, G, B], "CmMaj7"),
        case(vec![C, E, GSharp, ASharp], "Caug7"),
        case(vec![C, E, GSharp, B], "CaugMaj7"),
        case(vec![C, DSharp, FSharp, A], "Cdim7"),
        case(vec![C, DSharp, FSharp, ASharp], "Cm7b5"),
        // Test some chords with other root notes.
        case(vec![D, FSharp, A], "D"),
        case(vec![D, F, A], "Dm"),
        case(vec![D, FSharp, A, C], "D7"),
        case(vec![G, B, D], "G"),
        // Test pitch class list in different order.
        case(vec![C, G, E], "C"),
    )]
    fn test_get_chord_type(pitches: Vec<PitchClass>, chord_name: &str) {
        let chord1 = Chord::try_from(&pitches[..]).unwrap();
        let chord2 = Chord::from_str(chord_name).unwrap();
        assert_eq!(chord1, chord2);
    }

    #[rstest(
        chord,
        n,
        result,
        case("C", 0, "C"),
        case("C#", 0, "C#"),
        case("Db", 0, "Db"),
        case("Cm", 1, "C#m"),
        case("Cmaj7", 2, "Dmaj7"),
        case("Cdim", 4, "Edim"),
        case("C#", 2, "D#"),
        case("A#m", 3, "C#m"),
        case("A", 12, "A"),
        case("A#", 12, "A#"),
        case("Ab", 12, "Ab")
    )]
    fn test_add_semitones(chord: &str, n: Semitones, result: &str) {
        let c = Chord::from_str(chord).unwrap();
        assert_eq!(c + n, Chord::from_str(result).unwrap());
    }

    #[rstest(
        chord,
        n,
        result,
        case("C", 0, "C"),
        case("C#", 0, "C#"),
        case("Db", 0, "Db"),
        case("Cm", 1, "Bm"),
        case("Cmaj7", 2, "Bbmaj7"),
        case("Adim", 3, "Gbdim"),
        case("A", 12, "A"),
        case("A#", 12, "A#"),
        case("Ab", 12, "Ab")
    )]
    fn test_subtract_semitones(chord: &str, n: Semitones, result: &str) {
        let c = Chord::from_str(chord).unwrap();
        assert_eq!(c - n, Chord::from_str(result).unwrap());
    }
}
