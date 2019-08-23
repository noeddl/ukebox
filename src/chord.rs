use crate::note::Note;

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
    quality: ChordQuality,
    notes: Vec<Note<'a>>,
}

impl Chord<'_> {
    pub fn contains(&self, note: &Note) -> bool {
        self.notes.contains(note)
    }
}

impl<'a> From<&'a str> for Chord<'_> {
    fn from(s: &'a str) -> Self {
        let quality = ChordQuality::Major;
        let root = Note::from(s);
        let mut notes = vec![];

        for interval in quality.get_intervals() {
            notes.push(root + interval);
        }

        Self { quality, notes }
    }
}

#[cfg(test)]
mod tests {
    extern crate rstest;
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
        let c = Chord::from(chord);
        let r = Note::from(root);
        let t = Note::from(third);
        let f = Note::from(fifth);
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
        let c = Chord::from(chord);
        let n = Note::from(note);
        assert_eq!(c.contains(&n), contains);
    }
}
