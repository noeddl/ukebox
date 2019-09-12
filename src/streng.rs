use crate::note::Note;
use crate::Frets;
use std::fmt;
use std::str::FromStr;

/// Number of frets shown on the fretboard chart.
const CHART_WIDTH: Frets = 4;

/// A string of a ukulele (or potentially another string instrument).
/// We use the Danish word `streng` to avoid name clashes and confusion
/// with Rust's `String`.
pub struct Streng {
    /// The string's name (= name of the fundamental note).
    name: String,
    /// The note played on the string.
    note: Option<Note>,
    /// The fret pressed to play `note`.
    fret: Option<Frets>,
}

impl Streng {
    /// Press the string on `fret`.
    pub fn play(&mut self, fret: Frets) {
        let open_string = Note::from_str(&self.name).unwrap();

        self.note = Some(open_string + fret);
        self.fret = Some(fret);
    }
}

impl From<&str> for Streng {
    fn from(s: &str) -> Self {
        Self {
            name: s.to_string(),
            note: None,
            fret: None,
        }
    }
}

/// Display the string in ASCII art showing at which fret to press it
/// for playing the current note.
impl fmt::Display for Streng {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = "".to_owned();

        // 0 is the open (unpressed) string, so start at 1.
        for i in 1..=CHART_WIDTH {
            let c = match self.fret {
                Some(fret) if fret == i => "●",
                _ => "-",
            };

            s.push_str(&format!("-{}-+", c));
        }

        // Mark string as open or muted or neither.
        let sym = match self.fret {
            Some(fret) if fret == 0 => "○",
            None => "x",
            _ => " ",
        };

        // Get the name of the note played.
        let note = match self.note {
            Some(note) => format!("{}", note),
            None => "X".to_owned(),
        };

        write!(f, "{} {}||{} {}", self.name, sym, s, note)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest_parametrize;

    #[rstest_parametrize(string, case("C"), case("C#"))]
    fn test_from_str(string: &str) {
        let s = Streng::from(string);
        assert_eq!(s.name, string);
        assert_eq!(s.note, None);
        assert_eq!(s.fret, None);
    }

    #[rstest_parametrize(
        string, fret, note, display,
        case("C", 0, Some("C"), "C ○||---+---+---+---+ C"),
        case("C", 4, Some("E"), "C  ||---+---+---+-●-+ E"),
        case("C", 2, Some("D"), "C  ||---+-●-+---+---+ D"),
        case("G", 4, Some("B"), "G  ||---+---+---+-●-+ B"),
        //case("?", "?", 0, None, None, false), // TODO: We need a test for this case ...
    )]
    fn test_play(string: &str, fret: Frets, note: Option<&str>, display: &str) {
        let mut s = Streng::from(string);

        let n = match note {
            Some(n) => Some(Note::from_str(n).unwrap()),
            None => None,
        };

        s.play(fret);

        assert_eq!(s.note, n);
        assert_eq!(s.fret, Some(fret));
        assert_eq!(format!("{}", s), display);
    }
}
