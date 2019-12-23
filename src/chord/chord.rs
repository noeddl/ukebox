use crate::chord::ChordShapeSet;
use crate::chord::FretID;
use crate::chord::Tuning;
use crate::diagram::ChordDiagram;
use crate::note::Interval;
use crate::note::Note;
use crate::STRING_COUNT;
use regex::Regex;
use std::fmt;
use std::str::FromStr;

/// Custom error for strings that cannot be parsed into chords.
#[derive(Debug)]
pub struct ParseChordError {
    name: String,
}

impl fmt::Display for ParseChordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse chord name \"{}\"", self.name)
    }
}

/// The type of the chord depending on the intervals it contains.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChordType {
    Major,
    Minor,
    SuspendedSecond,
    SuspendedFourth,
    Augmented,
    Diminished,
    DominantSeventh,
    MinorSeventh,
    MajorSeventh,
    MinorMajorSeventh,
    AugmentedSeventh,
    AugmentedMajorSeventh,
    DiminishedSeventh,
    HalfDiminishedSeventh,
}

impl ChordType {
    fn get_intervals(self) -> Vec<Interval> {
        use ChordType::*;

        let interval_names = match self {
            Major => vec!["P1", "M3", "P5"],
            Minor => vec!["P1", "m3", "P5"],
            SuspendedSecond => vec!["P1", "M2", "P5"],
            SuspendedFourth => vec!["P1", "P4", "P5"],
            Augmented => vec!["P1", "M3", "A5"],
            Diminished => vec!["P1", "m3", "d5"],
            DominantSeventh => vec!["P1", "M3", "P5", "m7"],
            MinorSeventh => vec!["P1", "m3", "P5", "m7"],
            MajorSeventh => vec!["P1", "M3", "P5", "M7"],
            MinorMajorSeventh => vec!["P1", "m3", "P5", "M7"],
            AugmentedSeventh => vec!["P1", "M3", "A5", "m7"],
            AugmentedMajorSeventh => vec!["P1", "M3", "A5", "M7"],
            DiminishedSeventh => vec!["P1", "m3", "d5", "d7"],
            HalfDiminishedSeventh => vec!["P1", "m3", "d5", "m7"],
        };

        interval_names
            .iter()
            .map(|s| Interval::from_str(s).unwrap())
            .collect()
    }
}

impl fmt::Display for ChordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ChordType::*;

        let s = match self {
            Major => "major",
            Minor => "minor",
            SuspendedSecond => "suspended 2nd",
            SuspendedFourth => "suspended 4th",
            Augmented => "augmented",
            Diminished => "diminished",
            DominantSeventh => "dominant 7th",
            MinorSeventh => "minor 7th",
            MajorSeventh => "major 7th",
            MinorMajorSeventh => "minor/major 7th",
            AugmentedSeventh => "augmented 7th",
            AugmentedMajorSeventh => "augmented major 7th",
            DiminishedSeventh => "diminished 7th",
            HalfDiminishedSeventh => "half-diminished 7th",
        };

        write!(f, "{}", s)
    }
}

/// A chord such as C, Cm and so on.
pub struct Chord {
    name: String,
    pub chord_type: ChordType,
    pub root: Note,
    notes: Vec<Note>,
}

impl Chord {
    pub fn contains(&self, note: Note) -> bool {
        self.notes.contains(&note)
    }

    pub fn get_diagram(self, min_fret: FretID, tuning: Tuning) -> ChordDiagram {
        let chord_shapes = ChordShapeSet::new(self.chord_type);

        let (frets, intervals) = chord_shapes.get_config(self.root, min_fret, tuning);

        let mut notes = [self.root; STRING_COUNT];

        for (i, interval) in intervals.iter().enumerate() {
            notes[i] = notes[i] + *interval;
        }

        ChordDiagram::new(self, frets, notes, tuning)
    }
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {} {}", self.name, self.notes[0], self.chord_type)
    }
}

impl FromStr for Chord {
    type Err = ParseChordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ChordType::*;

        let name = s.to_string();

        // Regular expression for chord names.
        let re = Regex::new(
            r"(?x)
                ^                               # match full string
                (?P<root>[CDEFGAB][\#b]?)       # root note including accidentals
                (?P<type>                       # chord type
                      sus(?:2|4)                # suspended chords
                    | aug(?:Maj)?7?             # augmented chords
                    | dim7?                     # diminished chords
                    | maj7                      # chords with a major 7th
                    | m?(?:(?:Maj)?7(?:b5)?)?)  # minor chords + alterations
                $                               # match full string
            ",
        )
        .unwrap();

        // Match regex.
        let caps = match re.captures(s) {
            Some(caps) => caps,
            None => return Err(ParseChordError { name }),
        };

        // Get root note.
        let root = match Note::from_str(&caps["root"]) {
            Ok(note) => note,
            Err(_) => return Err(ParseChordError { name }),
        };

        // Get chord type.
        let chord_type = match &caps["type"] {
            "m" => Minor,
            "sus2" => SuspendedSecond,
            "sus4" => SuspendedFourth,
            "aug" => Augmented,
            "dim" => Diminished,
            "7" => DominantSeventh,
            "m7" => MinorSeventh,
            "maj7" => MajorSeventh,
            "mMaj7" => MinorMajorSeventh,
            "aug7" => AugmentedSeventh,
            "augMaj7" => AugmentedMajorSeventh,
            "dim7" => DiminishedSeventh,
            "m7b5" => HalfDiminishedSeventh,
            "" => Major,
            _ => return Err(ParseChordError { name }),
        };

        // Collect notes of the chord.
        let mut notes = vec![];

        for interval in chord_type.get_intervals() {
            notes.push(root + interval);
        }

        Ok(Self {
            name,
            root,
            chord_type,
            notes,
        })
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::many_single_char_names)]
    use super::*;
    use rstest::rstest;

    #[rstest(
        chord,
        case("Z"),
        case("c"),
        case("ABC"),
        case("C7b5"),
        case("C#mb5"),
        case("C#mbla"),
        case("CmMaj"),
        case("CmMaj7b5")
    )]
    fn test_from_str_fail(chord: &str) {
        assert!(Chord::from_str(chord).is_err())
    }

    #[rstest(
        chord,
        root,
        third,
        fifth,
        case("C", "C", "E", "G"),
        case("C#", "C#", "F", "G#"),
        case("Db", "Db", "F", "Ab"),
        case("D", "D", "F#", "A"),
        case("D#", "D#", "G", "A#"),
        case("Eb", "Eb", "G", "Bb"),
        case("E", "E", "G#", "B"),
        case("F", "F", "A", "C"),
        case("F#", "F#", "A#", "C#"),
        case("Gb", "Gb", "Bb", "Db"),
        case("G", "G", "B", "D"),
        case("G#", "G#", "C", "D#"),
        case("Ab", "Ab", "C", "Eb"),
        case("A", "A", "C#", "E"),
        case("A#", "A#", "D", "F"),
        case("Bb", "Bb", "D", "F"),
        case("B", "B", "D#", "F#")
    )]
    fn test_from_str_major(chord: &str, root: &str, third: &str, fifth: &str) {
        let c = Chord::from_str(chord).unwrap();
        let r = Note::from_str(root).unwrap();
        let t = Note::from_str(third).unwrap();
        let f = Note::from_str(fifth).unwrap();
        assert_eq!(c.notes, vec![r, t, f]);
        assert_eq!(c.chord_type, ChordType::Major);
    }

    #[rstest(
        chord,
        root,
        third,
        fifth,
        case("Cm", "C", "Eb", "G"),
        case("C#m", "C#", "E", "G#"),
        case("Dbm", "Db", "E", "Ab"),
        case("Dm", "D", "F", "A"),
        case("D#m", "D#", "F#", "A#"),
        case("Ebm", "Eb", "Gb", "Bb"),
        case("Em", "E", "G", "B"),
        case("Fm", "F", "Ab", "C"),
        case("F#m", "F#", "A", "C#"),
        case("Gbm", "Gb", "A", "Db"),
        case("Gm", "G", "Bb", "D"),
        case("G#m", "G#", "B", "D#"),
        case("Abm", "Ab", "B", "Eb"),
        case("Am", "A", "C", "E"),
        case("A#m", "A#", "C#", "F"),
        case("Bbm", "Bb", "Db", "F"),
        case("Bm", "B", "D", "F#")
    )]
    fn test_from_str_minor(chord: &str, root: &str, third: &str, fifth: &str) {
        let c = Chord::from_str(chord).unwrap();
        let r = Note::from_str(root).unwrap();
        let t = Note::from_str(third).unwrap();
        let f = Note::from_str(fifth).unwrap();
        assert_eq!(c.notes, vec![r, t, f]);
        assert_eq!(c.chord_type, ChordType::Minor);
    }

    #[rstest(
        chord,
        root,
        third,
        fifth,
        case("Csus2", "C", "D", "G"),
        case("C#sus2", "C#", "D#", "G#"),
        case("Dbsus2", "Db", "Eb", "Ab"),
        case("Dsus2", "D", "E", "A"),
        case("D#sus2", "D#", "F", "A#"),
        case("Ebsus2", "Eb", "F", "Bb"),
        case("Esus2", "E", "F#", "B"),
        case("Fsus2", "F", "G", "C"),
        case("F#sus2", "F#", "G#", "C#"),
        case("Gbsus2", "Gb", "Ab", "Db"),
        case("Gsus2", "G", "A", "D"),
        case("G#sus2", "G#", "A#", "D#"),
        case("Absus2", "Ab", "Bb", "Eb"),
        case("Asus2", "A", "B", "E"),
        case("A#sus2", "A#", "C", "F"),
        case("Bbsus2", "Bb", "C", "F"),
        case("Bsus2", "B", "C#", "F#")
    )]
    fn test_from_str_suspended_second(chord: &str, root: &str, third: &str, fifth: &str) {
        let c = Chord::from_str(chord).unwrap();
        let r = Note::from_str(root).unwrap();
        let t = Note::from_str(third).unwrap();
        let f = Note::from_str(fifth).unwrap();
        assert_eq!(c.notes, vec![r, t, f]);
        assert_eq!(c.chord_type, ChordType::SuspendedSecond);
    }

    #[rstest(
        chord,
        root,
        third,
        fifth,
        case("Csus4", "C", "F", "G"),
        case("C#sus4", "C#", "F#", "G#"),
        case("Dbsus4", "Db", "Gb", "Ab"),
        case("Dsus4", "D", "G", "A"),
        case("D#sus4", "D#", "G#", "A#"),
        case("Ebsus4", "Eb", "Ab", "Bb"),
        case("Esus4", "E", "A", "B"),
        case("Fsus4", "F", "Bb", "C"),
        case("F#sus4", "F#", "B", "C#"),
        case("Gbsus4", "Gb", "B", "Db"),
        case("Gsus4", "G", "C", "D"),
        case("G#sus4", "G#", "C#", "D#"),
        case("Absus4", "Ab", "Db", "Eb"),
        case("Asus4", "A", "D", "E"),
        case("A#sus4", "A#", "D#", "F"),
        case("Bbsus4", "Bb", "Eb", "F"),
        case("Bsus4", "B", "E", "F#")
    )]
    fn test_from_str_suspended_fourth(chord: &str, root: &str, third: &str, fifth: &str) {
        let c = Chord::from_str(chord).unwrap();
        let r = Note::from_str(root).unwrap();
        let t = Note::from_str(third).unwrap();
        let f = Note::from_str(fifth).unwrap();
        assert_eq!(c.notes, vec![r, t, f]);
        assert_eq!(c.chord_type, ChordType::SuspendedFourth);
    }

    #[rstest(
        chord,
        root,
        third,
        fifth,
        case("Caug", "C", "E", "G#"),
        case("C#aug", "C#", "F", "A"),
        case("Dbaug", "Db", "F", "A"),
        case("Daug", "D", "F#", "A#"),
        case("D#aug", "D#", "G", "B"),
        case("Ebaug", "Eb", "G", "B"),
        case("Eaug", "E", "G#", "C"),
        case("Faug", "F", "A", "C#"),
        case("F#aug", "F#", "A#", "D"),
        case("Gbaug", "Gb", "Bb", "D"),
        case("Gaug", "G", "B", "D#"),
        case("G#aug", "G#", "C", "E"),
        case("Abaug", "Ab", "C", "E"),
        case("Aaug", "A", "C#", "F"),
        case("A#aug", "A#", "D", "F#"),
        case("Bbaug", "Bb", "D", "F#"),
        case("Baug", "B", "D#", "G")
    )]
    fn test_from_str_augmented(chord: &str, root: &str, third: &str, fifth: &str) {
        let c = Chord::from_str(chord).unwrap();
        let r = Note::from_str(root).unwrap();
        let t = Note::from_str(third).unwrap();
        let f = Note::from_str(fifth).unwrap();
        assert_eq!(c.notes, vec![r, t, f]);
        assert_eq!(c.chord_type, ChordType::Augmented);
    }

    #[rstest(
        chord,
        root,
        third,
        fifth,
        case("Cdim", "C", "Eb", "Gb"),
        case("C#dim", "C#", "E", "G"),
        case("Dbdim", "Db", "E", "G"),
        case("Ddim", "D", "F", "Ab"),
        case("D#dim", "D#", "F#", "A"),
        case("Ebdim", "Eb", "Gb", "A"),
        case("Edim", "E", "G", "Bb"),
        case("Fdim", "F", "Ab", "B"),
        case("F#dim", "F#", "A", "C"),
        case("Gbdim", "Gb", "A", "C"),
        case("Gdim", "G", "Bb", "Db"),
        case("G#dim", "G#", "B", "D"),
        case("Abdim", "Ab", "B", "D"),
        case("Adim", "A", "C", "Eb"),
        case("A#dim", "A#", "C#", "E"),
        case("Bbdim", "Bb", "Db", "E"),
        case("Bdim", "B", "D", "F")
    )]
    fn test_from_str_diminished(chord: &str, root: &str, third: &str, fifth: &str) {
        let c = Chord::from_str(chord).unwrap();
        let r = Note::from_str(root).unwrap();
        let t = Note::from_str(third).unwrap();
        let f = Note::from_str(fifth).unwrap();
        assert_eq!(c.notes, vec![r, t, f]);
        assert_eq!(c.chord_type, ChordType::Diminished);
    }

    #[rstest(
        chord,
        root,
        third,
        fifth,
        seventh,
        case("C7", "C", "E", "G", "Bb"),
        case("C#7", "C#", "F", "G#", "B"),
        case("Db7", "Db", "F", "Ab", "B"),
        case("D7", "D", "F#", "A", "C"),
        case("D#7", "D#", "G", "A#", "C#"),
        case("Eb7", "Eb", "G", "Bb", "Db"),
        case("E7", "E", "G#", "B", "D"),
        case("F7", "F", "A", "C", "Eb"),
        case("F#7", "F#", "A#", "C#", "E"),
        case("Gb7", "Gb", "Bb", "Db", "E"),
        case("G7", "G", "B", "D", "F"),
        case("G#7", "G#", "C", "D#", "F#"),
        case("Ab7", "Ab", "C", "Eb", "Gb"),
        case("A7", "A", "C#", "E", "G"),
        case("A#7", "A#", "D", "F", "G#"),
        case("Bb7", "Bb", "D", "F", "Ab"),
        case("B7", "B", "D#", "F#", "A")
    )]
    fn test_from_str_dominant_seventh(
        chord: &str,
        root: &str,
        third: &str,
        fifth: &str,
        seventh: &str,
    ) {
        let c = Chord::from_str(chord).unwrap();
        let r = Note::from_str(root).unwrap();
        let t = Note::from_str(third).unwrap();
        let f = Note::from_str(fifth).unwrap();
        let s = Note::from_str(seventh).unwrap();
        assert_eq!(c.notes, vec![r, t, f, s]);
        assert_eq!(c.chord_type, ChordType::DominantSeventh);
    }

    #[rstest(
        chord,
        root,
        third,
        fifth,
        seventh,
        case("Cm7", "C", "Eb", "G", "Bb"),
        case("C#m7", "C#", "E", "G#", "B"),
        case("Dbm7", "Db", "E", "Ab", "B"),
        case("Dm7", "D", "F", "A", "C"),
        case("D#m7", "D#", "F#", "A#", "C#"),
        case("Ebm7", "Eb", "Gb", "Bb", "Db"),
        case("Em7", "E", "G", "B", "D"),
        case("Fm7", "F", "Ab", "C", "Eb"),
        case("F#m7", "F#", "A", "C#", "E"),
        case("Gbm7", "Gb", "A", "Db", "E"),
        case("Gm7", "G", "Bb", "D", "F"),
        case("G#m7", "G#", "B", "D#", "F#"),
        case("Abm7", "Ab", "B", "Eb", "Gb"),
        case("Am7", "A", "C", "E", "G"),
        case("A#m7", "A#", "C#", "F", "G#"),
        case("Bbm7", "Bb", "Db", "F", "Ab"),
        case("Bm7", "B", "D", "F#", "A")
    )]
    fn test_from_str_minor_seventh(
        chord: &str,
        root: &str,
        third: &str,
        fifth: &str,
        seventh: &str,
    ) {
        let c = Chord::from_str(chord).unwrap();
        let r = Note::from_str(root).unwrap();
        let t = Note::from_str(third).unwrap();
        let f = Note::from_str(fifth).unwrap();
        let s = Note::from_str(seventh).unwrap();
        assert_eq!(c.notes, vec![r, t, f, s]);
        assert_eq!(c.chord_type, ChordType::MinorSeventh);
    }

    #[rstest(
        chord,
        root,
        third,
        fifth,
        seventh,
        case("Cmaj7", "C", "E", "G", "B"),
        case("C#maj7", "C#", "F", "G#", "C"),
        case("Dbmaj7", "Db", "F", "Ab", "C"),
        case("Dmaj7", "D", "F#", "A", "C#"),
        case("D#maj7", "D#", "G", "A#", "D"),
        case("Ebmaj7", "Eb", "G", "Bb", "D"),
        case("Emaj7", "E", "G#", "B", "D#"),
        case("Fmaj7", "F", "A", "C", "E"),
        case("F#maj7", "F#", "A#", "C#", "F"),
        case("Gbmaj7", "Gb", "Bb", "Db", "F"),
        case("Gmaj7", "G", "B", "D", "F#"),
        case("G#maj7", "G#", "C", "D#", "G"),
        case("Abmaj7", "Ab", "C", "Eb", "G"),
        case("Amaj7", "A", "C#", "E", "G#"),
        case("A#maj7", "A#", "D", "F", "A"),
        case("Bbmaj7", "Bb", "D", "F", "A"),
        case("Bmaj7", "B", "D#", "F#", "A#")
    )]
    fn test_from_str_major_seventh(
        chord: &str,
        root: &str,
        third: &str,
        fifth: &str,
        seventh: &str,
    ) {
        let c = Chord::from_str(chord).unwrap();
        let r = Note::from_str(root).unwrap();
        let t = Note::from_str(third).unwrap();
        let f = Note::from_str(fifth).unwrap();
        let s = Note::from_str(seventh).unwrap();
        assert_eq!(c.notes, vec![r, t, f, s]);
        assert_eq!(c.chord_type, ChordType::MajorSeventh);
    }

    #[rstest(
        chord,
        root,
        third,
        fifth,
        seventh,
        case("CmMaj7", "C", "Eb", "G", "B"),
        case("C#mMaj7", "C#", "E", "G#", "C"),
        case("DbmMaj7", "Db", "E", "Ab", "C"),
        case("DmMaj7", "D", "F", "A", "C#"),
        case("D#mMaj7", "D#", "F#", "A#", "D"),
        case("EbmMaj7", "Eb", "Gb", "Bb", "D"),
        case("EmMaj7", "E", "G", "B", "D#"),
        case("FmMaj7", "F", "Ab", "C", "E"),
        case("F#mMaj7", "F#", "A", "C#", "F"),
        case("GbmMaj7", "Gb", "A", "Db", "F"),
        case("GmMaj7", "G", "Bb", "D", "F#"),
        case("G#mMaj7", "G#", "B", "D#", "G"),
        case("AbmMaj7", "Ab", "B", "Eb", "G"),
        case("AmMaj7", "A", "C", "E", "G#"),
        case("A#mMaj7", "A#", "C#", "F", "A"),
        case("BbmMaj7", "Bb", "Db", "F", "A"),
        case("BmMaj7", "B", "D", "F#", "A#")
    )]
    fn test_from_str_minor_major_seventh(
        chord: &str,
        root: &str,
        third: &str,
        fifth: &str,
        seventh: &str,
    ) {
        let c = Chord::from_str(chord).unwrap();
        let r = Note::from_str(root).unwrap();
        let t = Note::from_str(third).unwrap();
        let f = Note::from_str(fifth).unwrap();
        let s = Note::from_str(seventh).unwrap();
        assert_eq!(c.notes, vec![r, t, f, s]);
        assert_eq!(c.chord_type, ChordType::MinorMajorSeventh);
    }

    #[rstest(
        chord,
        root,
        third,
        fifth,
        seventh,
        case("Caug7", "C", "E", "G#", "Bb"),
        case("C#aug7", "C#", "F", "A", "B"),
        case("Dbaug7", "Db", "F", "A", "B"),
        case("Daug7", "D", "F#", "A#", "C"),
        case("D#aug7", "D#", "G", "B", "C#"),
        case("Ebaug7", "Eb", "G", "B", "Db"),
        case("Eaug7", "E", "G#", "C", "D"),
        case("Faug7", "F", "A", "C#", "Eb"),
        case("F#aug7", "F#", "A#", "D", "E"),
        case("Gbaug7", "Gb", "Bb", "D", "E"),
        case("Gaug7", "G", "B", "D#", "F"),
        case("G#aug7", "G#", "C", "E", "F#"),
        case("Abaug7", "Ab", "C", "E", "Gb"),
        case("Aaug7", "A", "C#", "F", "G"),
        case("A#aug7", "A#", "D", "F#", "G#"),
        case("Bbaug7", "Bb", "D", "F#", "Ab"),
        case("Baug7", "B", "D#", "G", "A")
    )]
    fn test_from_str_augmented_seventh(
        chord: &str,
        root: &str,
        third: &str,
        fifth: &str,
        seventh: &str,
    ) {
        let c = Chord::from_str(chord).unwrap();
        let r = Note::from_str(root).unwrap();
        let t = Note::from_str(third).unwrap();
        let f = Note::from_str(fifth).unwrap();
        let s = Note::from_str(seventh).unwrap();
        assert_eq!(c.notes, vec![r, t, f, s]);
        assert_eq!(c.chord_type, ChordType::AugmentedSeventh);
    }

    #[rstest(
        chord,
        root,
        third,
        fifth,
        seventh,
        case("CaugMaj7", "C", "E", "G#", "B"),
        case("C#augMaj7", "C#", "F", "A", "C"),
        case("DbaugMaj7", "Db", "F", "A", "C"),
        case("DaugMaj7", "D", "F#", "A#", "C#"),
        case("D#augMaj7", "D#", "G", "B", "D"),
        case("EbaugMaj7", "Eb", "G", "B", "D"),
        case("EaugMaj7", "E", "G#", "C", "D#"),
        case("FaugMaj7", "F", "A", "C#", "E"),
        case("F#augMaj7", "F#", "A#", "D", "F"),
        case("GbaugMaj7", "Gb", "Bb", "D", "F"),
        case("GaugMaj7", "G", "B", "D#", "F#"),
        case("G#augMaj7", "G#", "C", "E", "G"),
        case("AbaugMaj7", "Ab", "C", "E", "G"),
        case("AaugMaj7", "A", "C#", "F", "G#"),
        case("A#augMaj7", "A#", "D", "F#", "A"),
        case("BbaugMaj7", "Bb", "D", "F#", "A"),
        case("BaugMaj7", "B", "D#", "G", "A#")
    )]
    fn test_from_str_augmented_major_seventh(
        chord: &str,
        root: &str,
        third: &str,
        fifth: &str,
        seventh: &str,
    ) {
        let c = Chord::from_str(chord).unwrap();
        let r = Note::from_str(root).unwrap();
        let t = Note::from_str(third).unwrap();
        let f = Note::from_str(fifth).unwrap();
        let s = Note::from_str(seventh).unwrap();
        assert_eq!(c.notes, vec![r, t, f, s]);
        assert_eq!(c.chord_type, ChordType::AugmentedMajorSeventh);
    }

    #[rstest(
        chord,
        root,
        third,
        fifth,
        seventh,
        case("Cdim7", "C", "Eb", "Gb", "A"),
        case("C#dim7", "C#", "E", "G", "Bb"),
        case("Dbdim7", "Db", "E", "G", "Bb"),
        case("Ddim7", "D", "F", "Ab", "B"),
        case("D#dim7", "D#", "F#", "A", "C"),
        case("Ebdim7", "Eb", "Gb", "A", "C"),
        case("Edim7", "E", "G", "Bb", "Db"),
        case("Fdim7", "F", "Ab", "B", "D"),
        case("F#dim7", "F#", "A", "C", "Eb"),
        case("Gbdim7", "Gb", "A", "C", "Eb"),
        case("Gdim7", "G", "Bb", "Db", "E"),
        case("G#dim7", "G#", "B", "D", "F"),
        case("Abdim7", "Ab", "B", "D", "F"),
        case("Adim7", "A", "C", "Eb", "Gb"),
        case("A#dim7", "A#", "C#", "E", "G"),
        case("Bbdim7", "Bb", "Db", "E", "G"),
        case("Bdim7", "B", "D", "F", "Ab")
    )]
    fn test_from_str_diminished_seventh(
        chord: &str,
        root: &str,
        third: &str,
        fifth: &str,
        seventh: &str,
    ) {
        let c = Chord::from_str(chord).unwrap();
        let r = Note::from_str(root).unwrap();
        let t = Note::from_str(third).unwrap();
        let f = Note::from_str(fifth).unwrap();
        let s = Note::from_str(seventh).unwrap();
        assert_eq!(c.notes, vec![r, t, f, s]);
        assert_eq!(c.chord_type, ChordType::DiminishedSeventh);
    }

    #[rstest(
        chord,
        root,
        third,
        fifth,
        seventh,
        case("Cm7b5", "C", "Eb", "Gb", "Bb"),
        case("C#m7b5", "C#", "E", "G", "B"),
        case("Dbm7b5", "Db", "E", "G", "B"),
        case("Dm7b5", "D", "F", "Ab", "C"),
        case("D#m7b5", "D#", "F#", "A", "C#"),
        case("Ebm7b5", "Eb", "Gb", "A", "Db"),
        case("Em7b5", "E", "G", "Bb", "D"),
        case("Fm7b5", "F", "Ab", "B", "Eb"),
        case("F#m7b5", "F#", "A", "C", "E"),
        case("Gbm7b5", "Gb", "A", "C", "E"),
        case("Gm7b5", "G", "Bb", "Db", "F"),
        case("G#m7b5", "G#", "B", "D", "F#"),
        case("Abm7b5", "Ab", "B", "D", "Gb"),
        case("Am7b5", "A", "C", "Eb", "G"),
        case("A#m7b5", "A#", "C#", "E", "G#"),
        case("Bbm7b5", "Bb", "Db", "E", "Ab"),
        case("Bm7b5", "B", "D", "F", "A")
    )]
    fn test_from_str_half_diminished_seventh(
        chord: &str,
        root: &str,
        third: &str,
        fifth: &str,
        seventh: &str,
    ) {
        let c = Chord::from_str(chord).unwrap();
        let r = Note::from_str(root).unwrap();
        let t = Note::from_str(third).unwrap();
        let f = Note::from_str(fifth).unwrap();
        let s = Note::from_str(seventh).unwrap();
        assert_eq!(c.notes, vec![r, t, f, s]);
        assert_eq!(c.chord_type, ChordType::HalfDiminishedSeventh);
    }

    #[rstest(
        chord,
        note,
        contains,
        case("C", "C", true),
        case("C", "E", true),
        case("C", "D", false)
    )]
    fn test_contains(chord: &str, note: &str, contains: bool) {
        let c = Chord::from_str(chord).unwrap();
        let n = Note::from_str(note).unwrap();
        assert_eq!(c.contains(n), contains);
    }
}
