use crate::chord::ChordType;
use crate::chord::FretID;
use crate::chord::FretPattern;
use crate::chord::Tuning;
use crate::note::Interval;
use crate::note::Note;
use crate::note::Semitones;
use crate::STRING_COUNT;
use std::str::FromStr;

type IntervalPattern = [Interval; STRING_COUNT];

/// A chord shape is a configuration of frets to be pressed to play a
/// certain type of chord. The shape can be moved along the fretboard
/// to derive several chords.
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
    fn apply(self, n: Semitones) -> (FretPattern, IntervalPattern) {
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
    pub fn new(chord_type: ChordType) -> Self {
        let chord_shapes = match chord_type {
            ChordType::Major => vec![
                ChordShape::new("C", [0, 0, 0, 3], ["P5", "P1", "M3", "P1"]),
                ChordShape::new("A", [2, 1, 0, 0], ["P1", "M3", "P5", "P1"]),
                ChordShape::new("G", [0, 2, 3, 2], ["P1", "P5", "P1", "M3"]),
                ChordShape::new("F", [2, 0, 1, 0], ["M3", "P5", "P1", "M3"]),
                ChordShape::new("D", [2, 2, 2, 0], ["P5", "P1", "M3", "P5"]),
            ],
            ChordType::Minor => vec![
                ChordShape::new("C", [0, 3, 3, 3], ["P5", "m3", "P5", "P1"]),
                ChordShape::new("A", [2, 0, 0, 0], ["P1", "m3", "P5", "P1"]),
                ChordShape::new("G", [0, 2, 3, 1], ["P1", "P5", "P1", "m3"]),
                ChordShape::new("F", [1, 0, 1, 3], ["m3", "P5", "P1", "P5"]),
                ChordShape::new("D", [2, 2, 1, 0], ["P5", "P1", "m3", "P5"]),
            ],
            ChordType::Augmented => vec![
                ChordShape::new("C", [1, 0, 0, 3], ["A5", "P1", "M3", "P1"]),
                ChordShape::new("A", [2, 1, 1, 0], ["P1", "M3", "A5", "P1"]),
                ChordShape::new("G#", [1, 0, 0, 3], ["P1", "M3", "A5", "M3"]),
                ChordShape::new("G", [0, 3, 3, 2], ["P1", "A5", "P1", "M3"]),
                ChordShape::new("F", [2, 1, 1, 0], ["M3", "A5", "P1", "M3"]),
                ChordShape::new("E", [1, 0, 0, 3], ["M3", "A5", "P1", "A5"]),
                ChordShape::new("C#", [2, 1, 1, 0], ["A5", "P1", "M3", "A5"]),
            ],
            // Yikes, diminished chords are kind of tricky. In the lower positions,
            // some of them require either that certain strings are not played or to
            // stretch your fingers across more than 4 frets. So let's stick to the
            // higher positions in these cases for now.
            ChordType::Diminished => vec![
                ChordShape::new("D", [7, 5, 4, 5], ["P1", "m3", "d5", "P1"]),
                ChordShape::new("A#", [3, 1, 0, 1], ["P1", "m3", "d5", "P1"]),
                ChordShape::new("G", [0, 1, 3, 1], ["P1", "d5", "P1", "m3"]),
                ChordShape::new("F#", [2, 0, 2, 0], ["m3", "d5", "P1", "m3"]),
                ChordShape::new("D#", [2, 3, 2, 0], ["d5", "P1", "m3", "d5"]),
            ],
            ChordType::DominantSeventh => vec![
                ChordShape::new("C", [0, 0, 0, 1], ["P5", "P1", "M3", "m7"]),
                ChordShape::new("A", [0, 1, 0, 0], ["m7", "M3", "P5", "P1"]),
                ChordShape::new("G", [0, 2, 1, 2], ["P1", "P5", "m7", "M3"]),
                ChordShape::new("E", [1, 2, 0, 2], ["M3", "m7", "P1", "P5"]),
                ChordShape::new("D", [2, 0, 2, 0], ["P5", "m7", "M3", "P5"]),
            ],
            ChordType::MinorSeventh => vec![
                ChordShape::new("C#", [1, 1, 0, 2], ["P5", "P1", "m3", "m7"]),
                ChordShape::new("A", [0, 0, 0, 0], ["m7", "m3", "P5", "P1"]),
                ChordShape::new("G", [0, 2, 1, 1], ["P1", "P5", "m7", "m3"]),
                ChordShape::new("E", [0, 2, 0, 2], ["m3", "m7", "P1", "P5"]),
                ChordShape::new("D", [2, 0, 1, 0], ["P5", "m7", "m3", "P5"]),
            ],
            ChordType::MajorSeventh => vec![
                ChordShape::new("C", [0, 0, 0, 2], ["P5", "P1", "M3", "M7"]),
                ChordShape::new("A#", [3, 2, 1, 0], ["P1", "M3", "P5", "M7"]),
                ChordShape::new("A", [1, 1, 0, 0], ["M7", "M3", "P5", "P1"]),
                ChordShape::new("G", [0, 2, 2, 2], ["P1", "P5", "M7", "M3"]),
                ChordShape::new("E", [1, 3, 0, 2], ["M3", "M7", "P1", "P5"]),
            ],
            // The augmented seventh shape set is (except for the missing D shape)
            // very similar to the one for dominant seventh chords. All the perfect
            // fifth become augmented and thus move one fret up. How convenient!
            ChordType::AugmentedSeventh => vec![
                ChordShape::new("C", [1, 0, 0, 1], ["A5", "P1", "M3", "m7"]),
                ChordShape::new("A", [0, 1, 1, 0], ["m7", "M3", "A5", "P1"]),
                ChordShape::new("G", [0, 3, 1, 2], ["P1", "A5", "m7", "M3"]),
                ChordShape::new("E", [1, 2, 0, 3], ["M3", "m7", "P1", "A5"]),
            ],
            // The coolest shape set of all: The pattern stays the same all the time,
            // only the positions of the intervals change.
            ChordType::DiminishedSeventh => vec![
                ChordShape::new("C#", [0, 1, 0, 1], ["d5", "P1", "m3", "d7"]),
                ChordShape::new("A#", [0, 1, 0, 1], ["d7", "m3", "d5", "P1"]),
                ChordShape::new("G", [0, 1, 0, 1], ["P1", "d5", "d7", "m3"]),
                ChordShape::new("E", [0, 1, 0, 1], ["m3", "d7", "P1", "d5"]),
            ],
        };

        Self { chord_shapes }
    }

    /// Return a fret pattern to play `chord` starting from fret number `min_fret`.
    pub fn get_config(
        self,
        root: Note,
        min_fret: FretID,
        tuning: Tuning,
    ) -> (FretPattern, IntervalPattern) {
        let semitones = tuning.get_semitones();

        // Calculate offset of how far to move the chord shape on the fretboard.
        let get_offset =
            |cs: ChordShape| (root.pitch_class - min_fret) - (cs.root.pitch_class + semitones);

        let (chord_shape, offset) = self
            .chord_shapes
            .into_iter()
            .map(|cs| (cs, get_offset(cs)))
            .min_by_key(|&(_cs, offset)| offset)
            .unwrap();

        chord_shape.apply(min_fret + offset)
    }
}
