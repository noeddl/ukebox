use crate::note::Note;
use crate::ukulele::CHART_WIDTH;
use crate::Frets;
use std::fmt;

/// A line within a chord diagram which represents a string of a ukulele.
pub struct StringDiagram {
    root: Note,
    base_fret: Frets,
    fret: Frets,
    note: Note,
}

impl StringDiagram {
    pub fn new(root: Note, base_fret: Frets, fret: Frets, note: Note) -> Self {
        Self {
            root,
            base_fret,
            fret,
            note,
        }
    }
}

impl fmt::Display for StringDiagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Show a symbol for the nut if the chord is played on the lower
        // end of the fretboard. Indicate ongoing strings otherwise.
        let nut = match self.base_fret {
            1 => "||",
            _ => "-+",
        };

        // Mark open strings with a special symbol.
        let sym = match self.fret {
            0 => "○",
            _ => " ",
        };

        // Create a line representing the string with the fret to be pressed.
        let mut string = "".to_owned();

        for i in self.base_fret..self.base_fret + CHART_WIDTH {
            let c = match self.fret {
                fret if fret == i => "●",
                _ => "-",
            };

            string.push_str(&format!("-{}-+", c));
        }

        write!(f, "{} {}{}{} {}", self.root, sym, nut, string, self.note)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest_parametrize;
    use std::str::FromStr;

    #[rstest_parametrize(
        root_name,
        base_fret,
        fret,
        note_name,
        diagram,
        case("C", 1, 0, "C", "C ○||---+---+---+---+ C"),
        case("C", 1, 4, "E", "C  ||---+---+---+-●-+ E"),
        case("C", 1, 2, "D", "C  ||---+-●-+---+---+ D"),
        case("G", 1, 4, "B", "G  ||---+---+---+-●-+ B"),
        case("C", 5, 7, "G", "C  -+---+---+-●-+---+ G")
    )]
    fn test_format_line(
        root_name: &str,
        base_fret: Frets,
        fret: Frets,
        note_name: &str,
        diagram: &str,
    ) {
        let root = Note::from_str(root_name).unwrap();
        let note = Note::from_str(note_name).unwrap();

        let sd = StringDiagram::new(root, base_fret, fret, note);

        assert_eq!(sd.to_string(), diagram);
    }
}
