use crate::chord::Chord;
use crate::chord::ChordShapeSet;
use crate::ukulele::Streng;
use crate::ukulele::CHART_WIDTH;
use crate::Frets;
use crate::STRING_COUNT;
use std::fmt;

/// A ukulele.
pub struct Ukulele {
    strings: [Streng; STRING_COUNT],
    base_fret: Frets,
}

impl Ukulele {
    pub fn new() -> Self {
        Self {
            strings: [
                Streng::from("G"),
                Streng::from("C"),
                Streng::from("E"),
                Streng::from("A"),
            ],
            /// The first fret from which to show the fretboard chart.
            /// Corresponds to the position of the chord if `base_fret == 1`
            /// or `base_fret > CHART_WIDTH`.
            /// https://en.wikipedia.org/wiki/Position_(music)
            base_fret: 1,
        }
    }

    /// Play `chord` starting from fret number `min_fret`.
    pub fn play(&mut self, chord: &Chord, min_fret: Frets) {
        let chord_shapes = ChordShapeSet::new(chord.quality);

        let frets = chord_shapes.get_config(chord, min_fret);

        // Determine from which fret to show the fretboard.
        let mut base_fret = self.base_fret;
        let max_fret = *frets.iter().max().unwrap();

        if max_fret > CHART_WIDTH {
            base_fret = *frets.iter().min().unwrap();
            self.base_fret = base_fret;
        }

        self.strings
            .iter_mut()
            .zip(&frets)
            .for_each(|(s, f)| s.play(*f, base_fret));
    }
}

impl Default for Ukulele {
    fn default() -> Self {
        Self::new()
    }
}

/// Display the ukulele's fretboard in ASCII art.
impl fmt::Display for Ukulele {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = "".to_owned();

        for str in self.strings.iter().rev() {
            s.push_str(&format!("{}\n", str));
        }

        // If the fretboard section shown does not include the nut,
        // indicate the number of the first fret shown.
        if self.base_fret > 1 {
            s.push_str(&format!("      {}\n", self.base_fret))
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

    #[rstest_parametrize(chord, min_fret, display,
        case(
            "C",
            0,
            indoc!("
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
                A  -+-●-+---+---+---+ C
                E  -+-●-+---+---+---+ G
                C  -+---+-●-+---+---+ E
                G  -+---+---+-●-+---+ C
                      3
            ")
        ),
    )]
    fn test_play_and_display(chord: &str, min_fret: Frets, display: &str) {
        let mut uke = Ukulele::new();
        uke.play(&Chord::from_str(chord).unwrap(), min_fret);
        assert_eq!(format!("{}", uke), display);
    }
}
