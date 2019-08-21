use std::fmt;
use std::ops::Add;

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
///
/// There are several ways to access a specific note:
/// * Use a constant:
///   `C_SHARP`
/// * Convert a string into a Note:
///   `Note::from("C#")`
/// * Convert an integer into a Note:
///   `Note::from(1)`
/// * Convert a PitchClass into a Note:
///   `Note::from(PitchClass::Value(1))`
/// * Access the NOTES constant at the index that corresponds to the
///   note's pitch class:
///   `NOTES[1]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Note<'a> {
    pitch_class: PitchClass,
    name: &'a str,
    enharmonic_name: Option<&'a str>,
}

impl fmt::Display for Note<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.enharmonic_name {
            None => write!(f, "{}", self.name),
            Some(enh_name) => write!(f, "{}/{}", self.name, enh_name),
        }
    }
}

impl<'a> From<&'a str> for Note<'_> {
    fn from(s: &'a str) -> Self {
        match s {
            "C" => C_,
            "C#" => C_SHARP,
            "Db" => C_SHARP,
            "D" => D_,
            "D#" => D_SHARP,
            "Eb" => D_SHARP,
            "E" => E_,
            "F" => F_,
            "F#" => F_SHARP,
            "Gb" => F_SHARP,
            "G" => G_,
            "G#" => G_SHARP,
            "Ab" => G_SHARP,
            "A" => A_,
            "A#" => A_SHARP,
            "Bb" => A_SHARP,
            "B" => B_,
            _ => panic!("Invalid note specified"),
        }
    }
}

impl From<usize> for Note<'_> {
    fn from(n: usize) -> Self {
        // If i > 11, cycle the list of notes as often as necessary to retrieve
        // a note in a higher octave, e.g. index 12 corresponds to 0 (as does
        // 24, 36, ... In practice, it will however probably not be necessary to
        // go so far.)
        let i = n % NOTES.len();
        NOTES[i]
    }
}

impl From<PitchClass> for Note<'_> {
    fn from(pitch_class: PitchClass) -> Self {
        let PitchClass::Value(value) = pitch_class;
        Self::from(value)
    }
}

impl Add<usize> for Note<'_> {
    type Output = Self;

    /// Get the note that is `n` semitones higher than the current note.
    fn add(self, n: usize) -> Self {
        let pc = self.pitch_class + n;
        Note::from(pc)
    }
}

/// Macro to create Note constants C_, C_SHARP etc and the constant list NOTES
/// containing these Note constants.
macro_rules! note_consts {
    ($($value:expr, $id:ident, $name:expr, $enharmonic_name:expr;)*) => {
        $(
            #[doc = $name]
            pub const $id: Note = Note {
                pitch_class: PitchClass::Value($value),
                name: $name,
                enharmonic_name: $enharmonic_name,
            };
        )*

        /// A list of all of the notes.
        pub static NOTES: &[Note] = &[$($id, )*];
    }
}

note_consts! {
    0,  C_,         "C",    None;
    1,  C_SHARP,    "C#",   Some("Db");
    2,  D_,         "D",    None;
    3,  D_SHARP,    "D#",   Some("Eb");
    4,  E_,         "E",    None;
    5,  F_,         "F",    None;
    6,  F_SHARP,    "F#",   Some("Gb");
    7,  G_,         "G",    None;
    8,  G_SHARP,    "G#",   Some("Ab");
    9,  A_,         "A",    None;
    10, A_SHARP,    "A#",   Some("Bb");
    11, B_,         "B",    None;
}

#[cfg(test)]
mod tests {
    extern crate rstest;
    use super::*;
    use rstest::rstest_parametrize;

    #[rstest_parametrize(
        s,
        note,
        case("C", C_),
        case("C#", C_SHARP),
        case("Db", C_SHARP),
        case("D", D_),
        case("D#", D_SHARP),
        case("Eb", D_SHARP),
        case("E", E_),
        case("F", F_),
        case("F#", F_SHARP),
        case("Gb", F_SHARP),
        case("G", G_),
        case("G#", G_SHARP),
        case("Ab", G_SHARP),
        case("A", A_),
        case("A#", A_SHARP),
        case("Bb", A_SHARP),
        case("B", B_)
    )]
    fn test_from_str(s: &str, note: Note) {
        let n = Note::from(s);
        assert_eq!(n, note);
    }

    #[rstest_parametrize(
        i,
        note,
        case(0, C_),
        case(1, C_SHARP),
        case(2, D_),
        case(3, D_SHARP),
        case(4, E_),
        case(5, F_),
        case(6, F_SHARP),
        case(7, G_),
        case(8, G_SHARP),
        case(9, A_),
        case(10, A_SHARP),
        case(11, B_),
        case(12, C_),
        case(13, C_SHARP),
        case(24, C_),
        case(325, C_SHARP)
    )]
    fn test_from_int(i: usize, note: Note) {
        let n = Note::from(i);
        assert_eq!(n, note);
    }

    #[rstest_parametrize(
        i,
        note,
        case(0, C_),
        case(1, C_SHARP),
        case(2, D_),
        case(3, D_SHARP),
        case(4, E_),
        case(5, F_),
        case(6, F_SHARP),
        case(7, G_),
        case(8, G_SHARP),
        case(9, A_),
        case(10, A_SHARP),
        case(11, B_),
        case(12, C_),
        case(13, C_SHARP),
        case(24, C_),
        case(325, C_SHARP)
    )]
    fn test_from_pitch_class(i: usize, note: Note) {
        let n = Note::from(PitchClass::Value(i));
        assert_eq!(n, note);
    }

    #[rstest_parametrize(
        note,
        s,
        case(C_, "C"),
        case(C_SHARP, "C#/Db"),
        case(D_, "D"),
        case(D_SHARP, "D#/Eb"),
        case(E_, "E"),
        case(F_, "F"),
        case(F_SHARP, "F#/Gb"),
        case(G_, "G"),
        case(G_SHARP, "G#/Ab"),
        case(A_, "A"),
        case(A_SHARP, "A#/Bb"),
        case(B_, "B")
    )]
    fn test_display(note: Note, s: &str) {
        assert_eq!(format!("{}", note), s);
    }

    #[rstest_parametrize(
        note,
        i,
        result,
        case(C_, 0, C_),
        case(C_, 1, C_SHARP),
        case(C_, 10, A_SHARP),
        case(C_, 12, C_),
        case(C_, 13, C_SHARP),
        case(C_, 24, C_)
    )]
    fn test_add_int(note: Note, i: usize, result: Note) {
        assert_eq!(note + i, result);
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
