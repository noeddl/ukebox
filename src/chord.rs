use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;

use itertools::Itertools;

use crate::{
    ChordType, Note, PitchClass, Semitones, UkeString, Voicing, VoicingConfig, STRING_COUNT,
};

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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Chord {
    pub root: Note,
    pub chord_type: ChordType,
    pub notes: Vec<Note>,
}

impl Chord {
    pub fn new(root: Note, chord_type: ChordType) -> Self {
        let notes = chord_type.intervals().map(|i| root + i).collect();
        Self {
            root,
            chord_type,
            notes,
        }
    }

    /// Return an iterator over the chord's notes that are played on our instrument.
    ///
    /// If the chord contains more notes than we have strings on our instrument,
    /// only required notes are played.
    pub fn played_notes(&self) -> impl Iterator<Item = Note> + '_ {
        self.chord_type
            .required_intervals()
            .chain(self.chord_type.optional_intervals())
            .take(STRING_COUNT)
            .map(move |i| self.root + i)
    }

    pub fn voicings(&self, config: VoicingConfig) -> impl Iterator<Item = Voicing> + '_ {
        config
            .tuning
            .roots()
            // For each ukulele string, keep track of all the frets that when pressed down
            // while playing the string result in a note of the chord.
            .map(|root| {
                self.played_notes()
                    // Allow each note to be checked twice on the fretboard.
                    .cartesian_product(vec![0, 12])
                    // Determine the fret on which `note` is played.
                    .map(|(note, st)| (root, (note.pitch_class - root.pitch_class) + st, note))
                    // Keep only frets within the given boundaries.
                    .filter(|(_r, fret, _n)| fret >= &config.min_fret && fret <= &config.max_fret)
                    .collect::<Vec<UkeString>>()
            })
            // At this point, we have collected all possible positions of the notes in the chord
            // on each ukulele string. Now let's check all combinations and determine the ones
            // that result in a valid voicing of the chord.
            .multi_cartesian_product()
            // Create voicing from the UkeString vec.
            .map(|us_vec| Voicing::from(&us_vec[..]))
            // Keep only valid voicings.
            .filter(|voicing| voicing.spells_out(self) && voicing.get_span() <= config.max_span)
            .sorted()
    }

    pub fn transpose(&self, semitones: i8) -> Chord {
        match semitones {
            s if s < 0 => self.clone() - semitones.abs() as Semitones,
            _ => self.clone() + semitones as Semitones,
        }
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
        // 1. Check the two first characters of the input string (for notes such as `C#`).
        // 2. Check only the first character (for notes such as `C`).
        for i in (1..3).rev() {
            if let Some(prefix) = s.get(0..i) {
                // Try to convert the prefix into a `Note`.
                if let Ok(root) = Note::from_str(prefix) {
                    // Try to convert the remaining string into a `ChordType`.
                    if let Some(suffix) = s.get(i..) {
                        if let Ok(chord_type) = ChordType::from_str(suffix) {
                            return Ok(Self::new(root, chord_type));
                        }
                    }
                }
            }
        }

        let name = s.to_string();
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
    use rstest::rstest;
    use PitchClass::*;

    use super::*;

    #[rstest(
        chord,
        case("Z"),
        case("c"),
        case("ABC"),
        case("C#mb5"),
        case("C#mbla"),
        case("CmMaj"),
        case("CmMaj7b5")
    )]
    fn test_from_str_fail(chord: &str) {
        assert!(Chord::from_str(chord).is_err())
    }

    #[rstest(
        chord_base,
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
    fn test_from_str_major(
        #[values("", "maj", "M")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        third: Note,
        fifth: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(chord.notes, vec![root, third, fifth]);
        assert_eq!(chord.chord_type, ChordType::Major);
    }

    #[rstest(
        chord_base,
        root,
        third,
        fifth,
        seventh,
        case("C", "C", "E", "G", "B"),
        case("C#", "C#", "F", "G#", "C"),
        case("Db", "Db", "F", "Ab", "C"),
        case("D", "D", "F#", "A", "C#"),
        case("D#", "D#", "G", "A#", "D"),
        case("Eb", "Eb", "G", "Bb", "D"),
        case("E", "E", "G#", "B", "D#"),
        case("F", "F", "A", "C", "E"),
        case("F#", "F#", "A#", "C#", "F"),
        case("Gb", "Gb", "Bb", "Db", "F"),
        case("G", "G", "B", "D", "F#"),
        case("G#", "G#", "C", "D#", "G"),
        case("Ab", "Ab", "C", "Eb", "G"),
        case("A", "A", "C#", "E", "G#"),
        case("A#", "A#", "D", "F", "A"),
        case("Bb", "Bb", "D", "F", "A"),
        case("B", "B", "D#", "F#", "A#")
    )]
    fn test_from_str_major_seventh(
        #[values("maj7", "M7")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        third: Note,
        fifth: Note,
        seventh: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(chord.notes, vec![root, third, fifth, seventh]);
        assert_eq!(chord.chord_type, ChordType::MajorSeventh);
    }

    #[rstest(
        chord_base,
        root,
        third,
        fifth,
        seventh,
        ninth,
        case("C", "C", "E", "G", "B", "D"),
        case("C#", "C#", "F", "G#", "C", "D#"),
        case("Db", "Db", "F", "Ab", "C", "Eb"),
        case("D", "D", "F#", "A", "C#", "E"),
        case("D#", "D#", "G", "A#", "D", "F"),
        case("Eb", "Eb", "G", "Bb", "D", "F"),
        case("E", "E", "G#", "B", "D#", "F#"),
        case("F", "F", "A", "C", "E", "G"),
        case("F#", "F#", "A#", "C#", "F", "G#"),
        case("Gb", "Gb", "Bb", "Db", "F", "Ab"),
        case("G", "G", "B", "D", "F#", "A"),
        case("G#", "G#", "C", "D#", "G", "A#"),
        case("Ab", "Ab", "C", "Eb", "G", "Bb"),
        case("A", "A", "C#", "E", "G#", "B"),
        case("A#", "A#", "D", "F", "A", "C"),
        case("Bb", "Bb", "D", "F", "A", "C"),
        case("B", "B", "D#", "F#", "A#", "C#")
    )]
    fn test_from_str_major_ninth(
        #[values("maj9", "M9")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        third: Note,
        fifth: Note,
        seventh: Note,
        ninth: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(chord.notes, vec![root, third, fifth, seventh, ninth]);
        assert_eq!(chord.chord_type, ChordType::MajorNinth);
    }

    #[rstest(
        chord_base,
        root,
        third,
        fifth,
        seventh,
        ninth,
        eleventh,
        case("C", "C", "E", "G", "B", "D", "F"),
        case("C#", "C#", "F", "G#", "C", "D#", "F#"),
        case("Db", "Db", "F", "Ab", "C", "Eb", "Gb"),
        case("D", "D", "F#", "A", "C#", "E", "G"),
        case("D#", "D#", "G", "A#", "D", "F", "G#"),
        case("Eb", "Eb", "G", "Bb", "D", "F", "Ab"),
        case("E", "E", "G#", "B", "D#", "F#", "A"),
        case("F", "F", "A", "C", "E", "G", "A#"),
        case("F#", "F#", "A#", "C#", "F", "G#", "B"),
        case("Gb", "Gb", "Bb", "Db", "F", "Ab", "B"),
        case("G", "G", "B", "D", "F#", "A", "C"),
        case("G#", "G#", "C", "D#", "G", "A#", "C#"),
        case("Ab", "Ab", "C", "Eb", "G", "Bb", "Db"),
        case("A", "A", "C#", "E", "G#", "B", "D"),
        case("A#", "A#", "D", "F", "A", "C", "D#"),
        case("Bb", "Bb", "D", "F", "A", "C", "Eb"),
        case("B", "B", "D#", "F#", "A#", "C#", "E")
    )]
    fn test_from_str_major_eleventh(
        #[values("maj11", "M11")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        third: Note,
        fifth: Note,
        seventh: Note,
        ninth: Note,
        eleventh: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(
            chord.notes,
            vec![root, third, fifth, seventh, ninth, eleventh]
        );
        assert_eq!(chord.chord_type, ChordType::MajorEleventh);
    }

    #[rstest(
        chord_base,
        root,
        third,
        fifth,
        seventh,
        ninth,
        eleventh,
        thirteenth,
        case("C", "C", "E", "G", "B", "D", "F", "A"),
        case("C#", "C#", "F", "G#", "C", "D#", "F#", "A#"),
        case("Db", "Db", "F", "Ab", "C", "Eb", "Gb", "Bb"),
        case("D", "D", "F#", "A", "C#", "E", "G", "B"),
        case("D#", "D#", "G", "A#", "D", "F", "G#", "C"),
        case("Eb", "Eb", "G", "Bb", "D", "F", "Ab", "C"),
        case("E", "E", "G#", "B", "D#", "F#", "A", "C#"),
        case("F", "F", "A", "C", "E", "G", "A#", "D"),
        case("F#", "F#", "A#", "C#", "F", "G#", "B", "D#"),
        case("Gb", "Gb", "Bb", "Db", "F", "Ab", "B", "Eb"),
        case("G", "G", "B", "D", "F#", "A", "C", "E"),
        case("G#", "G#", "C", "D#", "G", "A#", "C#", "F"),
        case("Ab", "Ab", "C", "Eb", "G", "Bb", "Db", "F"),
        case("A", "A", "C#", "E", "G#", "B", "D", "F#"),
        case("A#", "A#", "D", "F", "A", "C", "D#", "G"),
        case("Bb", "Bb", "D", "F", "A", "C", "Eb", "G"),
        case("B", "B", "D#", "F#", "A#", "C#", "E", "G#")
    )]
    fn test_from_str_major_thirteenth(
        #[values("maj13", "M13")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        third: Note,
        fifth: Note,
        seventh: Note,
        ninth: Note,
        eleventh: Note,
        thirteenth: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(
            chord.notes,
            vec![root, third, fifth, seventh, ninth, eleventh, thirteenth]
        );
        assert_eq!(chord.chord_type, ChordType::MajorThirteenth);
    }

    #[rstest(
        chord_base,
        root,
        third,
        fifth,
        sixth,
        case("C", "C", "E", "G", "A"),
        case("C#", "C#", "F", "G#", "A#"),
        case("Db", "Db", "F", "Ab", "Bb"),
        case("D", "D", "F#", "A", "B"),
        case("D#", "D#", "G", "A#", "C"),
        case("Eb", "Eb", "G", "Bb", "C"),
        case("E", "E", "G#", "B", "C#"),
        case("F", "F", "A", "C", "D"),
        case("F#", "F#", "A#", "C#", "D#"),
        case("Gb", "Gb", "Bb", "Db", "Eb"),
        case("G", "G", "B", "D", "E"),
        case("G#", "G#", "C", "D#", "F"),
        case("Ab", "Ab", "C", "Eb", "F"),
        case("A", "A", "C#", "E", "F#"),
        case("A#", "A#", "D", "F", "G"),
        case("Bb", "Bb", "D", "F", "G"),
        case("B", "B", "D#", "F#", "G#")
    )]
    fn test_from_str_major_sixth(
        #[values("6", "maj6", "M6")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        third: Note,
        fifth: Note,
        sixth: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(chord.notes, vec![root, third, fifth, sixth]);
        assert_eq!(chord.chord_type, ChordType::MajorSixth);
    }

    #[rstest(
        chord_base,
        root,
        third,
        fifth,
        sixth,
        ninth,
        case("C", "C", "E", "G", "A", "D"),
        case("C#", "C#", "F", "G#", "A#", "D#"),
        case("Db", "Db", "F", "Ab", "Bb", "Eb"),
        case("D", "D", "F#", "A", "B", "E"),
        case("D#", "D#", "G", "A#", "C", "F"),
        case("Eb", "Eb", "G", "Bb", "C", "F"),
        case("E", "E", "G#", "B", "C#", "F#"),
        case("F", "F", "A", "C", "D", "G"),
        case("F#", "F#", "A#", "C#", "D#", "G#"),
        case("Gb", "Gb", "Bb", "Db", "Eb", "Ab"),
        case("G", "G", "B", "D", "E", "A"),
        case("G#", "G#", "C", "D#", "F", "A#"),
        case("Ab", "Ab", "C", "Eb", "F", "Bb"),
        case("A", "A", "C#", "E", "F#", "B"),
        case("A#", "A#", "D", "F", "G", "C"),
        case("Bb", "Bb", "D", "F", "G", "C"),
        case("B", "B", "D#", "F#", "G#", "C#")
    )]
    fn test_from_str_sixth_ninth(
        #[values("6/9", "maj6/9", "M6/9")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        third: Note,
        fifth: Note,
        sixth: Note,
        ninth: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(chord.notes, vec![root, third, fifth, sixth, ninth]);
        assert_eq!(chord.chord_type, ChordType::SixthNinth);
    }

    #[rstest(
        chord_base,
        root,
        third,
        fifth,
        seventh,
        case("C", "C", "E", "G", "Bb"),
        case("C#", "C#", "F", "G#", "B"),
        case("Db", "Db", "F", "Ab", "B"),
        case("D", "D", "F#", "A", "C"),
        case("D#", "D#", "G", "A#", "C#"),
        case("Eb", "Eb", "G", "Bb", "Db"),
        case("E", "E", "G#", "B", "D"),
        case("F", "F", "A", "C", "Eb"),
        case("F#", "F#", "A#", "C#", "E"),
        case("Gb", "Gb", "Bb", "Db", "E"),
        case("G", "G", "B", "D", "F"),
        case("G#", "G#", "C", "D#", "F#"),
        case("Ab", "Ab", "C", "Eb", "Gb"),
        case("A", "A", "C#", "E", "G"),
        case("A#", "A#", "D", "F", "G#"),
        case("Bb", "Bb", "D", "F", "Ab"),
        case("B", "B", "D#", "F#", "A")
    )]
    fn test_from_str_dominant_seventh(
        #[values("7", "dom")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        third: Note,
        fifth: Note,
        seventh: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(chord.notes, vec![root, third, fifth, seventh]);
        assert_eq!(chord.chord_type, ChordType::DominantSeventh);
    }

    #[rstest(
        chord_base,
        root,
        third,
        fifth,
        seventh,
        ninth,
        case("C", "C", "E", "G", "Bb", "D"),
        case("C#", "C#", "F", "G#", "B", "D#"),
        case("Db", "Db", "F", "Ab", "B", "Eb"),
        case("D", "D", "F#", "A", "C", "E"),
        case("D#", "D#", "G", "A#", "C#", "F"),
        case("Eb", "Eb", "G", "Bb", "Db", "F"),
        case("E", "E", "G#", "B", "D", "F#"),
        case("F", "F", "A", "C", "Eb", "G"),
        case("F#", "F#", "A#", "C#", "E", "G#"),
        case("Gb", "Gb", "Bb", "Db", "E", "Ab"),
        case("G", "G", "B", "D", "F", "A"),
        case("G#", "G#", "C", "D#", "F#", "A#"),
        case("Ab", "Ab", "C", "Eb", "Gb", "Bb"),
        case("A", "A", "C#", "E", "G", "B"),
        case("A#", "A#", "D", "F", "G#", "C"),
        case("Bb", "Bb", "D", "F", "Ab", "C"),
        case("B", "B", "D#", "F#", "A", "C#")
    )]
    fn test_from_str_dominant_ninth(
        #[values("9")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        third: Note,
        fifth: Note,
        seventh: Note,
        ninth: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(chord.notes, vec![root, third, fifth, seventh, ninth]);
        assert_eq!(chord.chord_type, ChordType::DominantNinth);
    }

    #[rstest(
        chord_base,
        root,
        third,
        fifth,
        seventh,
        ninth,
        eleventh,
        case("C", "C", "E", "G", "Bb", "D", "F"),
        case("C#", "C#", "F", "G#", "B", "D#", "F#"),
        case("Db", "Db", "F", "Ab", "B", "Eb", "Gb"),
        case("D", "D", "F#", "A", "C", "E", "G"),
        case("D#", "D#", "G", "A#", "C#", "F", "G#"),
        case("Eb", "Eb", "G", "Bb", "Db", "F", "Ab"),
        case("E", "E", "G#", "B", "D", "F#", "A"),
        case("F", "F", "A", "C", "Eb", "G", "A#"),
        case("F#", "F#", "A#", "C#", "E", "G#", "B"),
        case("Gb", "Gb", "Bb", "Db", "E", "Ab", "B"),
        case("G", "G", "B", "D", "F", "A", "C"),
        case("G#", "G#", "C", "D#", "F#", "A#", "C#"),
        case("Ab", "Ab", "C", "Eb", "Gb", "Bb", "Db"),
        case("A", "A", "C#", "E", "G", "B", "D"),
        case("A#", "A#", "D", "F", "G#", "C", "D#"),
        case("Bb", "Bb", "D", "F", "Ab", "C", "Eb"),
        case("B", "B", "D#", "F#", "A", "C#", "E")
    )]
    fn test_from_str_dominant_eleventh(
        #[values("11")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        third: Note,
        fifth: Note,
        seventh: Note,
        ninth: Note,
        eleventh: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(
            chord.notes,
            vec![root, third, fifth, seventh, ninth, eleventh]
        );
        assert_eq!(chord.chord_type, ChordType::DominantEleventh);
    }

    #[rstest(
        chord_base,
        root,
        third,
        fifth,
        seventh,
        ninth,
        eleventh,
        thirteenth,
        case("C", "C", "E", "G", "Bb", "D", "F", "A"),
        case("C#", "C#", "F", "G#", "B", "D#", "F#", "A#"),
        case("Db", "Db", "F", "Ab", "B", "Eb", "Gb", "Bb"),
        case("D", "D", "F#", "A", "C", "E", "G", "B"),
        case("D#", "D#", "G", "A#", "C#", "F", "G#", "C"),
        case("Eb", "Eb", "G", "Bb", "Db", "F", "Ab", "C"),
        case("E", "E", "G#", "B", "D", "F#", "A", "C#"),
        case("F", "F", "A", "C", "Eb", "G", "A#", "D"),
        case("F#", "F#", "A#", "C#", "E", "G#", "B", "D#"),
        case("Gb", "Gb", "Bb", "Db", "E", "Ab", "B", "Eb"),
        case("G", "G", "B", "D", "F", "A", "C", "E"),
        case("G#", "G#", "C", "D#", "F#", "A#", "C#", "F"),
        case("Ab", "Ab", "C", "Eb", "Gb", "Bb", "Db", "F"),
        case("A", "A", "C#", "E", "G", "B", "D", "F#"),
        case("A#", "A#", "D", "F", "G#", "C", "D#", "G"),
        case("Bb", "Bb", "D", "F", "Ab", "C", "Eb", "G"),
        case("B", "B", "D#", "F#", "A", "C#", "E", "G#")
    )]
    fn test_from_str_dominant_thirteenth(
        #[values("13")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        third: Note,
        fifth: Note,
        seventh: Note,
        ninth: Note,
        eleventh: Note,
        thirteenth: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(
            chord.notes,
            vec![root, third, fifth, seventh, ninth, eleventh, thirteenth]
        );
        assert_eq!(chord.chord_type, ChordType::DominantThirteenth);
    }

    #[rstest(
        chord_base,
        root,
        third,
        fifth,
        seventh,
        ninth,
        case("C", "C", "E", "G", "Bb", "Db"),
        case("C#", "C#", "F", "G#", "B", "D"),
        case("Db", "Db", "F", "Ab", "B", "D"),
        case("D", "D", "F#", "A", "C", "Eb"),
        case("D#", "D#", "G", "A#", "C#", "E"),
        case("Eb", "Eb", "G", "Bb", "Db", "E"),
        case("E", "E", "G#", "B", "D", "F"),
        case("F", "F", "A", "C", "Eb", "F#"),
        case("F#", "F#", "A#", "C#", "E", "G"),
        case("Gb", "Gb", "Bb", "Db", "E", "G"),
        case("G", "G", "B", "D", "F", "Ab"),
        case("G#", "G#", "C", "D#", "F#", "A"),
        case("Ab", "Ab", "C", "Eb", "Gb", "A"),
        case("A", "A", "C#", "E", "G", "Bb"),
        case("A#", "A#", "D", "F", "G#", "B"),
        case("Bb", "Bb", "D", "F", "Ab", "B"),
        case("B", "B", "D#", "F#", "A", "C")
    )]
    fn test_from_str_dominant_seventh_flat_ninth(
        #[values("7b9")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        third: Note,
        fifth: Note,
        seventh: Note,
        ninth: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(chord.notes, vec![root, third, fifth, seventh, ninth]);
        assert_eq!(chord.chord_type, ChordType::DominantSeventhFlatNinth);
    }

    #[rstest(
        chord_base,
        root,
        third,
        fifth,
        seventh,
        ninth,
        case("C", "C", "E", "G", "Bb", "D#"),
        case("C#", "C#", "F", "G#", "B", "E"),
        case("Db", "Db", "F", "Ab", "B", "E"),
        case("D", "D", "F#", "A", "C", "F"),
        case("D#", "D#", "G", "A#", "C#", "F#"),
        case("Eb", "Eb", "G", "Bb", "Db", "F#"),
        case("E", "E", "G#", "B", "D", "G"),
        case("F", "F", "A", "C", "Eb", "G#"),
        case("F#", "F#", "A#", "C#", "E", "A"),
        case("Gb", "Gb", "Bb", "Db", "E", "A"),
        case("G", "G", "B", "D", "F", "A#"),
        case("G#", "G#", "C", "D#", "F#", "B"),
        case("Ab", "Ab", "C", "Eb", "Gb", "B"),
        case("A", "A", "C#", "E", "G", "C"),
        case("A#", "A#", "D", "F", "G#", "C#"),
        case("Bb", "Bb", "D", "F", "Ab", "C#"),
        case("B", "B", "D#", "F#", "A", "D")
    )]
    fn test_from_str_dominant_seventh_sharp_ninth(
        #[values("7#9")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        third: Note,
        fifth: Note,
        seventh: Note,
        ninth: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(chord.notes, vec![root, third, fifth, seventh, ninth]);
        assert_eq!(chord.chord_type, ChordType::DominantSeventhSharpNinth);
    }

    #[rstest(
        chord_base,
        root,
        third,
        fifth,
        seventh,
        case("C", "C", "E", "Gb", "Bb"),
        case("C#", "C#", "F", "G", "B"),
        case("Db", "Db", "F", "G", "B"),
        case("D", "D", "F#", "Ab", "C"),
        case("D#", "D#", "G", "A", "C#"),
        case("Eb", "Eb", "G", "A", "Db"),
        case("E", "E", "G#", "Bb", "D"),
        case("F", "F", "A", "B", "Eb"),
        case("F#", "F#", "A#", "C", "E"),
        case("Gb", "Gb", "Bb", "C", "E"),
        case("G", "G", "B", "Db", "F"),
        case("G#", "G#", "C", "D", "F#"),
        case("Ab", "Ab", "C", "D", "Gb"),
        case("A", "A", "C#", "Eb", "G"),
        case("A#", "A#", "D", "E", "G#"),
        case("Bb", "Bb", "D", "E", "Ab"),
        case("B", "B", "D#", "F", "A")
    )]
    fn test_from_str_dominant_seventh_flat_fifth(
        #[values("7b5", "7dim5")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        third: Note,
        fifth: Note,
        seventh: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(chord.notes, vec![root, third, fifth, seventh]);
        assert_eq!(chord.chord_type, ChordType::DominantSeventhFlatFifth);
    }

    #[rstest(
        chord_base,
        root,
        fourth,
        fifth,
        case("C", "C", "F", "G"),
        case("C#", "C#", "F#", "G#"),
        case("Db", "Db", "Gb", "Ab"),
        case("D", "D", "G", "A"),
        case("D#", "D#", "G#", "A#"),
        case("Eb", "Eb", "Ab", "Bb"),
        case("E", "E", "A", "B"),
        case("F", "F", "Bb", "C"),
        case("F#", "F#", "B", "C#"),
        case("Gb", "Gb", "B", "Db"),
        case("G", "G", "C", "D"),
        case("G#", "G#", "C#", "D#"),
        case("Ab", "Ab", "Db", "Eb"),
        case("A", "A", "D", "E"),
        case("A#", "A#", "D#", "F"),
        case("Bb", "Bb", "Eb", "F"),
        case("B", "B", "E", "F#")
    )]
    fn test_from_str_suspended_fourth(
        #[values("sus4", "sus")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        fourth: Note,
        fifth: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(chord.notes, vec![root, fourth, fifth]);
        assert_eq!(chord.chord_type, ChordType::SuspendedFourth);
    }

    #[rstest(
        chord_base,
        root,
        second,
        fifth,
        case("C", "C", "D", "G"),
        case("C#", "C#", "D#", "G#"),
        case("Db", "Db", "Eb", "Ab"),
        case("D", "D", "E", "A"),
        case("D#", "D#", "F", "A#"),
        case("Eb", "Eb", "F", "Bb"),
        case("E", "E", "F#", "B"),
        case("F", "F", "G", "C"),
        case("F#", "F#", "G#", "C#"),
        case("Gb", "Gb", "Ab", "Db"),
        case("G", "G", "A", "D"),
        case("G#", "G#", "A#", "D#"),
        case("Ab", "Ab", "Bb", "Eb"),
        case("A", "A", "B", "E"),
        case("A#", "A#", "C", "F"),
        case("Bb", "Bb", "C", "F"),
        case("B", "B", "C#", "F#")
    )]
    fn test_from_str_suspended_second(
        #[values("sus2")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        second: Note,
        fifth: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(chord.notes, vec![root, second, fifth]);
        assert_eq!(chord.chord_type, ChordType::SuspendedSecond);
    }

    #[rstest(
        chord_base,
        root,
        fourth,
        fifth,
        seventh,
        case("C", "C", "F", "G", "Bb"),
        case("C#", "C#", "F#", "G#", "B"),
        case("Db", "Db", "Gb", "Ab", "B"),
        case("D", "D", "G", "A", "C"),
        case("D#", "D#", "G#", "A#", "C#"),
        case("Eb", "Eb", "Ab", "Bb", "Db"),
        case("E", "E", "A", "B", "D"),
        case("F", "F", "Bb", "C", "Eb"),
        case("F#", "F#", "B", "C#", "E"),
        case("Gb", "Gb", "B", "Db", "E"),
        case("G", "G", "C", "D", "F"),
        case("G#", "G#", "C#", "D#", "F#"),
        case("Ab", "Ab", "Db", "Eb", "Gb"),
        case("A", "A", "D", "E", "G"),
        case("A#", "A#", "D#", "F", "G#"),
        case("Bb", "Bb", "Eb", "F", "Ab"),
        case("B", "B", "E", "F#", "A")
    )]
    fn test_from_str_dominant_seventh_suspended_fourth(
        #[values("7sus4", "7sus")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        fourth: Note,
        fifth: Note,
        seventh: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(chord.notes, vec![root, fourth, fifth, seventh]);
        assert_eq!(chord.chord_type, ChordType::DominantSeventhSuspendedFourth);
    }

    #[rstest(
        chord_base,
        root,
        second,
        fifth,
        seventh,
        case("C", "C", "D", "G", "Bb"),
        case("C#", "C#", "D#", "G#", "B"),
        case("Db", "Db", "Eb", "Ab", "B"),
        case("D", "D", "E", "A", "C"),
        case("D#", "D#", "F", "A#", "C#"),
        case("Eb", "Eb", "F", "Bb", "Db"),
        case("E", "E", "F#", "B", "D"),
        case("F", "F", "G", "C", "Eb"),
        case("F#", "F#", "G#", "C#", "E"),
        case("Gb", "Gb", "Ab", "Db", "E"),
        case("G", "G", "A", "D", "F"),
        case("G#", "G#", "A#", "D#", "F#"),
        case("Ab", "Ab", "Bb", "Eb", "Gb"),
        case("A", "A", "B", "E", "G"),
        case("A#", "A#", "C", "F", "G#"),
        case("Bb", "Bb", "C", "F", "Ab"),
        case("B", "B", "Db", "F#", "A")
    )]
    fn test_from_str_dominant_seventh_suspended_second(
        #[values("7sus2")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        second: Note,
        fifth: Note,
        seventh: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(chord.notes, vec![root, second, fifth, seventh]);
        assert_eq!(chord.chord_type, ChordType::DominantSeventhSuspendedSecond);
    }

    #[rstest(
        chord_base,
        root,
        third,
        fifth,
        case("C", "C", "Eb", "G"),
        case("C#", "C#", "E", "G#"),
        case("Db", "Db", "E", "Ab"),
        case("D", "D", "F", "A"),
        case("D#", "D#", "F#", "A#"),
        case("Eb", "Eb", "Gb", "Bb"),
        case("E", "E", "G", "B"),
        case("F", "F", "Ab", "C"),
        case("F#", "F#", "A", "C#"),
        case("Gb", "Gb", "A", "Db"),
        case("G", "G", "Bb", "D"),
        case("G#", "G#", "B", "D#"),
        case("Ab", "Ab", "B", "Eb"),
        case("A", "A", "C", "E"),
        case("A#", "A#", "C#", "F"),
        case("Bb", "Bb", "Db", "F"),
        case("B", "B", "D", "F#")
    )]
    fn test_from_str_minor(
        #[values("m", "min")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        third: Note,
        fifth: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(chord.notes, vec![root, third, fifth]);
        assert_eq!(chord.chord_type, ChordType::Minor);
    }

    #[rstest(
        chord_base,
        root,
        third,
        fifth,
        seventh,
        case("C", "C", "Eb", "G", "Bb"),
        case("C#", "C#", "E", "G#", "B"),
        case("Db", "Db", "E", "Ab", "B"),
        case("D", "D", "F", "A", "C"),
        case("D#", "D#", "F#", "A#", "C#"),
        case("Eb", "Eb", "Gb", "Bb", "Db"),
        case("E", "E", "G", "B", "D"),
        case("F", "F", "Ab", "C", "Eb"),
        case("F#", "F#", "A", "C#", "E"),
        case("Gb", "Gb", "A", "Db", "E"),
        case("G", "G", "Bb", "D", "F"),
        case("G#", "G#", "B", "D#", "F#"),
        case("Ab", "Ab", "B", "Eb", "Gb"),
        case("A", "A", "C", "E", "G"),
        case("A#", "A#", "C#", "F", "G#"),
        case("Bb", "Bb", "Db", "F", "Ab"),
        case("B", "B", "D", "F#", "A")
    )]
    fn test_from_str_minor_seventh(
        #[values("m7", "min7")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        third: Note,
        fifth: Note,
        seventh: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(chord.notes, vec![root, third, fifth, seventh]);
        assert_eq!(chord.chord_type, ChordType::MinorSeventh);
    }

    #[rstest(
        chord_base,
        root,
        third,
        fifth,
        seventh,
        case("C", "C", "Eb", "G", "B"),
        case("C#", "C#", "E", "G#", "C"),
        case("Db", "Db", "E", "Ab", "C"),
        case("D", "D", "F", "A", "C#"),
        case("D#", "D#", "F#", "A#", "D"),
        case("Eb", "Eb", "Gb", "Bb", "D"),
        case("E", "E", "G", "B", "D#"),
        case("F", "F", "Ab", "C", "E"),
        case("F#", "F#", "A", "C#", "F"),
        case("Gb", "Gb", "A", "Db", "F"),
        case("G", "G", "Bb", "D", "F#"),
        case("G#", "G#", "B", "D#", "G"),
        case("Ab", "Ab", "B", "Eb", "G"),
        case("A", "A", "C", "E", "G#"),
        case("A#", "A#", "C#", "F", "A"),
        case("Bb", "Bb", "Db", "F", "A"),
        case("B", "B", "D", "F#", "A#")
    )]
    fn test_from_str_minor_major_seventh(
        #[values("mMaj7", "mM7", "minMaj7")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        third: Note,
        fifth: Note,
        seventh: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(chord.notes, vec![root, third, fifth, seventh]);
        assert_eq!(chord.chord_type, ChordType::MinorMajorSeventh);
    }

    #[rstest(
        chord_base,
        root,
        third,
        fifth,
        sixth,
        case("C", "C", "Eb", "G", "A"),
        case("C#", "C#", "E", "G#", "A#"),
        case("Db", "Db", "E", "Ab", "Bb"),
        case("D", "D", "F", "A", "B"),
        case("D#", "D#", "F#", "A#", "C"),
        case("Eb", "Eb", "Gb", "Bb", "C"),
        case("E", "E", "G", "B", "C#"),
        case("F", "F", "Ab", "C", "D"),
        case("F#", "F#", "A", "C#", "D#"),
        case("Gb", "Gb", "A", "Db", "Eb"),
        case("G", "G", "Bb", "D", "E"),
        case("G#", "G#", "B", "D#", "F"),
        case("Ab", "Ab", "B", "Eb", "F"),
        case("A", "A", "C", "E", "F#"),
        case("A#", "A#", "C#", "F", "G"),
        case("Bb", "Bb", "Db", "F", "G"),
        case("B", "B", "D", "F#", "G#")
    )]
    fn test_from_str_minor_sixth(
        #[values("m6", "min6")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        third: Note,
        fifth: Note,
        sixth: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(chord.notes, vec![root, third, fifth, sixth]);
        assert_eq!(chord.chord_type, ChordType::MinorSixth);
    }

    #[rstest(
        chord_base,
        root,
        third,
        fifth,
        seventh,
        ninth,
        case("C", "C", "Eb", "G", "Bb", "D"),
        case("C#", "C#", "E", "G#", "B", "D#"),
        case("Db", "Db", "E", "Ab", "B", "Eb"),
        case("D", "D", "F", "A", "C", "E"),
        case("D#", "D#", "F#", "A#", "C#", "F"),
        case("Eb", "Eb", "Gb", "Bb", "Db", "F"),
        case("E", "E", "G", "B", "D", "F#"),
        case("F", "F", "Ab", "C", "Eb", "G"),
        case("F#", "F#", "A", "C#", "E", "G#"),
        case("Gb", "Gb", "A", "Db", "E", "Ab"),
        case("G", "G", "Bb", "D", "F", "A"),
        case("G#", "G#", "B", "D#", "F#", "A#"),
        case("Ab", "Ab", "B", "Eb", "Gb", "Bb"),
        case("A", "A", "C", "E", "G", "B"),
        case("A#", "A#", "C#", "F", "G#", "C"),
        case("Bb", "Bb", "Db", "F", "Ab", "C"),
        case("B", "B", "D", "F#", "A", "C#")
    )]
    fn test_from_str_minor_ninth(
        #[values("m9", "min9")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        third: Note,
        fifth: Note,
        seventh: Note,
        ninth: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(chord.notes, vec![root, third, fifth, seventh, ninth]);
        assert_eq!(chord.chord_type, ChordType::MinorNinth);
    }

    #[rstest(
        chord_base,
        root,
        third,
        fifth,
        seventh,
        ninth,
        eleventh,
        case("C", "C", "Eb", "G", "Bb", "D", "F"),
        case("C#", "C#", "E", "G#", "B", "D#", "F#"),
        case("Db", "Db", "E", "Ab", "B", "Eb", "Gb"),
        case("D", "D", "F", "A", "C", "E", "G"),
        case("D#", "D#", "F#", "A#", "C#", "F", "G#"),
        case("Eb", "Eb", "Gb", "Bb", "Db", "F", "Ab"),
        case("E", "E", "G", "B", "D", "F#", "A"),
        case("F", "F", "Ab", "C", "Eb", "G", "A#"),
        case("F#", "F#", "A", "C#", "E", "G#", "B"),
        case("Gb", "Gb", "A", "Db", "E", "Ab", "B"),
        case("G", "G", "Bb", "D", "F", "A", "C"),
        case("G#", "G#", "B", "D#", "F#", "A#", "C#"),
        case("Ab", "Ab", "B", "Eb", "Gb", "Bb", "Db"),
        case("A", "A", "C", "E", "G", "B", "D"),
        case("A#", "A#", "C#", "F", "G#", "C", "D#"),
        case("Bb", "Bb", "Db", "F", "Ab", "C", "Eb"),
        case("B", "B", "D", "F#", "A", "C#", "E")
    )]
    fn test_from_str_minor_eleventh(
        #[values("m11", "min11")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        third: Note,
        fifth: Note,
        seventh: Note,
        ninth: Note,
        eleventh: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(
            chord.notes,
            vec![root, third, fifth, seventh, ninth, eleventh]
        );
        assert_eq!(chord.chord_type, ChordType::MinorEleventh);
    }

    #[rstest(
        chord_base,
        root,
        third,
        fifth,
        seventh,
        ninth,
        eleventh,
        thirteenth,
        case("C", "C", "Eb", "G", "Bb", "D", "F", "A"),
        case("C#", "C#", "E", "G#", "B", "D#", "F#", "A#"),
        case("Db", "Db", "E", "Ab", "B", "Eb", "Gb", "Bb"),
        case("D", "D", "F", "A", "C", "E", "G", "B"),
        case("D#", "D#", "F#", "A#", "C#", "F", "G#", "C"),
        case("Eb", "Eb", "Gb", "Bb", "Db", "F", "Ab", "C"),
        case("E", "E", "G", "B", "D", "F#", "A", "C#"),
        case("F", "F", "Ab", "C", "Eb", "G", "A#", "D"),
        case("F#", "F#", "A", "C#", "E", "G#", "B", "D#"),
        case("Gb", "Gb", "A", "Db", "E", "Ab", "B", "Eb"),
        case("G", "G", "Bb", "D", "F", "A", "C", "E"),
        case("G#", "G#", "B", "D#", "F#", "A#", "C#", "F"),
        case("Ab", "Ab", "B", "Eb", "Gb", "Bb", "Db", "F"),
        case("A", "A", "C", "E", "G", "B", "D", "F#"),
        case("A#", "A#", "C#", "F", "G#", "C", "D#", "G"),
        case("Bb", "Bb", "Db", "F", "Ab", "C", "Eb", "G"),
        case("B", "B", "D", "F#", "A", "C#", "E", "G#")
    )]
    fn test_from_str_minor_thirteenth(
        #[values("m13", "min13")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        third: Note,
        fifth: Note,
        seventh: Note,
        ninth: Note,
        eleventh: Note,
        thirteenth: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(
            chord.notes,
            vec![root, third, fifth, seventh, ninth, eleventh, thirteenth]
        );
        assert_eq!(chord.chord_type, ChordType::MinorThirteenth);
    }

    #[rstest(
        chord_base,
        root,
        third,
        fifth,
        case("C", "C", "Eb", "Gb"),
        case("C#", "C#", "E", "G"),
        case("Db", "Db", "E", "G"),
        case("D", "D", "F", "Ab"),
        case("D#", "D#", "F#", "A"),
        case("Eb", "Eb", "Gb", "A"),
        case("E", "E", "G", "Bb"),
        case("F", "F", "Ab", "B"),
        case("F#", "F#", "A", "C"),
        case("Gb", "Gb", "A", "C"),
        case("G", "G", "Bb", "Db"),
        case("G#", "G#", "B", "D"),
        case("Ab", "Ab", "B", "D"),
        case("A", "A", "C", "Eb"),
        case("A#", "A#", "C#", "E"),
        case("Bb", "Bb", "Db", "E"),
        case("B", "B", "D", "F")
    )]
    fn test_from_str_diminished(
        #[values("dim", "o")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        third: Note,
        fifth: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(chord.notes, vec![root, third, fifth]);
        assert_eq!(chord.chord_type, ChordType::Diminished);
    }

    #[rstest(
        chord_base,
        root,
        third,
        fifth,
        seventh,
        case("C", "C", "Eb", "Gb", "A"),
        case("C#", "C#", "E", "G", "Bb"),
        case("Db", "Db", "E", "G", "Bb"),
        case("D", "D", "F", "Ab", "B"),
        case("D#", "D#", "F#", "A", "C"),
        case("Eb", "Eb", "Gb", "A", "C"),
        case("E", "E", "G", "Bb", "Db"),
        case("F", "F", "Ab", "B", "D"),
        case("F#", "F#", "A", "C", "Eb"),
        case("Gb", "Gb", "A", "C", "Eb"),
        case("G", "G", "Bb", "Db", "E"),
        case("G#", "G#", "B", "D", "F"),
        case("Ab", "Ab", "B", "D", "F"),
        case("A", "A", "C", "Eb", "Gb"),
        case("A#", "A#", "C#", "E", "G"),
        case("Bb", "Bb", "Db", "E", "G"),
        case("B", "B", "D", "F", "Ab")
    )]
    fn test_from_str_diminished_seventh(
        #[values("dim7", "o7")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        third: Note,
        fifth: Note,
        seventh: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(chord.notes, vec![root, third, fifth, seventh]);
        assert_eq!(chord.chord_type, ChordType::DiminishedSeventh);
    }

    #[rstest(
        chord_base,
        root,
        third,
        fifth,
        seventh,
        case("C", "C", "Eb", "Gb", "Bb"),
        case("C#", "C#", "E", "G", "B"),
        case("Db", "Db", "E", "G", "B"),
        case("D", "D", "F", "Ab", "C"),
        case("D#", "D#", "F#", "A", "C#"),
        case("Eb", "Eb", "Gb", "A", "Db"),
        case("E", "E", "G", "Bb", "D"),
        case("F", "F", "Ab", "B", "Eb"),
        case("F#", "F#", "A", "C", "E"),
        case("Gb", "Gb", "A", "C", "E"),
        case("G", "G", "Bb", "Db", "F"),
        case("G#", "G#", "B", "D", "F#"),
        case("Ab", "Ab", "B", "D", "Gb"),
        case("A", "A", "C", "Eb", "G"),
        case("A#", "A#", "C#", "E", "G#"),
        case("Bb", "Bb", "Db", "E", "Ab"),
        case("B", "B", "D", "F", "A")
    )]
    fn test_from_str_half_diminished_seventh(
        #[values("m7b5", "ø", "ø7")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        third: Note,
        fifth: Note,
        seventh: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(chord.notes, vec![root, third, fifth, seventh]);
        assert_eq!(chord.chord_type, ChordType::HalfDiminishedSeventh);
    }

    #[rstest(
        chord_base,
        root,
        fifth,
        case("C", "C", "G"),
        case("C#", "C#", "G#"),
        case("Db", "Db", "Ab"),
        case("D", "D", "A"),
        case("D#", "D#", "A#"),
        case("Eb", "Eb", "Bb"),
        case("E", "E", "B"),
        case("F", "F", "C"),
        case("F#", "F#", "C#"),
        case("Gb", "Gb", "Db"),
        case("G", "G", "D"),
        case("G#", "G#", "D#"),
        case("Ab", "Ab", "Eb"),
        case("A", "A", "E"),
        case("A#", "A#", "F"),
        case("Bb", "Bb", "F"),
        case("B", "B", "F#")
    )]
    fn test_from_str_fifth(
        #[values("5")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        fifth: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(chord.notes, vec![root, fifth]);
        assert_eq!(chord.chord_type, ChordType::Fifth);
    }

    #[rstest(
        chord_base,
        root,
        third,
        fifth,
        case("C", "C", "E", "G#"),
        case("C#", "C#", "F", "A"),
        case("Db", "Db", "F", "A"),
        case("D", "D", "F#", "A#"),
        case("D#", "D#", "G", "B"),
        case("Eb", "Eb", "G", "B"),
        case("E", "E", "G#", "C"),
        case("F", "F", "A", "C#"),
        case("F#", "F#", "A#", "D"),
        case("Gb", "Gb", "Bb", "D"),
        case("G", "G", "B", "D#"),
        case("G#", "G#", "C", "E"),
        case("Ab", "Ab", "C", "E"),
        case("A", "A", "C#", "F"),
        case("A#", "A#", "D", "F#"),
        case("Bb", "Bb", "D", "F#"),
        case("B", "B", "D#", "G")
    )]
    fn test_from_str_augmented(
        #[values("aug", "+")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        third: Note,
        fifth: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(chord.notes, vec![root, third, fifth]);
        assert_eq!(chord.chord_type, ChordType::Augmented);
    }

    #[rstest(
        chord_base,
        root,
        third,
        fifth,
        seventh,
        case("C", "C", "E", "G#", "Bb"),
        case("C#", "C#", "F", "A", "B"),
        case("Db", "Db", "F", "A", "B"),
        case("D", "D", "F#", "A#", "C"),
        case("D#", "D#", "G", "B", "C#"),
        case("Eb", "Eb", "G", "B", "Db"),
        case("E", "E", "G#", "C", "D"),
        case("F", "F", "A", "C#", "Eb"),
        case("F#", "F#", "A#", "D", "E"),
        case("Gb", "Gb", "Bb", "D", "E"),
        case("G", "G", "B", "D#", "F"),
        case("G#", "G#", "C", "E", "F#"),
        case("Ab", "Ab", "C", "E", "Gb"),
        case("A", "A", "C#", "F", "G"),
        case("A#", "A#", "D", "F#", "G#"),
        case("Bb", "Bb", "D", "F#", "Ab"),
        case("B", "B", "D#", "G", "A")
    )]
    fn test_from_str_augmented_seventh(
        #[values("aug7", "+7", "7#5")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        third: Note,
        fifth: Note,
        seventh: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(chord.notes, vec![root, third, fifth, seventh]);
        assert_eq!(chord.chord_type, ChordType::AugmentedSeventh);
    }

    #[rstest(
        chord_base,
        root,
        third,
        fifth,
        seventh,
        case("C", "C", "E", "G#", "B"),
        case("C#", "C#", "F", "A", "C"),
        case("Db", "Db", "F", "A", "C"),
        case("D", "D", "F#", "A#", "C#"),
        case("D#", "D#", "G", "B", "D"),
        case("Eb", "Eb", "G", "B", "D"),
        case("E", "E", "G#", "C", "D#"),
        case("F", "F", "A", "C#", "E"),
        case("F#", "F#", "A#", "D", "F"),
        case("Gb", "Gb", "Bb", "D", "F"),
        case("G", "G", "B", "D#", "F#"),
        case("G#", "G#", "C", "E", "G"),
        case("Ab", "Ab", "C", "E", "G"),
        case("A", "A", "C#", "F", "G#"),
        case("A#", "A#", "D", "F#", "A"),
        case("Bb", "Bb", "D", "F#", "A"),
        case("B", "B", "D#", "G", "A#")
    )]
    fn test_from_str_augmented_major_seventh(
        #[values("augMaj7", "+M7")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        third: Note,
        fifth: Note,
        seventh: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(chord.notes, vec![root, third, fifth, seventh]);
        assert_eq!(chord.chord_type, ChordType::AugmentedMajorSeventh);
    }

    #[rstest(
        chord_base,
        root,
        third,
        fifth,
        ninth,
        case("C", "C", "E", "G", "D"),
        case("C#", "C#", "F", "G#", "D#"),
        case("Db", "Db", "F", "Ab", "Eb"),
        case("D", "D", "F#", "A", "E"),
        case("D#", "D#", "G", "A#", "F"),
        case("Eb", "Eb", "G", "Bb", "F"),
        case("E", "E", "G#", "B", "F#"),
        case("F", "F", "A", "C", "G"),
        case("F#", "F#", "A#", "C#", "G#"),
        case("Gb", "Gb", "Bb", "Db", "Ab"),
        case("G", "G", "B", "D", "A"),
        case("G#", "G#", "C", "D#", "A#"),
        case("Ab", "Ab", "C", "Eb", "Bb"),
        case("A", "A", "C#", "E", "B"),
        case("A#", "A#", "D", "F", "C"),
        case("Bb", "Bb", "D", "F", "C"),
        case("B", "B", "D#", "F#", "C#")
    )]
    fn test_from_str_added_ninth(
        #[values("add9", "add2")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        third: Note,
        fifth: Note,
        ninth: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(chord.notes, vec![root, third, fifth, ninth]);
        assert_eq!(chord.chord_type, ChordType::AddedNinth);
    }

    #[rstest(
        chord_base,
        root,
        third,
        fourth,
        fifth,
        case("C", "C", "E", "F", "G"),
        case("C#", "C#", "F", "F#", "G#"),
        case("Db", "Db", "F", "Gb", "Ab"),
        case("D", "D", "F#", "G", "A"),
        case("D#", "D#", "G", "G#", "A#"),
        case("Eb", "Eb", "G", "Ab", "Bb"),
        case("E", "E", "G#", "A", "B"),
        case("F", "F", "A", "Bb", "C"),
        case("F#", "F#", "A#", "B", "C#"),
        case("Gb", "Gb", "Bb", "B", "Db"),
        case("G", "G", "B", "C", "D"),
        case("G#", "G#", "C", "C#", "D#"),
        case("Ab", "Ab", "C", "Db", "Eb"),
        case("A", "A", "C#", "D", "E"),
        case("A#", "A#", "D", "D#", "F"),
        case("Bb", "Bb", "D", "Eb", "F"),
        case("B", "B", "D#", "E", "F#")
    )]
    fn test_from_str_added_fourth(
        #[values("add4")] chord_suffix: &str,
        chord_base: &str,
        root: Note,
        third: Note,
        fourth: Note,
        fifth: Note,
    ) {
        let chord = Chord::from_str(&format!("{}{}", chord_base, chord_suffix)).unwrap();

        assert_eq!(chord.notes, vec![root, third, fourth, fifth]);
        assert_eq!(chord.chord_type, ChordType::AddedFourth);
    }

    #[rstest(
        pitches,
        chord,
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
    fn test_get_chord_type(pitches: Vec<PitchClass>, chord: Chord) {
        assert_eq!(Chord::try_from(&pitches[..]).unwrap(), chord);
    }

    #[rstest(
        chord1,
        n,
        chord2,
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
    fn test_add_semitones(chord1: Chord, n: Semitones, chord2: Chord) {
        assert_eq!(chord1 + n, chord2);
    }

    #[rstest(
        chord1,
        n,
        chord2,
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
    fn test_subtract_semitones(chord1: Chord, n: Semitones, chord2: Chord) {
        assert_eq!(chord1 - n, chord2);
    }

    #[rstest(
        chord1,
        n,
        chord2,
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
        case("Ab", 12, "Ab"),
        case("Cm", -1, "Bm"),
        case("Cmaj7", -2, "Bbmaj7"),
        case("Adim", -3, "Gbdim"),
        case("A", -12, "A"),
        case("A#", -12, "A#"),
        case("Ab", -12, "Ab")
    )]
    fn test_transpose(chord1: Chord, n: i8, chord2: Chord) {
        assert_eq!(chord1.transpose(n), chord2);
    }

    #[rstest(
        chord,
        played_notes,
        case("C", vec!["C", "E", "G"]),
        case("C7", vec!["C", "E", "Bb", "G"]),
        case("C11", vec!["C", "E", "Bb", "F"]),
        case("C13", vec!["C", "E", "Bb", "A"]),
    )]
    fn test_played_notes(chord: Chord, played_notes: Vec<&str>) {
        let pn1: Vec<_> = chord.played_notes().collect();
        let pn2: Vec<_> = played_notes
            .iter()
            .map(|&s| Note::from_str(s).unwrap())
            .collect();

        assert_eq!(pn1, pn2);
    }
}
