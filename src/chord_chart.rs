use std::cmp::max;
use std::fmt;

use crate::{FretID, Semitones, UkeString, Voicing, MIN_CHART_WIDTH};

pub struct ChordChart {
    voicing: Voicing,
    /// Number of frets to use to display the chord voicing
    width: Semitones,
}

impl ChordChart {
    pub fn new(voicing: Voicing, width: Semitones) -> Self {
        let width = max(width, MIN_CHART_WIDTH);

        assert!(voicing.get_span() <= width);

        Self { voicing, width }
    }

    /// Determine from which fret to show the fretboard.
    ///
    /// If the rightmost fret fits on the diagram, show the fretboard
    /// beginning at the first fret, otherwise use the leftmost fret
    /// needed for the chords to be played.
    pub fn get_base_fret(&self) -> FretID {
        let max_fret = self.voicing.get_max_fret();

        match max_fret {
            max_fret if max_fret <= self.width => 1,
            _ => self.voicing.get_min_pressed_fret(),
        }
    }

    /// Get the width of the space that we need to print the names
    /// of the root notes (the names of the strings).
    pub fn get_root_width(&self) -> usize {
        self.voicing
            .roots()
            .map(|n| n.to_string().len())
            .max()
            .unwrap()
    }

    /// Format a line that represents a ukulele string in a chord diagram.
    pub fn format_line(
        &self,
        uke_string: UkeString,
        base_fret: FretID,
        root_width: usize,
        finger: u8,
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
        let s: String = (base_fret..base_fret + self.width)
            .map(|i| {
                if fret == i {
                    finger.to_string()
                } else {
                    "-".to_string()
                }
            })
            .map(|c| format!("-{}-|", c))
            .collect();

        format!("{} {}{}{}- {}\n", root_str, sym, nut, s, note)
    }
}

impl fmt::Display for ChordChart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Determine from which fret to show the fretboard.
        let base_fret = self.get_base_fret();

        // Get the width of the space that we need to print the name
        // of the root notes (the names of the strings).
        let root_width = self.get_root_width();

        let fingers_on_strings = self.voicing.fingers_on_strings();

        // Create a diagram for each ukulele string.
        let s: String = self
            .voicing
            .uke_strings()
            .rev()
            .zip(fingers_on_strings.iter().rev())
            .map(|(us, f)| self.format_line(*us, base_fret, root_width, *f))
            .collect();

        // If the fretboard section shown does not include the nut,
        // indicate the number of the first fret shown.
        if base_fret > 1 {
            return writeln!(f, "{}{:width$}", s, base_fret, width = root_width + 6);
        }

        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use rstest::rstest;

    use super::*;
    use crate::{Chord, Tuning, VoicingConfig};

    #[rstest(chord, tuning, diagram,
        case(
            "C",
            Tuning::C,
            indoc!("
                A  ||---|---|-3-|---|- C
                E o||---|---|---|---|- E
                C o||---|---|---|---|- C
                G o||---|---|---|---|- G
            "),
        ),
        case(
            "C#",
            Tuning::C,
            indoc!("
                A  ||---|---|---|-4-|- C#
                E  ||-1-|---|---|---|- F
                C  ||-1-|---|---|---|- C#
                G  ||-1-|---|---|---|- G#
            ")
        ),
        case(
            "Db",
            Tuning::C,
            indoc!("
                A  ||---|---|---|-4-|- Db
                E  ||-1-|---|---|---|- F
                C  ||-1-|---|---|---|- Db
                G  ||-1-|---|---|---|- Ab
            ")
        ),
        case(
            "C#m",
            Tuning::C,
            indoc!("
                A  ||---|---|---|-4-|- C#
                E o||---|---|---|---|- E
                C  ||-2-|---|---|---|- C#
                G  ||-1-|---|---|---|- G#
            ")
        ),
        case(
            "Dbm",
            Tuning::C,
            indoc!("
                A  ||---|---|---|-4-|- Db
                E o||---|---|---|---|- E
                C  ||-2-|---|---|---|- Db
                G  ||-1-|---|---|---|- Ab
            ")
        ),
        case(
            "D",
            Tuning::D,
            indoc!("
                B   ||---|---|-3-|---|- D
                F# o||---|---|---|---|- F#
                D  o||---|---|---|---|- D
                A  o||---|---|---|---|- A
            "),
        ),
        case(
            "G",
            Tuning::G,
            indoc!("
                E  ||---|---|-3-|---|- G
                B o||---|---|---|---|- B
                G o||---|---|---|---|- G
                D o||---|---|---|---|- D
            "),
        ),
    )]
    fn test_to_diagram(chord: Chord, tuning: Tuning, diagram: &str) {
        let config = VoicingConfig {
            tuning,
            ..Default::default()
        };
        let voicing = chord.voicings(config).next().unwrap();
        let chord_chart = ChordChart::new(voicing, 4);
        assert_eq!(chord_chart.to_string(), diagram);
    }
}
