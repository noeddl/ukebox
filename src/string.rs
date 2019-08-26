use crate::chord::Chord;
use crate::note::Note;
use std::fmt;

/// Number of frets shown on the fretboard chart.
const CHART_WIDTH: usize = 5;

/// A string of a Ukulele - not to be confused with Rust's own `String`.
pub struct String<'a> {
    /// The string's name (= name of the fundamental note).
    name: &'a str,
    /// The note played on the string.
    note: Option<Note<'a>>,
    /// The fret pressed to play `note`.
    fret: Option<usize>,
}

impl String<'_> {
    /// Play the note from `chord` which is the next on the string, starting
    /// from fret number `min_fret`.
    /// Return `true` if a note from `chord` can be played on the string under
    /// the given conditions, return `false` otherwise.
    pub fn play_note(&mut self, chord: &Chord, min_fret: usize) -> bool {
        let open_string = Note::from(self.name);

        let max_fret = min_fret + CHART_WIDTH;

        for i in min_fret..max_fret {
            // Get the next note on the fretboard.
            let note = open_string + i;

            // Check if the current note is one of the ones we're looking for.
            if chord.contains(&note) {
                self.note = Some(note);
                self.fret = Some(i);
                return true;
            }
        }

        false
    }
}

impl<'a> From<&'a str> for String<'a> {
    fn from(s: &'a str) -> Self {
        Self {
            name: s,
            note: None,
            fret: None,
        }
    }
}

/// Display the string in ASCII art showing at which fret to press it
/// for playing the current note.
impl fmt::Display for String<'_> {
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

        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest_parametrize;

    #[rstest_parametrize(string, case("C"), case("C#"))]
    fn test_from_str(string: &str) {
        let s = String::from(string);
        assert_eq!(s.name, string);
        assert_eq!(s.note, None);
        assert_eq!(s.fret, None);
    }

    #[rstest_parametrize(
        string, chord, min_fret, note, fret, played, display,
        case("C", "C", 0, Some("C"), Some(0), true, "---+---+---+---+---+"),
        case("C", "C", 1, Some("E"), Some(4), true, "---+---+---+-●-+---+"),
        case("C", "D", 0, Some("D"), Some(2), true, "---+-●-+---+---+---+"),
        //case("?", "?", 0, None, None, false), // TODO: We need a test for this case ...
    )]
    fn test_play_note(
        string: &str,
        chord: &str,
        min_fret: usize,
        note: Option<&str>,
        fret: Option<usize>,
        played: bool,
        display: &str,
    ) {
        let mut s = String::from(string);
        let c = Chord::from(chord);
        let n = match note {
            Some(n) => Some(Note::from(n)),
            None => None,
        };

        let p = s.play_note(&c, min_fret);

        assert_eq!(s.note, n);
        assert_eq!(s.fret, fret);
        assert_eq!(p, played);
        assert_eq!(format!("{}", s), display);
    }
}
