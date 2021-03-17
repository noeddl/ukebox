use std::fmt;
use std::slice::Iter;

use crate::{Chord, FretID, Note, Semitones, UkeString, STRING_COUNT};

pub struct ChordDiagram {
    uke_strings: [UkeString; STRING_COUNT],
    max_span: Semitones,
}

impl ChordDiagram {
    pub fn new(uke_strings: [UkeString; STRING_COUNT], max_span: Semitones) -> Self {
        Self {
            uke_strings,
            max_span,
        }
    }

    pub fn uke_strings(&self) -> Iter<'_, UkeString> {
        self.uke_strings.iter()
    }

    pub fn roots(&self) -> impl Iterator<Item = Note> + '_ {
        self.uke_strings.iter().map(|(r, _f, _n)| *r)
    }

    pub fn frets(&self) -> impl Iterator<Item = FretID> + '_ {
        self.uke_strings.iter().map(|(_r, f, _n)| *f)
    }

    pub fn notes(&self) -> impl Iterator<Item = Note> + '_ {
        self.uke_strings.iter().map(|(_r, _f, n)| *n)
    }

    /// Return `true` if the diagram is a valid depiction of how to
    /// play the given `chord`.
    pub fn depicts(&self, chord: &Chord) -> bool {
        let notes: Vec<Note> = self.notes().collect();
        chord.consists_of(&notes)
    }

    /// Return the lowest fret at which a string is pressed down.
    pub fn get_min_fret(&self) -> FretID {
        match self.frets().filter(|&x| x > 0).min() {
            Some(x) => x,
            // Special case [0, 0, 0, 0]: no string is pressed down.
            _ => 0,
        }
    }

    pub fn get_max_fret(&self) -> FretID {
        self.frets().max().unwrap()
    }

    pub fn get_span(&self) -> FretID {
        self.get_max_fret() - self.get_min_fret()
    }

    /// Determine from which fret to show the fretboard.
    ///
    /// If the rightmost fret fits on the diagram, show the fretboard
    /// beginning at the first fret, otherwise use the leftmost fret
    /// needed for the chords to be played.
    pub fn get_base_fret(&self) -> FretID {
        let max_fret = self.get_max_fret();

        match max_fret {
            max_fret if max_fret <= self.max_span => 1,
            _ => self.get_min_fret(),
        }
    }

    /// Get the width of the space that we need to print the names
    /// of the root notes (the names of the strings).
    pub fn get_root_width(&self) -> usize {
        self.roots().map(|n| format!("{}", n).len()).max().unwrap()
    }

    /// Format a line that represents a ukulele string in a chord diagram.
    pub fn format_line(
        &self,
        uke_string: UkeString,
        base_fret: FretID,
        root_width: usize,
    ) -> String {
        let (root, fret, note) = uke_string;

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
        let s: String = (base_fret..base_fret + self.max_span)
            .map(|i| if fret == i { 'o' } else { '-' })
            .map(|c| format!("-{}-|", c))
            .collect();

        format!("{} {}{}{}- {}\n", root_str, sym, nut, s, note)
    }
}

impl fmt::Display for ChordDiagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Determine from which fret to show the fretboard.
        let base_fret = self.get_base_fret();

        // Get the width of the space that we need to print the name
        // of the root notes (the names of the strings).
        let root_width = self.get_root_width();

        // Create a diagram for each ukulele string.
        let mut s: String = self
            .uke_strings()
            .rev()
            .map(|us| self.format_line(*us, base_fret, root_width))
            .collect();

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
    use std::str::FromStr;

    use indoc::indoc;
    use rstest::rstest;

    use super::*;
    use crate::{Chord, Tuning};

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
