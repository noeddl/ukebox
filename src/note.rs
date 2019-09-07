use std::collections::HashMap;
use std::fmt;
use std::ops::Add;
use std::str::FromStr;

const PITCH_CLASS_COUNT: usize = 12;

lazy_static! {
    /// Mapping of note names to pitch classes.
    static ref NOTE_CLASSES: HashMap<&'static str, usize> = [
        ("C", 0), ("C#", 1), ("Db", 1), ("D", 2),("D#", 3),
        ("Eb", 3), ("E", 4), ("F", 5), ("F#", 6), ("Gb", 6),
        ("G", 7), ("G#", 8), ("Ab", 8), ("A", 9), ("A#", 10),
        ("Bb", 10), ("B", 11)
    ].iter().cloned().collect();
}

/// Custom error for strings that cannot be parsed into notes.
#[derive(Debug)]
pub struct ParseNoteError {
    name: String,
}

impl fmt::Display for ParseNoteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse note name \"{}\"", self.name)
    }
}

/// A pitch class is "a set of all pitches that are a whole number of octaves
/// apart, e.g., the pitch class C consists of the Cs in all octaves."
/// https://en.wikipedia.org/wiki/Pitch_class
///
/// Our 12 pitch classes are represented with integers from 0 to 11.
/// Values > 11 are also possible, they will be used to model retrieval
/// of the same pitch class in a higher octave.
/// For example, pitch class 12 is the same as pitch class 0 and corresponds
/// to the pitch class of C.
#[derive(Debug, Clone, Copy, PartialEq)]
enum PitchClass {
    Value(usize),
}

impl Add<usize> for PitchClass {
    type Output = Self;

    /// Get the pitch class that is `n` semitones higher than the current
    /// pitch class.
    fn add(self, n: usize) -> Self {
        let value = match self {
            Self::Value(i) => i + n,
        };
        Self::Value(value)
    }
}

/// A note such a C, C# and so on.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Note {
    pitch_class: PitchClass,
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let PitchClass::Value(value) = self.pitch_class;

        let s = match value {
            0 => "C",
            1 => "C#/Db",
            2 => "D",
            3 => "D#/Eb",
            4 => "E",
            5 => "F",
            6 => "F#/Gb",
            7 => "G",
            8 => "G#/Ab",
            9 => "A",
            10 => "A#/Bb",
            11 => "B",
            _ => panic!(format!("Pitch class {} is out of range", value)),
        };

        write!(f, "{}", s)
    }
}

impl FromStr for Note {
    type Err = ParseNoteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = s.to_string();

        let pitch_class = match NOTE_CLASSES.get(s) {
            Some(value) => PitchClass::Value(*value),
            None => return Err(ParseNoteError { name }),
        };

        Ok(Self { pitch_class })
    }
}

impl From<usize> for Note {
    fn from(n: usize) -> Self {
        // If i > 11, cycle the list of notes as often as necessary to retrieve
        // a note in a higher octave, e.g. index 12 corresponds to 0 (as does
        // 24, 36, ... In practice, it will however probably not be necessary to
        // go so far.)
        let i = n % PITCH_CLASS_COUNT;
        Self {
            pitch_class: PitchClass::Value(i),
        }
    }
}

impl From<PitchClass> for Note {
    fn from(pitch_class: PitchClass) -> Self {
        let PitchClass::Value(value) = pitch_class;
        Self::from(value)
    }
}

impl Add<usize> for Note {
    type Output = Self;

    /// Get the note that is `n` semitones higher than the current note.
    fn add(self, n: usize) -> Self {
        let pc = self.pitch_class + n;
        Note::from(pc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest_parametrize;

    #[rstest_parametrize(
        s,
        pc,
        case("C", 0),
        case("C#", 1),
        case("Db", 1),
        case("D", 2),
        case("D#", 3),
        case("Eb", 3),
        case("E", 4),
        case("F", 5),
        case("F#", 6),
        case("Gb", 6),
        case("G", 7),
        case("G#", 8),
        case("Ab", 8),
        case("A", 9),
        case("A#", 10),
        case("Bb", 10),
        case("B", 11)
    )]
    fn test_from_str(s: &str, pc: usize) {
        let n = Note::from_str(s).unwrap();
        let PitchClass::Value(value) = n.pitch_class;
        assert_eq!(value, pc);
    }

    #[rstest_parametrize(
        i,
        pc,
        case(0, 0),
        case(1, 1),
        case(2, 2),
        case(3, 3),
        case(4, 4),
        case(5, 5),
        case(6, 6),
        case(7, 7),
        case(8, 8),
        case(9, 9),
        case(10, 10),
        case(11, 11),
        case(12, 0),
        case(13, 1),
        case(24, 0),
        case(325, 1)
    )]
    fn test_from_int(i: usize, pc: usize) {
        let n = Note::from(i);
        let PitchClass::Value(value) = n.pitch_class;
        assert_eq!(value, pc);
    }

    #[rstest_parametrize(
        i,
        pc,
        case(0, 0),
        case(1, 1),
        case(2, 2),
        case(3, 3),
        case(4, 4),
        case(5, 5),
        case(6, 6),
        case(7, 7),
        case(8, 8),
        case(9, 9),
        case(10, 10),
        case(11, 11),
        case(12, 0),
        case(13, 1),
        case(24, 0),
        case(325, 1)
    )]
    fn test_from_pitch_class(i: usize, pc: usize) {
        let n = Note::from(PitchClass::Value(i));
        let PitchClass::Value(value) = n.pitch_class;
        assert_eq!(value, pc);
    }

    #[rstest_parametrize(
        pc,
        s,
        case(0, "C"),
        case(1, "C#/Db"),
        case(2, "D"),
        case(3, "D#/Eb"),
        case(4, "E"),
        case(5, "F"),
        case(6, "F#/Gb"),
        case(7, "G"),
        case(8, "G#/Ab"),
        case(9, "A"),
        case(10, "A#/Bb"),
        case(11, "B")
    )]
    fn test_display(pc: usize, s: &str) {
        let note = Note::from(pc);
        assert_eq!(format!("{}", note), s);
    }

    #[rstest_parametrize(
        pc,
        i,
        result,
        case(0, 0, 0),
        case(0, 1, 1),
        case(0, 10, 10),
        case(0, 12, 0),
        case(0, 13, 1),
        case(0, 24, 0)
    )]
    fn test_add_int(pc: usize, i: usize, result: usize) {
        let note = Note::from(pc);
        assert_eq!(note + i, Note::from(result));
    }

    #[rstest_parametrize(
        n,
        i,
        value,
        case(0, 0, 0),
        case(1, 1, 2),
        case(2, 3, 5),
        case(2, 10, 12),
        case(12, 10, 22)
    )]
    fn test_pitch_class_add_int(n: usize, i: usize, value: usize) {
        assert_eq!(PitchClass::Value(n) + i, PitchClass::Value(value));
    }
}
