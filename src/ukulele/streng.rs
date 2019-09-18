use crate::note::Note;
use crate::ukulele::CHART_WIDTH;
use crate::Frets;
use std::fmt;

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
    /// The first fret from which to show the fretboard chart.
    base_fret: Frets,
}

impl Streng {
    /// Press the string on `fret`.
    pub fn play(&mut self, fret: Frets, note: Note, base_fret: Frets) {
        //let open_string = Note::from_str(&self.name).unwrap();

        self.note = Some(note);
        self.fret = Some(fret);
        self.base_fret = base_fret;
    }
}

impl From<&str> for Streng {
    fn from(s: &str) -> Self {
        Self {
            name: s.to_string(),
            note: None,
            fret: None,
            // 0 is the open (unpressed) string, so start at 1 by default.
            base_fret: 1,
        }
    }
}

/// Display the string in ASCII art showing at which fret to press it
/// for playing the current note.
impl fmt::Display for Streng {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = "".to_owned();

        let base_fret = self.base_fret;

        // Show a symbol for the nut if the chord is played on the lower
        // end of the fretboard. Indicate ongoing strings otherwise.
        let nut = match base_fret {
            1 => "||",
            _ => "-+",
        };

        // Create a line representing the string with the fret to be pressed.
        for i in base_fret..base_fret + CHART_WIDTH {
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

        write!(f, "{} {}{}{} {}", self.name, sym, nut, s, note)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest_parametrize;
    use std::str::FromStr;

    #[rstest_parametrize(string, case("C"), case("C#"))]
    fn test_from_str(string: &str) {
        let s = Streng::from(string);
        assert_eq!(s.name, string);
        assert_eq!(s.note, None);
        assert_eq!(s.fret, None);
        assert_eq!(s.base_fret, 1);
    }

    #[rstest_parametrize(
        string, fret, base_fret, note, display,
        case("C", 0, 1, "C", "C ○||---+---+---+---+ C"),
        case("C", 4, 1, "E", "C  ||---+---+---+-●-+ E"),
        case("C", 2, 1, "D", "C  ||---+-●-+---+---+ D"),
        case("G", 4, 1, "B", "G  ||---+---+---+-●-+ B"),
        case("C", 7, 5, "G", "C  -+---+---+-●-+---+ G"),
        //case("?", "?", 0, None, None, false), // TODO: We need a test for this case ...
    )]
    fn test_play(string: &str, fret: Frets, base_fret: Frets, note: &str, display: &str) {
        let mut s = Streng::from(string);
        let n = Note::from_str(note).unwrap();

        s.play(fret, n, base_fret);

        assert_eq!(s.fret, Some(fret));
        assert_eq!(s.base_fret, base_fret);
        assert_eq!(format!("{}", s), display);
    }
}
