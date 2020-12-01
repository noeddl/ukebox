use crate::chord::Chord;
use crate::chord::FretID;
use crate::chord::Tuning;
use crate::diagram::FretPattern;
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
    root_width: usize,
}

impl ChordDiagram {
    pub fn new(chord: Chord, frets: impl Into<FretPattern>, tuning: Tuning) -> Self {
        let interval = tuning.get_interval();

        let roots = [
            Note::from_str("G").unwrap() + interval,
            Note::from_str("C").unwrap() + interval,
            Note::from_str("E").unwrap() + interval,
            Note::from_str("A").unwrap() + interval,
        ];

        Self {
            roots,
            chord,
            frets: frets.into(),
            root_width: tuning.get_root_width(),
        }
    }

    /// Determine from which fret to show the fretboard.
    ///
    /// If the rightmost fret fits on the diagram, show the fretboard
    /// beginning at the first fret, otherwise use the leftmost fret
    /// needed for the chords to be played.
    fn get_base_fret(&self) -> FretID {
        let max_fret = self.frets.get_max_fret();

        match max_fret {
            max_fret if max_fret <= CHART_WIDTH => 1,
            _ => self.frets.get_min_fret(),
        }
    }

    /// Compute the notes that correspond to the frets shown as pressed
    /// in the chord diagram.
    fn get_notes(&self) -> NotePattern {
        let mut notes = self.roots;

        for (i, fret) in self.frets.iter().enumerate() {
            let pitch_class = notes[i].pitch_class + *fret;
            notes[i] = match self.chord.get_note(pitch_class) {
                Some(note) => *note,
                _ => panic!(
                    "No note with pitch class {:?} in chord {}",
                    pitch_class, self.chord
                ),
            }
        }

        notes
    }
}

impl fmt::Display for ChordDiagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display the name of the chord shown.
        let mut s = format!("[{}]\n\n", self.chord);

        // Determine from which fret to show the fretboard.
        let base_fret = self.get_base_fret();

        let notes = self.get_notes();

        // Create a diagram for each ukulele string.
        for i in (0..STRING_COUNT).rev() {
            let root = self.roots[i];
            let fret = self.frets[i];
            let note = notes[i];
            let sd = StringDiagram::new(root, base_fret, fret, note, self.root_width);
            s.push_str(&format!("{}\n", sd.to_string()));
        }

        // If the fretboard section shown does not include the nut,
        // indicate the number of the first fret shown.
        if base_fret > 1 {
            s.push_str(&format!(
                "{:width$}\n",
                base_fret,
                width = self.root_width + 6
            ))
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
