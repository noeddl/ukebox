use crate::chord::ChordShapeSet;
use crate::diagram::ChordDiagram;
use crate::note::Interval;
use crate::note::Note;
use crate::Frets;
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

/// Chord quality.
/// https://en.wikipedia.org/wiki/Chord_names_and_symbols_(popular_music)#Chord_quality
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChordQuality {
    Major,
    Minor,
}

impl ChordQuality {
    fn get_intervals(self) -> Vec<Interval> {
        use Interval::*;

        match self {
            Self::Major => vec![PerfectUnison, MajorThird, PerfectFifth],
            Self::Minor => vec![PerfectUnison, MinorThird, PerfectFifth],
        }
    }
}

impl fmt::Display for ChordQuality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Major => "major",
            Self::Minor => "minor",
        };

        write!(f, "{}", s)
    }
}

/// A chord such as C, Cm and so on.
pub struct Chord {
    name: String,
    pub quality: ChordQuality,
    pub root: Note,
    notes: Vec<Note>,
}

impl Chord {
    pub fn contains(&self, note: Note) -> bool {
        self.notes.contains(&note)
    }

    pub fn get_diagram(self, min_fret: Frets) -> ChordDiagram {
        let chord_shapes = ChordShapeSet::new(self.quality);

        let (frets, intervals) = chord_shapes.get_config(self.root, min_fret);

        let mut notes = [self.root; STRING_COUNT];

        for (i, interval) in intervals.iter().enumerate() {
            notes[i] = notes[i] + *interval;
        }

        ChordDiagram::new(self, frets, notes)
    }
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {} {}", self.name, self.notes[0], self.quality)
    }
}

impl FromStr for Chord {
    type Err = ParseChordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = s.to_string();

        // Regular expression for chord names.
        let re = Regex::new(r"(?P<root>[CDEFGAB][#b]?)(?P<quality>m?)").unwrap();

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

        // Get chord quality.
        let quality = match &caps["quality"] {
            "m" => ChordQuality::Minor,
            _ => ChordQuality::Major,
        };

        // Collect notes of the chord.
        let mut notes = vec![];

        for interval in quality.get_intervals() {
            notes.push(root + interval);
        }

        Ok(Self {
            name,
            root,
            quality,
            notes,
        })
    }
}

#[cfg(test)]
mod tests {
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
        assert_eq!(c.quality, ChordQuality::Major);
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
        assert_eq!(c.quality, ChordQuality::Minor);
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
