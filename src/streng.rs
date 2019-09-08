use crate::chord::Chord;
use crate::note::Note;
use std::fmt;
use std::str::FromStr;

/// Number of frets shown on the fretboard chart.
const CHART_WIDTH: u8 = 4;

/// A string of a ukulele (or potentially another string instrument).
/// We use the Danish word `streng` to avoid name clashes and confusion
/// with Rust's `String`.
pub struct Streng {
    /// The string's name (= name of the fundamental note).
    name: String,
    /// The note played on the string.
    note: Option<Note>,
    /// The fret pressed to play `note`.
    fret: Option<u8>,
}

impl Streng {
    /// Play the note from `chord` which is the next on the string, starting
    /// from fret number `min_fret`.
    /// Return `true` if a note from `chord` can be played on the string under
    /// the given conditions, return `false` otherwise.
    pub fn play_note(&mut self, chord: &Chord, min_fret: u8) -> bool {
        let open_string = Note::from_str(&self.name).unwrap();

        let max_fret = min_fret + CHART_WIDTH;

        for i in min_fret..=max_fret {
            // Get the next note on the fretboard.
            let note = open_string + i;

            // Check if the current note is one of the ones we're looking for.
            if chord.contains(note) {
                self.note = Some(note);
                self.fret = Some(i);
                return true;
            }
        }

        false
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
        string, chord, min_fret, note, fret, played, display,
        case("C", "C", 0, Some("C"), Some(0), true, "C ○||---+---+---+---+ C"),
        case("C", "C", 1, Some("E"), Some(4), true, "C  ||---+---+---+-●-+ E"),
        case("C", "D", 0, Some("D"), Some(2), true, "C  ||---+-●-+---+---+ D"),
        case("G", "B", 0, Some("B"), Some(4), true, "G  ||---+---+---+-●-+ B"),
        //case("?", "?", 0, None, None, false), // TODO: We need a test for this case ...
    )]
    fn test_play_note(
        string: &str,
        chord: &str,
        min_fret: u8,
        note: Option<&str>,
        fret: Option<u8>,
        played: bool,
        display: &str,
    ) {
        let mut s = Streng::from(string);
        let c = Chord::from_str(chord).unwrap();
        let n = match note {
            Some(n) => Some(Note::from_str(n).unwrap()),
            None => None,
        };

        let p = s.play_note(&c, min_fret);

        assert_eq!(s.note, n);
        assert_eq!(s.fret, fret);
        assert_eq!(p, played);
        assert_eq!(format!("{}", s), display);
    }
}
