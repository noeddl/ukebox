use crate::chord::Chord;
use crate::chord::FretID;
use crate::chord::FretPattern;
use crate::diagram::StringDiagram;
use crate::diagram::CHART_WIDTH;
use crate::note::Note;
use crate::STRING_COUNT;
use std::fmt;
use std::str::FromStr;

type NotePattern = [Note; STRING_COUNT];

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
    fn get_base_fret(&self) -> FretID {
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

                A  ||---|---|-o-|---|- C
                E o||---|---|---|---|- E
                C o||---|---|---|---|- C
                G o||---|---|---|---|- G
            "),
        ),
        case(
            "C",
            1,
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
            indoc!("
                [Dbm - Db minor]

                A  ||---|---|---|-o-|- Db
                E  ||---|---|---|-o-|- Ab
                C  ||---|---|---|-o-|- E
                G  ||-o-|---|---|---|- Ab
            ")
        ),
    )]
    fn test_to_diagram(chord_name: &str, min_fret: FretID, diagram: &str) {
        let chord = Chord::from_str(chord_name).unwrap();
        let chord_diagram = chord.get_diagram(min_fret);
        assert_eq!(chord_diagram.to_string(), diagram);
    }
}
