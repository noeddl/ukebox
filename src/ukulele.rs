use crate::chord::Chord;
use crate::string::String;
use std::fmt;

const STRING_COUNT: usize = 4;

/// A ukulele.
pub struct Ukulele<'a> {
    strings: [String<'a>; STRING_COUNT],
}

impl Ukulele<'_> {
    pub fn new() -> Self {
        Self {
            strings: [
                String::from("G"),
                String::from("C"),
                String::from("E"),
                String::from("A"),
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

/// Display the ukulele's fretboard in ASCII art.
impl fmt::Display for Ukulele<'_> {
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
    extern crate rstest;
    use super::*;
    use indoc::indoc;
    use rstest::rstest_parametrize;

    #[rstest_parametrize(chord, min_fret, display,
        case(
            "C",
            0,
            indoc!("
                ---+---+-●-+---+---+
                ---+---+---+---+---+
                ---+---+---+---+---+
                ---+---+---+---+---+
            ")
        ),
    )]
    fn test_play_and_display(chord: &str, min_fret: usize, display: &str) {
        let mut uke = Ukulele::new();
        uke.play(&Chord::from(chord), min_fret);
        assert_eq!(format!("{}", uke), display);
    }
}
