use crate::chord::FretID;
use crate::diagram::CHART_WIDTH;
use crate::note::Note;
use std::fmt;

/// A line within a chord diagram which represents a string of a ukulele.
pub struct StringDiagram {
    root: Note,
    base_fret: FretID,
    fret: FretID,
    note: Note,
    root_width: usize,
}

impl StringDiagram {
    pub fn new(root: Note, base_fret: FretID, fret: FretID, note: Note, root_width: usize) -> Self {
        Self {
            root,
            base_fret,
            fret,
            note,
            root_width,
        }
    }
}

impl fmt::Display for StringDiagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let root = format!("{:width$}", self.root.to_string(), width = self.root_width);

        // Show a symbol for the nut if the chord is played on the lower
        // end of the fretboard. Indicate ongoing strings otherwise.
        let nut = match self.base_fret {
            1 => "||",
            _ => "-|",
        };

        // Mark open strings with a special symbol.
        let sym = match self.fret {
            0 => "o",
            _ => " ",
        };

        // Create a line representing the string with the fret to be pressed.
        let mut string = "".to_owned();

        for i in self.base_fret..self.base_fret + CHART_WIDTH {
            let c = match self.fret {
                fret if fret == i => "o",
                _ => "-",
            };

            string.push_str(&format!("-{}-|", c));
        }

        write!(f, "{} {}{}{}- {}", root, sym, nut, string, self.note)
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
        case("C", 1, 0, "C", "C o||---|---|---|---|- C"),
        case("C", 1, 4, "E", "C  ||---|---|---|-o-|- E"),
        case("C", 1, 2, "D", "C  ||---|-o-|---|---|- D"),
        case("G", 1, 4, "B", "G  ||---|---|---|-o-|- B"),
        case("C", 5, 7, "G", "C  -|---|---|-o-|---|- G"),
        case("C", 1, 1, "C#", "C  ||-o-|---|---|---|- C#"),
        case("C", 1, 1, "Db", "C  ||-o-|---|---|---|- Db"),
        case("C", 5, 6, "F#", "C  -|---|-o-|---|---|- F#"),
        case("C", 5, 6, "Gb", "C  -|---|-o-|---|---|- Gb")
    )]
    fn test_format_line(
        root_name: &str,
        base_fret: FretID,
        fret: FretID,
        note_name: &str,
        diagram: &str,
    ) {
        let root = Note::from_str(root_name).unwrap();
        let note = Note::from_str(note_name).unwrap();

        let sd = StringDiagram::new(root, base_fret, fret, note, 1);

        assert_eq!(sd.to_string(), diagram);
    }
}
