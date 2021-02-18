use crate::chord::FretID;
use crate::chord::Tuning;
use crate::diagram::FretPattern;
use crate::note::Note;
use crate::STRING_COUNT;
use itertools::izip;
use std::fmt;

pub struct ChordDiagram {
    frets: FretPattern,
    tuning: Tuning,
    notes: [Note; STRING_COUNT],
    max_span: FretID,
}

impl ChordDiagram {
    pub fn new(
        frets: impl Into<FretPattern>,
        tuning: Tuning,
        notes: [Note; STRING_COUNT],
        max_span: FretID,
    ) -> Self {
        Self {
            frets: frets.into(),
            tuning,
            notes,
            max_span,
        }
    }

    /// Format a line that represents a ukulele string in a chord diagram.
    pub fn format_line(&self, root: Note, fret: FretID, note: Note) -> String {
        // Determine from which fret to show the fretboard.
        let base_fret = self.frets.get_base_fret(self.max_span);

        // Get the width of the space that we need to print the name
        // of the root notes (the names of the strings).
        let root_width = self.tuning.get_root_width();

        let root_str = format!("{:width$}", root.to_string(), width = root_width);

        // Show a symbol for the nut if the chord is played on the lower
        // end of the fretboard. Indicate ongoing strings otherwise.
        let nut = match base_fret {
            1 => "||",
            _ => "-|",
        };

        // Mark open strings with a special symbol.
        let sym = match fret {
            0 => "o",
            _ => " ",
        };

        // Create a line representing the string with the fret to be pressed.
        let mut string = "".to_owned();

        for i in base_fret..base_fret + self.max_span {
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

        // Determine from which fret to show the fretboard.
        let base_fret = self.frets.get_base_fret(self.max_span);

        // Get the width of the space that we need to print the name
        // of the root notes (the names of the strings).
        let root_width = self.tuning.get_root_width();

        // If the fretboard section shown does not include the nut,
        // indicate the number of the first fret shown.
        if base_fret > 1 {
            s.push_str(&format!("{:width$}\n", base_fret, width = root_width + 6))
        }

        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rstest::rstest;
    use std::str::FromStr;

    #[rstest(
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
        case("C", 5, 6, "Gb", "C  -|---|-o-|---|---|- Gb"),
        case("F#", 1, 0, "F#", "F# o||---|---|---|---|- F#"),
        case("F#", 1, 4, "A#", "F#  ||---|---|---|-o-|- A#"),
        case("F#", 5, 7, "D", "F#  -|---|---|-o-|---|- D")
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

        let root_width = root_name.len();

        let sd = format_line(root, base_fret, fret, note, root_width);

        assert_eq!(sd, diagram);
    }

    #[rstest(chord_name, min_fret, tuning, diagram,
        case(
            "C",
            0,
            Tuning::C,
            indoc!("
                [C - C major]

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
                [C - C major]

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
                [C# - C# major]

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
                [Db - Db major]

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
                [Cm - C minor]

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
                [C#m - C# minor]

                A  ||---|---|---|-o-|- C#
                E  ||---|---|---|-o-|- G#
                C  ||---|---|---|-o-|- E
                G  ||-o-|---|---|---|- G#
            ")
        ),
        case(
            "Dbm",
            0,
            Tuning::C,
            indoc!("
                [Dbm - Db minor]

                A  ||---|---|---|-o-|- Db
                E  ||---|---|---|-o-|- Ab
                C  ||---|---|---|-o-|- E
                G  ||-o-|---|---|---|- Ab
            ")
        ),
        case(
            "D",
            0,
            Tuning::D,
            indoc!("
                [D - D major]

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
                [D - D major]

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
                [G - G major]

                E  ||---|---|-o-|---|- G
                B o||---|---|---|---|- B
                G o||---|---|---|---|- G
                D o||---|---|---|---|- D
            "),
        ),
    )]
    fn test_to_diagram(chord_name: &str, min_fret: FretID, tuning: Tuning, diagram: &str) {
        let chord = Chord::from_str(chord_name).unwrap();
        let chord_diagram = chord.get_diagram(min_fret, tuning);
        assert_eq!(chord_diagram.to_string(), diagram);
    }
}
