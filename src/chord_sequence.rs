use std::slice::Iter;
use std::str::FromStr;

use crate::Chord;

#[derive(Debug, PartialEq)]
pub struct ChordSequence {
    chords: Vec<Chord>,
}

impl ChordSequence {
    pub fn chords(&self) -> Iter<'_, Chord> {
        self.chords.iter()
    }

    pub fn transpose(&self, semitones: i8) -> ChordSequence {
        let chords = self.chords().map(|c| c.transpose(semitones)).collect();
        Self { chords }
    }
}

impl FromStr for ChordSequence {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: Result<Vec<_>, _> = s.split_whitespace().map(|s| Chord::from_str(s)).collect();

        if let Ok(chords) = res {
            return Ok(Self { chords });
        }

        Err("Could not parse chord sequence")
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest(
        chord_seq,
        chords,
        case("", &[]),
        case("C", &["C"]),
        case("C F G", &["C", "F", "G"]),
        case("Dsus2 Am7 C#", &["Dsus2", "Am7", "C#"]),
    )]
    fn test_from_str(chord_seq: &str, chords: &[&str]) {
        let cs = ChordSequence::from_str(chord_seq).unwrap();
        let chords1: Vec<Chord> = cs.chords().cloned().collect();
        let chords2: Vec<Chord> = chords.iter().map(|c| Chord::from_str(c).unwrap()).collect();
        assert_eq!(chords1, chords2);
    }

    #[rstest(chord_seq, case("Z"), case("A Z"))]
    fn test_from_str_fail(chord_seq: &str) {
        assert!(ChordSequence::from_str(chord_seq).is_err())
    }

    #[rstest(
        chord_seq1,
        semitones,
        chord_seq2,
        case("", 0, ""),
        case("C F G", 0, "C F G"),
        case("C F G", 1, "C# F# G#"),
        case("C F G", -1, "B E Gb"),
        case("C F G", 12, "C F G"),
    )]
    fn test_transpose(chord_seq1: &str, semitones: i8, chord_seq2: &str) {
        let cs1 = ChordSequence::from_str(chord_seq1).unwrap();
        let cs2 = ChordSequence::from_str(chord_seq2).unwrap();
        assert_eq!(cs1.transpose(semitones), cs2);
    }
}
