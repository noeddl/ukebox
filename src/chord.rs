use crate::note::Note;
use std::fmt;
use std::str::FromStr;

/// Custom error for strings that cannot be parsed into chords.
#[derive(Debug)]
pub struct ParseChordError;

impl fmt::Display for ParseChordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse chord name")
    }
}

/// Chord quality.
/// https://en.wikipedia.org/wiki/Chord_names_and_symbols_(popular_music)#Chord_quality
#[derive(Debug, PartialEq)]
enum ChordQuality {
    Major,
}

impl ChordQuality {
    fn get_intervals(&self) -> Vec<usize> {
        match self {
            Self::Major => vec![0, 4, 7],
        }
    }
}

/// A chord such as C, Cm and so on.
pub struct Chord<'a> {
    name: String,
    quality: ChordQuality,
    notes: Vec<Note<'a>>,
}

impl Chord<'_> {
    pub fn contains(&self, note: &Note) -> bool {
        self.notes.contains(note)
    }
}

impl fmt::Display for Chord<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl FromStr for Chord<'_> {
    type Err = ParseChordError;

    // Parses a color hex code of the form '#rRgGbB..' into an
    // instance of 'RGB'
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let quality = ChordQuality::Major;

        let root = match Note::from_str(s) {
            Ok(note) => note,
            Err(_) => return Err(ParseChordError),
        };

        let mut notes = vec![];

        for interval in quality.get_intervals() {
            notes.push(root + interval);
        }

        let name = s.to_string();

        Ok(Self {
            name,
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
        note,
        contains,
        case("C", "C", true),
        case("C", "E", true),
        case("C", "D", false)
    )]
    fn test_contains(chord: &str, note: &str, contains: bool) {
        let c = Chord::from_str(chord).unwrap();
        let n = Note::from_str(note).unwrap();
        assert_eq!(c.contains(&n), contains);
    }
}
