use crate::chord::ChordQuality;
use crate::note::Interval;
use crate::note::Note;
use crate::FretPattern;
use crate::Frets;
use crate::IntervalPattern;
use crate::STRING_COUNT;
use std::str::FromStr;

/// A chord shape is a configuration of frets to be pressed to play a
/// chord with a certain chord quality. The shape can be moved along
/// the fretboard to derive several chords.
///
/// http://play-ukulele.simonplantinga.nl/2014/05/ukulele-chords-iii/
/// https://newhamukes.wordpress.com/2013/08/30/moveable-chords/
/// http://kauairainbow.com/Ukulele/Chord%20Magic/cm1.html
#[derive(Debug, Clone, Copy)]
pub struct ChordShape {
    root: Note,
    frets: FretPattern,
    intervals: IntervalPattern,
}

impl ChordShape {
    fn new(note_name: &str, frets: FretPattern, int_names: [&str; STRING_COUNT]) -> Self {
        let mut intervals = [Interval::PerfectUnison; STRING_COUNT];

        for (i, s) in int_names.iter().enumerate() {
            intervals[i] = Interval::from_str(s).unwrap();
        }

        Self {
            root: Note::from_str(note_name).unwrap(),
            frets,
            intervals,
        }
    }

    /// Apply the chord shape while moving it `n` frets forward on the fretboard.
    /// Return the resulting fret pattern.
    fn apply(self, n: Frets) -> (FretPattern, IntervalPattern) {
        let mut frets = self.frets;

        for f in &mut frets[..] {
            *f += n;
        }

        (frets, self.intervals)
    }
}

/// A set of chord shapes to be used for a certain instrument -
/// in our case the ukulele.
pub struct ChordShapeSet {
    chord_shapes: Vec<ChordShape>,
}

impl ChordShapeSet {
    pub fn new(chord_quality: ChordQuality) -> Self {
        let chord_shapes = match chord_quality {
            ChordQuality::Major => vec![
                ChordShape::new("C", [0, 0, 0, 3], ["P5", "P1", "M3", "P1"]),
                ChordShape::new("A", [2, 1, 0, 0], ["P1", "M3", "P5", "P1"]),
                ChordShape::new("G", [0, 2, 3, 2], ["P1", "P5", "P1", "M3"]),
                ChordShape::new("F", [2, 0, 1, 0], ["M3", "P5", "P1", "M3"]),
                ChordShape::new("D", [2, 2, 2, 0], ["P5", "P1", "M3", "P5"]),
            ],
            ChordQuality::Minor => vec![
                ChordShape::new("C", [0, 3, 3, 3], ["P5", "m3", "P5", "P1"]),
                ChordShape::new("A", [2, 0, 0, 0], ["P1", "m3", "P5", "P1"]),
                ChordShape::new("G", [0, 2, 3, 1], ["P1", "P5", "P1", "m3"]),
                ChordShape::new("F", [1, 0, 1, 3], ["m3", "P5", "P1", "P5"]),
                ChordShape::new("D", [2, 2, 1, 0], ["P5", "P1", "m3", "P5"]),
            ],
        };

        Self { chord_shapes }
    }

    /// Return a fret pattern to play `chord` starting from fret number `min_fret`.
    pub fn get_config(self, root: Note, min_fret: Frets) -> (FretPattern, IntervalPattern) {
        let (chord_shape, diff) = self
            .chord_shapes
            .into_iter()
            .map(|cs| (cs, (root.pitch_class - min_fret) - cs.root.pitch_class))
            .min_by_key(|&(_cs, diff)| diff)
            .unwrap();

        chord_shape.apply(min_fret + diff)
    }
}
