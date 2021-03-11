use crate::FretID;
use crate::FretPattern;
use crate::Note;
use crate::Tuning;
use crate::STRING_COUNT;
use itertools::izip;
use std::fmt;

pub struct ChordDiagram {
    frets: FretPattern,
    tuning: Tuning,
    notes: [Note; STRING_COUNT],
    max_span: FretID,
    base_fret: FretID,
    root_width: usize,
}

impl ChordDiagram {
    pub fn new(
        frets: impl Into<FretPattern>,
        tuning: Tuning,
        notes: [Note; STRING_COUNT],
        max_span: FretID,
    ) -> Self {
        let frets = frets.into();

        // Determine from which fret to show the fretboard.
        let base_fret = frets.get_base_fret(max_span);

        // Get the width of the space that we need to print the name
        // of the root notes (the names of the strings).
        let root_width = tuning.get_root_width();

        Self {
            frets,
            tuning,
            notes,
            max_span,
            base_fret,
            root_width,
        }
    }

    /// Format a line that represents a ukulele string in a chord diagram.
    pub fn format_line(&self, root: Note, fret: FretID, note: Note) -> String {
        let root_str = format!("{:width$}", root.to_string(), width = self.root_width);

        // Show a symbol for the nut if the chord is played on the lower
        // end of the fretboard. Indicate ongoing strings otherwise.
        let nut = match self.base_fret {
            1 => "||",
            _ => "-|",
        };

        // Mark open strings with a special symbol.
        let sym = match fret {
            0 => "o",
            _ => " ",
        };

        // Create a line representing the string with the fret to be pressed.
        let mut string = "".to_string();

        for i in self.base_fret..self.base_fret + self.max_span {
            let c = match fret {
                fret if fret == i => "o",
                _ => "-",
            };

            string.push_str(&format!("-{}-|", c));
        }

        format!("{} {}{}{}- {}", root_str, sym, nut, string, note)
    }
}

impl fmt::Display for ChordDiagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = "".to_string();

        let roots = self.tuning.get_roots();

        // Create a diagram for each ukulele string.
        for (root, fret, note) in izip!(&roots, self.frets.iter(), &self.notes).rev() {
            let sd = self.format_line(*root, *fret, *note);
            s.push_str(&sd);
            s.push('\n');
        }

        // If the fretboard section shown does not include the nut,
        // indicate the number of the first fret shown.
        if self.base_fret > 1 {
            s.push_str(&format!(
                "{:width$}\n",
                self.base_fret,
                width = self.root_width + 6
            ))
        }

        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chord::Chord;
    use indoc::indoc;
    use rstest::rstest;
    use std::str::FromStr;

    #[rstest(
        root_name,
        min_fret,
        fret,
        note_name,
        diagram,
        case("C", 0, 0, "C", "C o||---|---|---|---|- C"),
        case("C", 0, 4, "E", "C  ||---|---|---|-o-|- E"),
        case("C", 0, 2, "D", "C  ||---|-o-|---|---|- D"),
        case("G", 0, 4, "B", "G  ||---|---|---|-o-|- B"),
        case("C", 5, 7, "G", "C  -|---|---|-o-|---|- G"),
        case("C", 0, 1, "C#", "C  ||-o-|---|---|---|- C#"),
        case("C", 0, 1, "Db", "C  ||-o-|---|---|---|- Db"),
        case("C", 5, 6, "F#", "C  -|---|-o-|---|---|- F#"),
        case("C", 5, 6, "Gb", "C  -|---|-o-|---|---|- Gb"),
        case("F#", 0, 0, "F#", "F# o||---|---|---|---|- F#"),
        case("F#", 0, 4, "A#", "F#  ||---|---|---|-o-|- A#"),
        case("F#", 5, 7, "D", "F#  -|---|-o-|---|---|- D")
    )]
    fn test_format_line(
        root_name: &str,
        min_fret: FretID,
        fret: FretID,
        note_name: &str,
        diagram: &str,
    ) {
        let chord = Chord::from_str(root_name).unwrap();
        let chord_diagrams = chord.get_voicings(min_fret, Tuning::C);
        let chord_diagram = &chord_diagrams[0];

        let root = Note::from_str(root_name).unwrap();
        let note = Note::from_str(note_name).unwrap();

        let sd = chord_diagram.format_line(root, fret, note);

        assert_eq!(sd, diagram);
    }

    #[rstest(chord_name, min_fret, tuning, diagram,
        case(
            "C",
            0,
            Tuning::C,
            indoc!("
                A  ||---|---|-o-|---|- C
                E o||---|---|---|---|- E
                C o||---|---|---|---|- C
                G o||---|---|---|---|- G
            "),
        ),
        case(
            "C",
            1,
            Tuning::C,
            indoc!("
                A  -|-o-|---|---|---|- C
                E  -|-o-|---|---|---|- G
                C  -|---|-o-|---|---|- E
                G  -|---|---|-o-|---|- C
                      3
            ")
        ),
        case(
            "C#",
            0,
            Tuning::C,
            indoc!("
                A  ||---|---|---|-o-|- C#
                E  ||-o-|---|---|---|- F
                C  ||-o-|---|---|---|- C#
                G  ||-o-|---|---|---|- G#
            ")
        ),
        case(
            "Db",
            0,
            Tuning::C,
            indoc!("
                A  ||---|---|---|-o-|- Db
                E  ||-o-|---|---|---|- F
                C  ||-o-|---|---|---|- Db
                G  ||-o-|---|---|---|- Ab
            ")
        ),
        case(
            "Cm",
            0,
            Tuning::C,
            indoc!("
                A  ||---|---|-o-|---|- C
                E  ||---|---|-o-|---|- G
                C  ||---|---|-o-|---|- Eb
                G o||---|---|---|---|- G
            "),
        ),
        case(
            "C#m",
            0,
            Tuning::C,
            indoc!("
                A  ||---|---|---|-o-|- C#
                E o||---|---|---|---|- E
                C  ||-o-|---|---|---|- C#
                G  ||-o-|---|---|---|- G#
            ")
        ),
        case(
            "Dbm",
            0,
            Tuning::C,
            indoc!("
                A  ||---|---|---|-o-|- Db
                E o||---|---|---|---|- E
                C  ||-o-|---|---|---|- Db
                G  ||-o-|---|---|---|- Ab
            ")
        ),
        case(
            "D",
            0,
            Tuning::D,
            indoc!("
                B   ||---|---|-o-|---|- D
                F# o||---|---|---|---|- F#
                D  o||---|---|---|---|- D
                A  o||---|---|---|---|- A
            "),
        ),
        case(
            "D",
            5,
            Tuning::D,
            indoc!("
                B   -|---|---|-o-|---|- F#
                F#  -|---|---|---|-o-|- D
                D   -|---|---|-o-|---|- A
                A   -|-o-|---|---|---|- D
                       5
            "),
        ),
        case(
            "G",
            0,
            Tuning::G,
            indoc!("
                E  ||---|---|-o-|---|- G
                B o||---|---|---|---|- B
                G o||---|---|---|---|- G
                D o||---|---|---|---|- D
            "),
        ),
    )]
    fn test_to_diagram(chord_name: &str, min_fret: FretID, tuning: Tuning, diagram: &str) {
        let chord = Chord::from_str(chord_name).unwrap();
        let chord_diagrams = chord.get_voicings(min_fret, tuning);
        let chord_diagram = &chord_diagrams[0];
        assert_eq!(chord_diagram.to_string(), diagram);
    }
}
