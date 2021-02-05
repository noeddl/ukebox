use crate::note::Interval;
use crate::note::PitchClass;
use crate::note::Semitones;
use crate::note::StaffPosition;
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
#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord)]
pub struct Note {
    pub pitch_class: PitchClass,
    staff_position: StaffPosition,
}

impl Note {
    pub fn new(pitch_class: PitchClass, staff_position: StaffPosition) -> Self {
        Self {
            pitch_class,
            staff_position,
        }
    }

    /// Return `true` if this note is a "white note", i.e. a note represented
    /// by a white key on the piano (i.e. the note is part of the C major scale).
    pub fn is_white_note(&self) -> bool {
        use PitchClass::*;

        matches!(self.pitch_class, C | D | E | F | G | A | B)
    }
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

        Ok(Self::new(pitch_class, staff_position))
    }
}

impl From<PitchClass> for Note {
    /// Convert a pitch class into a note.
    /// For notes that can be sharp or flat use the sharp version.
    fn from(pitch_class: PitchClass) -> Self {
        use PitchClass::*;
        use StaffPosition::*;

        let staff_position = match pitch_class {
            C | CSharp => CPos,
            D | DSharp => DPos,
            E => EPos,
            F | FSharp => FPos,
            G | GSharp => GPos,
            A | ASharp => APos,
            B => BPos,
        };

        Self::new(pitch_class, staff_position)
    }
}

impl Add<Interval> for Note {
    type Output = Self;

    /// Get the next note when adding `interval` to the current note.
    fn add(self, interval: Interval) -> Self {
        let pitch_class = self.pitch_class + interval.to_semitones();
        let staff_position = self.staff_position + (interval.to_number() - 1);
        Self::new(pitch_class, staff_position)
    }
}

impl Add<Semitones> for Note {
    type Output = Self;

    fn add(self, n: Semitones) -> Self {
        let note = Self::from(self.pitch_class + n);

        // Make sure the staff position stays the same if the pitch class
        // stays the same (e.g. when adding 0 or 12 semitones).
        if note.pitch_class == self.pitch_class {
            return Self::new(self.pitch_class, self.staff_position);
        }

        // Otherwise, the staff position will by default be chosen so that
        // sharp/flat notes turn out sharp (e.g. C + 1 = C#).
        note
    }
}

impl Sub<Semitones> for Note {
    type Output = Self;

    fn sub(self, n: Semitones) -> Self {
        let note = Self::from(self.pitch_class - n);

        // Make sure the staff position stays the same if the pitch class
        // stays the same (e.g. when subtracting 0 or 12 semitones).
        if note.pitch_class == self.pitch_class {
            return Self::new(self.pitch_class, self.staff_position);
        }

        // Make sure that the staff position will be chosen so that
        // sharp/flat notes turn out flat (e.g. D - 1 = Db).
        let staff_position = match note {
            n if n.is_white_note() => note.staff_position,
            _ => note.staff_position + 1,
        };

        Self::new(note.pitch_class, staff_position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use Interval::*;
    use PitchClass::*;

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
        s,
        is_white_note,
        case("C", true),
        case("C#", false),
        case("Db", false),
        case("D", true),
        case("D#", false),
        case("Eb", false),
        case("E", true),
        case("F", true),
        case("F#", false),
        case("Gb", false),
        case("G", true),
        case("G#", false),
        case("Ab", false),
        case("A", true),
        case("A#", false),
        case("Bb", false),
        case("B", true)
    )]
    fn test_is_white_note(s: &str, is_white_note: bool) {
        let note = Note::from_str(s).unwrap();
        assert_eq!(note.is_white_note(), is_white_note);
    }

    #[rstest(
        pitch_class,
        s,
        case(C, "C"),
        case(CSharp, "C#"),
        case(D, "D"),
        case(DSharp, "D#"),
        case(E, "E"),
        case(F, "F"),
        case(FSharp, "F#"),
        case(G, "G"),
        case(GSharp, "G#"),
        case(A, "A"),
        case(ASharp, "A#"),
        case(B, "B")
    )]
    fn test_from_pitch_class(pitch_class: PitchClass, s: &str) {
        let note = Note::from(pitch_class);
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

    #[rstest(
        note_name,
        n,
        result_name,
        case("C", 0, "C"),
        case("C#", 0, "C#"),
        case("Db", 0, "Db"),
        case("C", 1, "C#"),
        case("C#", 1, "D"),
        case("Db", 1, "D"),
        case("C", 3, "D#"),
        case("C", 4, "E"),
        case("C", 7, "G"),
        case("A", 3, "C"),
        case("A", 12, "A"),
        case("A#", 12, "A#"),
        case("Ab", 12, "Ab")
    )]
    fn test_add_semitones(note_name: &str, n: Semitones, result_name: &str) {
        let note = Note::from_str(note_name).unwrap();
        assert_eq!(note + n, Note::from_str(result_name).unwrap());
    }

    #[rstest(
        note_name,
        n,
        result_name,
        case("C", 0, "C"),
        case("C#", 0, "C#"),
        case("Db", 0, "Db"),
        case("C", 1, "B"),
        case("C", 2, "Bb"),
        case("C#", 3, "Bb"),
        case("Db", 3, "Bb"),
        case("A", 3, "Gb"),
        case("A", 12, "A")
    )]
    fn test_subtract_semitones(note_name: &str, n: Semitones, result_name: &str) {
        let note = Note::from_str(note_name).unwrap();
        assert_eq!(note - n, Note::from_str(result_name).unwrap());
    }
}
