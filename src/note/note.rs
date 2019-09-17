use crate::note::PitchClass;
use crate::Frets;
use std::fmt;
use std::ops::Add;
use std::ops::Sub;
use std::str::FromStr;

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
        assert_eq!(note1 - n, note2);
    }
}
