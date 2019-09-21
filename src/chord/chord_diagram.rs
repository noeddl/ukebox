use crate::chord::Chord;
use crate::note::Note;
use crate::ukulele::CHART_WIDTH;
use crate::FretPattern;
use crate::Frets;
use crate::NotePattern;
use crate::STRING_COUNT;
use std::fmt;
use std::str::FromStr;

pub struct ChordDiagram {
    chord: Chord,
    roots: NotePattern,
    frets: FretPattern,
    notes: NotePattern,
}

impl ChordDiagram {
    pub fn new(chord: Chord, frets: FretPattern, notes: NotePattern) -> Self {
        let roots = [
            Note::from_str("G").unwrap(),
            Note::from_str("C").unwrap(),
            Note::from_str("E").unwrap(),
            Note::from_str("A").unwrap(),
        ];

        Self {
            roots,
            chord,
            frets,
            notes,
        }
    }

    /// Determine from which fret to show the fretboard.
    ///
    /// If the rightmost fret fits on the diagram, show the fretboard
    /// beginning at the first fret, otherwise use the leftmost fret
    /// needed for the chords to be played.
    fn get_base_fret(&self) -> Frets {
        let max_fret = *self.frets.iter().max().unwrap();

        match max_fret {
            max_fret if max_fret <= CHART_WIDTH => 1,
            _ => *self.frets.iter().min().unwrap(),
        }
    }

    /// Format a line of the diagram which represents a string of the ukulele.
    fn format_line(&self, root: Note, base_fret: Frets, fret: Frets, note: Note) -> String {
        // Show a symbol for the nut if the chord is played on the lower
        // end of the fretboard. Indicate ongoing strings otherwise.
        let nut = match base_fret {
            1 => "||",
            _ => "-+",
        };

        // Mark open strings with a special symbol.
        let sym = match fret {
            0 => "○",
            _ => " ",
        };

        // Create a line representing the string with the fret to be pressed.
        let mut string = "".to_owned();

        for i in base_fret..base_fret + CHART_WIDTH {
            let c = match fret {
                fret if fret == i => "●",
                _ => "-",
            };

            string.push_str(&format!("-{}-+", c));
        }

        format!("{} {}{}{} {}", root, sym, nut, string, note)
    }
}

impl fmt::Display for ChordDiagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = format!("[{}]\n\n", self.chord);

        // Determine from which fret to show the fretboard.
        let base_fret = self.get_base_fret();

        for i in (0..STRING_COUNT).rev() {
            let root = self.roots[i];
            let fret = self.frets[i];
            let note = self.notes[i];
            let line = self.format_line(root, base_fret, fret, note);
            s.push_str(&format!("{}\n", line));
        }

        // If the fretboard section shown does not include the nut,
        // indicate the number of the first fret shown.
        if base_fret > 1 {
            s.push_str(&format!("      {}\n", base_fret))
        }

        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rstest::rstest_parametrize;
    use std::str::FromStr;

    #[rstest_parametrize(chord_name, min_fret, diagram,
        case(
            "C",
            0,
            indoc!("
                [C - C major]

                A  ||---+---+-●-+---+ C
                E ○||---+---+---+---+ E
                C ○||---+---+---+---+ C
                G ○||---+---+---+---+ G
            "),
        ),
        case(
            "C",
            1,
            indoc!("
                [C - C major]

                A  -+-●-+---+---+---+ C
                E  -+-●-+---+---+---+ G
                C  -+---+-●-+---+---+ E
                G  -+---+---+-●-+---+ C
                      3
            ")
        ),
    )]
    fn test_to_diagram(chord_name: &str, min_fret: Frets, diagram: &str) {
        let chord = Chord::from_str(chord_name).unwrap();
        let chord_diagram = chord.get_diagram(min_fret);
        assert_eq!(chord_diagram.to_string(), diagram);
    }
}
