use crate::note::Interval;
use crate::note::PitchClass;
use crate::note::StaffPosition;
use std::fmt;
use std::ops::Add;
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
#[derive(Debug, Clone, Copy)]
pub struct Note {
    pub pitch_class: PitchClass,
    staff_position: StaffPosition,
}

impl PartialEq for Note {
    /// Treat two notes as equal if they are represented by the same symbol.
    /// For example, `B sharp`, `C` and `D double flat` are all casually
    /// called `C`.
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PitchClass::*;
        use StaffPosition::*;

        let s = match (self.staff_position, self.pitch_class) {
            // Notes on staff position for C.
            (CPos, ASharp) => "Bb", // C double flat
            (CPos, B) => "B",       // Cb
            (CPos, C) => "C",
            (CPos, CSharp) => "C#",
            (CPos, D) => "D", // C double sharp
            // Notes on staff position for D.
            (DPos, C) => "C", // D double flat
            (DPos, CSharp) => "Db",
            (DPos, D) => "D",
            (DPos, DSharp) => "D#",
            (DPos, E) => "E", // D double sharp
            // Notes on staff position for E.
            (EPos, D) => "D", // E double flat
            (EPos, DSharp) => "Eb",
            (EPos, E) => "E",
            (EPos, F) => "F",       // E#
            (EPos, FSharp) => "F#", // E double sharp
            // Notes on staff position for F.
            (FPos, DSharp) => "Eb", // F double flat
            (FPos, E) => "E",       // Fb
            (FPos, F) => "F",
            (FPos, FSharp) => "F#",
            (FPos, G) => "G", // F double sharp
            // Notes on staff position for G.
            (GPos, F) => "F", // G double flat
            (GPos, FSharp) => "Gb",
            (GPos, G) => "G",
            (GPos, GSharp) => "G#",
            (GPos, A) => "A", // G double sharp
            // Notes on staff position for A.
            (APos, G) => "G", // A double flat
            (APos, GSharp) => "Ab",
            (APos, A) => "A",
            (APos, ASharp) => "A#",
            (APos, B) => "B", // A double sharp
            // Notes on staff position for B.
            (BPos, A) => "A", // B double flat
            (BPos, ASharp) => "Bb",
            (BPos, B) => "B",
            (BPos, C) => "C",       // B#
            (BPos, CSharp) => "C#", // B double sharp
            _ => panic!("Impossible combination of PitchClass and StaffPosition"),
        };

        write!(f, "{}", s)
    }
}

impl FromStr for Note {
    type Err = ParseNoteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PitchClass::*;
        use StaffPosition::*;

        let name = s.to_string();

        let (pitch_class, staff_position) = match s {
            "C" => (C, CPos),
            "C#" => (CSharp, CPos),
            "Db" => (CSharp, DPos),
            "D" => (D, DPos),
            "D#" => (DSharp, DPos),
            "Eb" => (DSharp, EPos),
            "E" => (E, EPos),
            "F" => (F, FPos),
            "F#" => (FSharp, FPos),
            "Gb" => (FSharp, GPos),
            "G" => (G, GPos),
            "G#" => (GSharp, GPos),
            "Ab" => (GSharp, APos),
            "A" => (A, APos),
            "A#" => (ASharp, APos),
            "Bb" => (ASharp, BPos),
            "B" => (B, BPos),
            _ => return Err(ParseNoteError { name }),
        };

        Ok(Self {
            pitch_class,
            staff_position,
        })
    }
}

impl Add<Interval> for Note {
    type Output = Self;

    /// Get the next note when adding `interval` to the current note.
    fn add(self, interval: Interval) -> Self {
        let pitch_class = self.pitch_class + interval.to_semitones();
        let staff_position = self.staff_position + (interval.to_number() - 1);
        Self {
            pitch_class,
            staff_position,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use Interval::*;

    #[rstest(
        s,
        case("C"),
        case("C#"),
        case("Db"),
        case("D"),
        case("D#"),
        case("Eb"),
        case("E"),
        case("F"),
        case("F#"),
        case("Gb"),
        case("G"),
        case("G#"),
        case("Ab"),
        case("A"),
        case("A#"),
        case("Bb"),
        case("B")
    )]
    fn test_from_and_to_str(s: &str) {
        let note = Note::from_str(s).unwrap();
        assert_eq!(format!("{}", note), s);
    }

    #[rstest(
        note_name,
        interval,
        result_name,
        case("C", PerfectUnison, "C"),
        case("C", MinorThird, "Eb"),
        case("C", MajorThird, "E"),
        case("C", PerfectFifth, "G"),
        case("C#", PerfectUnison, "C#"),
        case("C#", MajorThird, "F")
    )]
    fn test_add_interval(note_name: &str, interval: Interval, result_name: &str) {
        let note = Note::from_str(note_name).unwrap();
        assert_eq!(note + interval, Note::from_str(result_name).unwrap());
    }
}
