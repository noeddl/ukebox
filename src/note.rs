#![allow(clippy::suspicious_arithmetic_impl)]
use crate::Frets;
use std::fmt;
use std::ops::Add;
use std::ops::Sub;
use std::str::FromStr;

/// Number of pitch classes.
const PITCH_CLASS_COUNT: Frets = 12;

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
/// Values > 11 will be used to model retrieval of the same pitch class in a
/// higher octave.
/// For example, pitch class 12 is the same as pitch class 0 and corresponds
/// to the pitch class of C.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PitchClass {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}

impl PitchClass {
    /// Convert an integer into a pitch class.
    ///
    /// To model the fact that e.g. all instances of the note `C` in different
    /// octaves belong to the same pitch class, each integer is placed in the
    /// range of potential pitch classes (between 0 and 11).
    /// For example, 12, 24, 36, etc. all correspond to pitch class 0.
    fn from_int(n: Frets) -> Self {
        use PitchClass::*;

        // Make sure we get a value between 0 and 11.
        let v = n % PITCH_CLASS_COUNT;

        // There does not seem to be a good way to turn integers into enum
        // variants without using external crates. Hardcoding the mapping
        // is not so elegant but at least readable.
        match v {
            0 => C,
            1 => CSharp,
            2 => D,
            3 => DSharp,
            4 => E,
            5 => F,
            6 => FSharp,
            7 => G,
            8 => GSharp,
            9 => A,
            10 => ASharp,
            11 => B,
            // Because of the modulo, `v` will always be in the correct range.
            _ => unreachable!(),
        }
    }
}

impl Add<Frets> for PitchClass {
    type Output = Self;

    /// Get the pitch class that is `n` semitones higher than the current
    /// pitch class.
    fn add(self, n: Frets) -> Self {
        let v = self as Frets + n;
        Self::from_int(v)
    }
}

impl Sub for PitchClass {
    type Output = Frets;

    /// Get the difference between two pitch classes in number of frets
    /// or semitones.
    ///
    /// `self` is assumed to always be higher as `other` with a difference
    /// of at most one octave.
    ///
    /// Examples:
    /// * D - C: both pitch classes are assumed to be in the same octave, D being
    ///          higher than C. The difference is 2.
    /// * D - A: D is higher than A, the difference is 5.
    fn sub(self, other: Self) -> Frets {
        let d = self as i8 - other as i8;

        let diff = match d {
            d if d >= 0 => d,
            _ => d + PITCH_CLASS_COUNT as i8,
        };

        diff as Frets
    }
}

impl Sub<Frets> for PitchClass {
    type Output = Self;

    /// Get the pitch class that is `n` semitones lower than the current
    /// pitch class.
    fn sub(self, n: Frets) -> Self {
        Self::from_int(self - Self::from_int(n))
    }
}

/// A note such a C, C# and so on.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Note {
    pitch_class: PitchClass,
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self.pitch_class {
            PitchClass::C => "C",
            PitchClass::CSharp => "C#/Db",
            PitchClass::D => "D",
            PitchClass::DSharp => "D#/Eb",
            PitchClass::E => "E",
            PitchClass::F => "F",
            PitchClass::FSharp => "F#/Gb",
            PitchClass::G => "G",
            PitchClass::GSharp => "G#/Ab",
            PitchClass::A => "A",
            PitchClass::ASharp => "A#/Bb",
            PitchClass::B => "B",
        };

        write!(f, "{}", s)
    }
}

impl FromStr for Note {
    type Err = ParseNoteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = s.to_string();

        let pitch_class = match s {
            "C" => PitchClass::C,
            "C#" => PitchClass::CSharp,
            "Db" => PitchClass::CSharp,
            "D" => PitchClass::D,
            "D#" => PitchClass::DSharp,
            "Eb" => PitchClass::DSharp,
            "E" => PitchClass::E,
            "F" => PitchClass::F,
            "F#" => PitchClass::FSharp,
            "Gb" => PitchClass::FSharp,
            "G" => PitchClass::G,
            "G#" => PitchClass::GSharp,
            "Ab" => PitchClass::GSharp,
            "A" => PitchClass::A,
            "A#" => PitchClass::ASharp,
            "Bb" => PitchClass::ASharp,
            "B" => PitchClass::B,
            _ => return Err(ParseNoteError { name }),
        };

        Ok(Self { pitch_class })
    }
}

impl From<PitchClass> for Note {
    fn from(pitch_class: PitchClass) -> Self {
        Self { pitch_class }
    }
}

impl Add<Frets> for Note {
    type Output = Self;

    /// Get the note that is `n` semitones higher than the current note.
    fn add(self, n: Frets) -> Self {
        let pc = self.pitch_class + n;
        Note::from(pc)
    }
}

impl Sub for Note {
    type Output = Frets;

    /// Get the difference between two notes in number of frets
    /// or semitones.
    fn sub(self, other: Self) -> Frets {
        self.pitch_class - other.pitch_class
    }
}

impl Sub<Frets> for Note {
    type Output = Self;

    /// Get the note that is `n` semitones lower than the current note.
    fn sub(self, n: Frets) -> Self {
        let pitch_class = self.pitch_class - n;
        Self { pitch_class }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest_parametrize;

    #[rstest_parametrize(
        s,
        pitch_class,
        case("C", PitchClass::C),
        case("C#", PitchClass::CSharp),
        case("Db", PitchClass::CSharp),
        case("D", PitchClass::D),
        case("D#", PitchClass::DSharp),
        case("Eb", PitchClass::DSharp),
        case("E", PitchClass::E),
        case("F", PitchClass::F),
        case("F#", PitchClass::FSharp),
        case("Gb", PitchClass::FSharp),
        case("G", PitchClass::G),
        case("G#", PitchClass::GSharp),
        case("Ab", PitchClass::GSharp),
        case("A", PitchClass::A),
        case("A#", PitchClass::ASharp),
        case("Bb", PitchClass::ASharp),
        case("B", PitchClass::B)
    )]
    fn test_from_str(s: &str, pitch_class: PitchClass) {
        let note = Note::from_str(s).unwrap();
        assert_eq!(note.pitch_class, pitch_class);
    }

    #[rstest_parametrize(
        n,
        pitch_class,
        case(0, PitchClass::C),
        case(1, PitchClass::CSharp),
        case(2, PitchClass::D),
        case(3, PitchClass::DSharp),
        case(4, PitchClass::E),
        case(5, PitchClass::F),
        case(6, PitchClass::FSharp),
        case(7, PitchClass::G),
        case(8, PitchClass::GSharp),
        case(9, PitchClass::A),
        case(10, PitchClass::ASharp),
        case(11, PitchClass::B),
        case(12, PitchClass::C),
        case(13, PitchClass::CSharp),
        case(24, PitchClass::C),
        case(127, PitchClass::G),
        case(255, PitchClass::DSharp)
    )]
    fn test_pitch_class_from_int(n: Frets, pitch_class: PitchClass) {
        assert_eq!(PitchClass::from_int(n), pitch_class);
    }

    #[rstest_parametrize(
        pitch_class,
        case(PitchClass::C),
        case(PitchClass::CSharp),
        case(PitchClass::D),
        case(PitchClass::DSharp),
        case(PitchClass::E),
        case(PitchClass::F),
        case(PitchClass::FSharp),
        case(PitchClass::G),
        case(PitchClass::GSharp),
        case(PitchClass::A),
        case(PitchClass::ASharp),
        case(PitchClass::B)
    )]
    fn test_from_pitch_class(pitch_class: PitchClass) {
        let note = Note::from(pitch_class);
        assert_eq!(note.pitch_class, pitch_class);
    }

    #[rstest_parametrize(
        pitch_class,
        s,
        case(PitchClass::C, "C"),
        case(PitchClass::CSharp, "C#/Db"),
        case(PitchClass::D, "D"),
        case(PitchClass::DSharp, "D#/Eb"),
        case(PitchClass::E, "E"),
        case(PitchClass::F, "F"),
        case(PitchClass::FSharp, "F#/Gb"),
        case(PitchClass::G, "G"),
        case(PitchClass::GSharp, "G#/Ab"),
        case(PitchClass::A, "A"),
        case(PitchClass::ASharp, "A#/Bb"),
        case(PitchClass::B, "B")
    )]
    fn test_display(pitch_class: PitchClass, s: &str) {
        let note = Note::from(pitch_class);
        assert_eq!(format!("{}", note), s);
    }

    #[rstest_parametrize(
        pitch_class,
        n,
        result,
        case(PitchClass::C, 0, PitchClass::C),
        case(PitchClass::C, 1, PitchClass::CSharp),
        case(PitchClass::C, 10, PitchClass::ASharp),
        case(PitchClass::C, 12, PitchClass::C),
        case(PitchClass::C, 13, PitchClass::CSharp),
        case(PitchClass::C, 24, PitchClass::C)
    )]
    fn test_add_int(pitch_class: PitchClass, n: Frets, result: PitchClass) {
        let note = Note::from(pitch_class);
        assert_eq!(note + n, Note::from(result));
    }

    #[rstest_parametrize(
        pitch_class,
        n,
        result,
        case(PitchClass::C, 0, PitchClass::C),
        case(PitchClass::C, 1, PitchClass::CSharp),
        case(PitchClass::C, 10, PitchClass::ASharp),
        case(PitchClass::C, 12, PitchClass::C),
        case(PitchClass::C, 13, PitchClass::CSharp),
        case(PitchClass::C, 24, PitchClass::C)
    )]
    fn test_pitch_class_add_int(pitch_class: PitchClass, n: Frets, result: PitchClass) {
        assert_eq!(pitch_class + n, result);
    }

    #[rstest_parametrize(
        pc1,
        pc2,
        n,
        case(PitchClass::C, PitchClass::C, 0),
        case(PitchClass::D, PitchClass::C, 2),
        case(PitchClass::D, PitchClass::A, 5),
        case(PitchClass::C, PitchClass::CSharp, 11)
    )]
    fn test_sub_self(pc1: PitchClass, pc2: PitchClass, n: Frets) {
        let note1 = Note::from(pc1);
        let note2 = Note::from(pc2);
        assert_eq!(pc1 - pc2, n);
        assert_eq!(note1 - note2, n);
    }

    #[rstest_parametrize(
        pc1,
        n,
        pc2,
        case(PitchClass::C, 0, PitchClass::C),
        case(PitchClass::D, 2, PitchClass::C),
        case(PitchClass::D, 5, PitchClass::A),
        case(PitchClass::C, 11, PitchClass::CSharp),
        case(PitchClass::C, 12, PitchClass::C),
        case(PitchClass::C, 13, PitchClass::B)
    )]
    fn test_sub_int(pc1: PitchClass, n: Frets, pc2: PitchClass) {
        let note1 = Note::from(pc1);
        let note2 = Note::from(pc2);
        assert_eq!(pc1 - n, pc2);
        assert_eq!(note1 - n, note2);
    }
}
