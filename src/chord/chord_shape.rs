use crate::chord::ChordType;
use crate::chord::FretID;
use crate::chord::Tuning;
use crate::diagram::FretPattern;
use crate::note::Note;
use crate::note::Semitones;
use std::str::FromStr;

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
}

impl ChordShape {
    fn new(note_name: &str, frets: impl Into<FretPattern>) -> Self {
        Self {
            root: Note::from_str(note_name).unwrap(),
            frets: frets.into(),
        }
    }

    /// Apply the chord shape while moving it `n` frets forward on the fretboard.
    /// Return the resulting fret pattern.
    fn apply(self, n: Semitones) -> FretPattern {
        self.frets + n
    }
}

/// A set of chord shapes to be used for a certain instrument -
/// in our case the ukulele.
pub struct ChordShapeSet {
    chord_shapes: Vec<ChordShape>,
}

impl ChordShapeSet {
    pub fn new(chord_type: ChordType) -> Self {
        use ChordType::*;

        let chord_shapes = match chord_type {
            Major => vec![
                ChordShape::new("C", [0, 0, 0, 3]),
                ChordShape::new("A", [2, 1, 0, 0]),
                ChordShape::new("G", [0, 2, 3, 2]),
                ChordShape::new("F", [2, 0, 1, 0]),
                ChordShape::new("D", [2, 2, 2, 0]),
            ],
            Minor => vec![
                ChordShape::new("C", [0, 3, 3, 3]),
                ChordShape::new("A", [2, 0, 0, 0]),
                ChordShape::new("G", [0, 2, 3, 1]),
                ChordShape::new("F", [1, 0, 1, 3]),
                ChordShape::new("D", [2, 2, 1, 0]),
            ],
            SuspendedSecond => vec![
                ChordShape::new("C", [0, 2, 3, 3]),
                ChordShape::new("A#", [3, 0, 1, 1]),
                //ChordShape::new("A", [2, 4, 0, 2]),
                ChordShape::new("G", [0, 2, 3, 0]),
                ChordShape::new("F", [0, 0, 1, 3]),
                ChordShape::new("D", [2, 2, 0, 0]),
            ],
            SuspendedFourth => vec![
                ChordShape::new("C", [0, 0, 1, 3]),
                ChordShape::new("A", [2, 2, 0, 0]),
                ChordShape::new("G", [0, 2, 3, 3]),
                ChordShape::new("F", [3, 0, 1, 1]),
                //ChordShape::new("E", [4, 4, 0, 0]),
                ChordShape::new("D", [0, 2, 3, 0]),
            ],
            Augmented => vec![
                ChordShape::new("C", [1, 0, 0, 3]),
                ChordShape::new("A", [2, 1, 1, 0]),
                ChordShape::new("G#", [1, 0, 0, 3]),
                // TODO: This pattern does also apply to Baug and D#aug.
                // Fix e.g. when dealing with issue #11.
                //ChordShape::new("G", [0, 3, 3, 2]),
                ChordShape::new("F", [2, 1, 1, 0]),
                ChordShape::new("E", [1, 0, 0, 3]),
                ChordShape::new("C#", [2, 1, 1, 0]),
            ],
            // Yikes, diminished chords are kind of tricky. In the lower positions,
            // some of them require either that certain strings are not played or to
            // stretch your fingers across more than 4 frets. So let's stick to the
            // higher positions in these cases for now.
            Diminished => vec![
                ChordShape::new("D", [7, 5, 4, 5]),
                ChordShape::new("A#", [3, 1, 0, 1]),
                ChordShape::new("G", [0, 1, 3, 1]),
                ChordShape::new("F#", [2, 0, 2, 0]),
                ChordShape::new("D#", [2, 3, 2, 0]),
            ],
            DominantSeventh => vec![
                ChordShape::new("C", [0, 0, 0, 1]),
                ChordShape::new("A", [0, 1, 0, 0]),
                ChordShape::new("G", [0, 2, 1, 2]),
                ChordShape::new("E", [1, 2, 0, 2]),
            ],
            MinorSeventh => vec![
                ChordShape::new("C#", [1, 1, 0, 2]),
                ChordShape::new("A", [0, 0, 0, 0]),
                ChordShape::new("G", [0, 2, 1, 1]),
                ChordShape::new("E", [0, 2, 0, 2]),
            ],
            MajorSeventh => vec![
                ChordShape::new("C", [0, 0, 0, 2]),
                ChordShape::new("A#", [3, 2, 1, 0]),
                ChordShape::new("A", [1, 1, 0, 0]),
                ChordShape::new("G", [0, 2, 2, 2]),
                ChordShape::new("E", [1, 3, 0, 2]),
            ],
            MinorMajorSeventh => vec![
                ChordShape::new("C#", [1, 1, 0, 3]),
                ChordShape::new("A#", [3, 1, 1, 0]),
                ChordShape::new("A", [1, 0, 0, 0]),
                // The shape for G#mMaj7 would be convenient to play, but not so much when
                // moving it further along the fretboard (spreads across 5 frets then).
                //ChordShape::new("G#", [0, 3, 4, 2]),
                ChordShape::new("G", [0, 2, 2, 1]),
                ChordShape::new("E", [0, 3, 0, 2]),
            ],
            // The augmented seventh shape set is (except for the missing D shape)
            // very similar to the one for dominant seventh chords. All the perfect
            // fifth become augmented and thus move one fret up. How convenient!
            AugmentedSeventh => vec![
                ChordShape::new("C", [1, 0, 0, 1]),
                ChordShape::new("A", [0, 1, 1, 0]),
                ChordShape::new("G", [0, 3, 1, 2]),
                ChordShape::new("E", [1, 2, 0, 3]),
            ],
            AugmentedMajorSeventh => vec![
                ChordShape::new("C", [1, 0, 0, 2]),
                ChordShape::new("A#", [3, 2, 2, 0]),
                ChordShape::new("A", [1, 1, 1, 0]),
                ChordShape::new("G", [0, 3, 2, 2]),
                ChordShape::new("E", [1, 3, 0, 3]),
            ],
            // The coolest shape set of all: The pattern stays the same all the time,
            // only the positions of the intervals change.
            DiminishedSeventh => vec![
                ChordShape::new("C#", [0, 1, 0, 1]),
                ChordShape::new("A#", [0, 1, 0, 1]),
                ChordShape::new("G", [0, 1, 0, 1]),
                ChordShape::new("E", [0, 1, 0, 1]),
            ],
            // These are the same as for the diminished seventh chords but the sevenths
            // move one fret up because they are minor instead of diminished.
            HalfDiminishedSeventh => vec![
                ChordShape::new("C#", [0, 1, 0, 2]),
                ChordShape::new("A#", [1, 1, 0, 1]),
                ChordShape::new("G", [0, 1, 1, 1]),
                ChordShape::new("E", [0, 2, 0, 1]),
            ],
        };

        Self { chord_shapes }
    }

    /// Return a fret pattern to play `chord` starting from fret number `min_fret`.
    pub fn get_config(self, root: Note, min_fret: FretID, tuning: Tuning) -> FretPattern {
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
