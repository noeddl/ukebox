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
    DominantSeventh,
    MinorSeventh,
    MajorSeventh,
}

impl ChordType {
    fn get_intervals(self) -> Vec<Interval> {
        use Interval::*;

        match self {
            Self::Major => vec![PerfectUnison, MajorThird, PerfectFifth],
            Self::Minor => vec![PerfectUnison, MinorThird, PerfectFifth],
            Self::DominantSeventh => vec![PerfectUnison, MajorThird, PerfectFifth, MinorSeventh],
            Self::MinorSeventh => vec![PerfectUnison, MinorThird, PerfectFifth, MinorSeventh],
            Self::MajorSeventh => vec![PerfectUnison, MajorThird, PerfectFifth, MajorSeventh],
        }
    }
}

impl fmt::Display for ChordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Major => "major",
            Self::Minor => "minor",
            Self::DominantSeventh => "dominant 7th",
            Self::MinorSeventh => "minor 7th",
            Self::MajorSeventh => "major 7th",
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
        let re = Regex::new(r"(?P<root>[CDEFGAB][#b]?)(?P<type>maj7|m?7?)").unwrap();

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
            "7" => ChordType::DominantSeventh,
            "m7" => ChordType::MinorSeventh,
            "maj7" => ChordType::MajorSeventh,
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
