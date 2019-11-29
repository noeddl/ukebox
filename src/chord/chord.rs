use crate::chord::ChordShapeSet;
use crate::chord::FretID;
use crate::chord::Tuning;
use crate::diagram::ChordDiagram;
use crate::note::Interval;
use crate::note::Note;
use crate::STRING_COUNT;
use regex::Regex;
use std::fmt;
use std::str::FromStr;

/// Custom error for strings that cannot be parsed into chords.
#[derive(Debug)]
pub struct ParseChordError {
    name: String,
}

impl fmt::Display for ParseChordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse chord name \"{}\"", self.name)
    }
}

/// The type of the chord depending on the intervals it contains.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChordType {
    Major,
    Minor,
    Augmented,
    Diminished,
    DominantSeventh,
    MinorSeventh,
    MajorSeventh,
    AugmentedSeventh,
    DiminishedSeventh,
}

impl ChordType {
    fn get_intervals(self) -> Vec<Interval> {
        let interval_names = match self {
            Self::Major => vec!["P1", "M3", "P5"],
            Self::Minor => vec!["P1", "m3", "P5"],
            Self::Augmented => vec!["P1", "M3", "A5"],
            Self::Diminished => vec!["P1", "m3", "d5"],
            Self::DominantSeventh => vec!["P1", "M3", "P5", "m7"],
            Self::MinorSeventh => vec!["P1", "m3", "P5", "m7"],
            Self::MajorSeventh => vec!["P1", "M3", "P5", "M7"],
            Self::AugmentedSeventh => vec!["P1", "M3", "A5", "m7"],
            Self::DiminishedSeventh => vec!["P1", "m3", "d5", "d7"],
        };

        interval_names
            .iter()
            .map(|s| Interval::from_str(s).unwrap())
            .collect()
    }
}

impl fmt::Display for ChordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Major => "major",
            Self::Minor => "minor",
            Self::Augmented => "augmented",
            Self::Diminished => "diminished",
            Self::DominantSeventh => "dominant 7th",
            Self::MinorSeventh => "minor 7th",
            Self::MajorSeventh => "major 7th",
            Self::AugmentedSeventh => "augmented 7th",
            Self::DiminishedSeventh => "diminished 7th",
        };

        write!(f, "{}", s)
    }
}

/// A chord such as C, Cm and so on.
pub struct Chord {
    name: String,
    pub chord_type: ChordType,
    pub root: Note,
    notes: Vec<Note>,
}

impl Chord {
    pub fn contains(&self, note: Note) -> bool {
        self.notes.contains(&note)
    }

    pub fn get_diagram(self, min_fret: FretID, tuning: Tuning) -> ChordDiagram {
        let chord_shapes = ChordShapeSet::new(self.chord_type);

        let (frets, intervals) = chord_shapes.get_config(self.root, min_fret, tuning);

        let mut notes = [self.root; STRING_COUNT];

        for (i, interval) in intervals.iter().enumerate() {
            notes[i] = notes[i] + *interval;
        }

        ChordDiagram::new(self, frets, notes, tuning)
    }
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {} {}", self.name, self.notes[0], self.chord_type)
    }
}

impl FromStr for Chord {
    type Err = ParseChordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = s.to_string();

        // Regular expression for chord names.
        let re = Regex::new(r"(?P<root>[CDEFGAB][#b]?)(?P<type>aug7?|dim7?|maj7|m?7?)").unwrap();

        // Match regex.
        let caps = match re.captures(s) {
            Some(caps) => caps,
            None => return Err(ParseChordError { name }),
        };

        // Get root note.
        let root = match Note::from_str(&caps["root"]) {
            Ok(note) => note,
            Err(_) => return Err(ParseChordError { name }),
        };

        // Get chord type.
        let chord_type = match &caps["type"] {
            "m" => ChordType::Minor,
            "aug" => ChordType::Augmented,
            "dim" => ChordType::Diminished,
            "7" => ChordType::DominantSeventh,
            "m7" => ChordType::MinorSeventh,
            "maj7" => ChordType::MajorSeventh,
            "aug7" => ChordType::AugmentedSeventh,
            "dim7" => ChordType::DiminishedSeventh,
            _ => ChordType::Major,
        };

        // Collect notes of the chord.
        let mut notes = vec![];

        for interval in chord_type.get_intervals() {
            notes.push(root + interval);
        }

        Ok(Self {
            name,
            root,
            chord_type,
            notes,
        })
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::many_single_char_names)]
    use super::*;
    use rstest::rstest_parametrize;

    #[rstest_parametrize(
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
        assert_eq!(c.notes, vec![r, t, f]);
        assert_eq!(c.chord_type, ChordType::Major);
    }

    #[rstest_parametrize(
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
        assert_eq!(c.notes, vec![r, t, f]);
        assert_eq!(c.chord_type, ChordType::Minor);
    }

    #[rstest_parametrize(
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
        assert_eq!(c.notes, vec![r, t, f]);
        assert_eq!(c.chord_type, ChordType::Augmented);
    }

    #[rstest_parametrize(
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
        assert_eq!(c.notes, vec![r, t, f]);
        assert_eq!(c.chord_type, ChordType::Diminished);
    }

    #[rstest_parametrize(
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
        assert_eq!(c.notes, vec![r, t, f, s]);
        assert_eq!(c.chord_type, ChordType::DominantSeventh);
    }

    #[rstest_parametrize(
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
        assert_eq!(c.notes, vec![r, t, f, s]);
        assert_eq!(c.chord_type, ChordType::MinorSeventh);
    }

    #[rstest_parametrize(
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
        assert_eq!(c.notes, vec![r, t, f, s]);
        assert_eq!(c.chord_type, ChordType::MajorSeventh);
    }

    #[rstest_parametrize(
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
        assert_eq!(c.notes, vec![r, t, f, s]);
        assert_eq!(c.chord_type, ChordType::AugmentedSeventh);
    }

    #[rstest_parametrize(
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
        assert_eq!(c.notes, vec![r, t, f, s]);
        assert_eq!(c.chord_type, ChordType::DiminishedSeventh);
    }

    #[rstest_parametrize(
        chord,
        note,
        contains,
        case("C", "C", true),
        case("C", "E", true),
        case("C", "D", false)
    )]
    fn test_contains(chord: &str, note: &str, contains: bool) {
        let c = Chord::from_str(chord).unwrap();
        let n = Note::from_str(note).unwrap();
        assert_eq!(c.contains(n), contains);
    }
}
