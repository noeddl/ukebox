use crate::chord::Chord;
use crate::diagram::StringDiagram;
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
}

impl fmt::Display for ChordDiagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display the name of the chord shown.
        let mut s = format!("[{}]\n\n", self.chord);

        // Determine from which fret to show the fretboard.
        let base_fret = self.get_base_fret();

        // Create a diagram for each ukulele string.
        for i in (0..STRING_COUNT).rev() {
            let root = self.roots[i];
            let fret = self.frets[i];
            let note = self.notes[i];
            let sd = StringDiagram::new(root, base_fret, fret, note);
            s.push_str(&format!("{}\n", sd.to_string()));
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
