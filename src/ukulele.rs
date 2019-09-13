use crate::chord::Chord;
use crate::chord::ChordQuality;
use crate::note::Note;
use crate::note::PitchClass;
use crate::streng::Streng;
use crate::Frets;
use std::fmt;

const STRING_COUNT: usize = 4;
type FretPattern = [Frets; STRING_COUNT];

/// A chord shape is a configuration of frets to be pressed to play a
/// chord with a certain chord quality. The shape can be moved along
/// the fretboard to derive several chords.
///
/// http://play-ukulele.simonplantinga.nl/2014/05/ukulele-chords-iii/
/// https://newhamukes.wordpress.com/2013/08/30/moveable-chords/
/// http://kauairainbow.com/Ukulele/Chord%20Magic/cm1.html
#[derive(Debug, Clone, Copy)]
struct ChordShape {
    root: Note,
    frets: FretPattern,
}

impl ChordShape {
    fn new(pitch_class: PitchClass, frets: FretPattern) -> Self {
        Self {
            root: Note::from(pitch_class),
            frets,
        }
    }

    /// Apply the chord shape while moving it `n` frets forward on the fretboard.
    /// Return the resulting fret pattern.
    fn apply(self, n: Frets) -> FretPattern {
        let mut frets = self.frets;

        for f in &mut frets[..] {
            *f += n;
        }

        frets
    }
}

/// A set of chord shapes to be used for a certain instrument -
/// in our case the ukulele.
struct ChordShapeSet {
    chord_shapes: Vec<ChordShape>,
}

impl ChordShapeSet {
    fn new(chord_quality: ChordQuality) -> Self {
        use PitchClass::*;

        let chord_shapes = match chord_quality {
            ChordQuality::Major => vec![
                ChordShape::new(C, [0, 0, 0, 3]),
                ChordShape::new(A, [2, 1, 0, 0]),
                ChordShape::new(G, [0, 2, 3, 2]),
                ChordShape::new(F, [2, 0, 1, 0]),
                ChordShape::new(D, [2, 2, 2, 0]),
            ],
            ChordQuality::Minor => vec![
                ChordShape::new(C, [0, 3, 3, 3]),
                ChordShape::new(A, [2, 0, 0, 0]),
                ChordShape::new(G, [0, 2, 3, 1]),
                ChordShape::new(F, [1, 0, 1, 3]),
                ChordShape::new(D, [2, 2, 1, 0]),
            ],
        };

        Self { chord_shapes }
    }

    /// Return a fret pattern to play `chord` starting from fret number `min_fret`.
    fn get_config(self, chord: &Chord, min_fret: Frets) -> FretPattern {
        let (chord_shape, diff) = self
            .chord_shapes
            .into_iter()
            .map(|cs| (cs, (chord.root - min_fret) - cs.root))
            .min_by_key(|&(_cs, diff)| diff)
            .unwrap();

        chord_shape.apply(min_fret + diff)
    }
}

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
    pub fn play(&mut self, chord: &Chord, min_fret: Frets) {
        let chord_shapes = ChordShapeSet::new(chord.quality);

        let frets = chord_shapes.get_config(chord, min_fret);

        self.strings
            .iter_mut()
            .zip(&frets)
            .for_each(|(s, f)| s.play(*f));
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
    fn test_play_and_display(chord: &str, min_fret: Frets, display: &str) {
        let mut uke = Ukulele::new();
        uke.play(&Chord::from_str(chord).unwrap(), min_fret);
        assert_eq!(format!("{}", uke), display);
    }
}
