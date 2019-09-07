use crate::chord::Chord;
use crate::streng::Streng;
use std::fmt;

const STRING_COUNT: usize = 4;

/// A ukulele.
pub struct Ukulele {
    strings: [Streng; STRING_COUNT],
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
        }
    }

    /// Play `chord` starting from fret number `min_fret`.
    pub fn play(&mut self, chord: &Chord, min_fret: usize) {
        for s in &mut self.strings {
            s.play_note(chord, min_fret);
        }
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
            ")
        ),
    )]
    fn test_play_and_display(chord: &str, min_fret: usize, display: &str) {
        let mut uke = Ukulele::new();
        uke.play(&Chord::from_str(chord).unwrap(), min_fret);
        assert_eq!(format!("{}", uke), display);
    }
}
