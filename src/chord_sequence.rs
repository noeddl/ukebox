use std::slice::Iter;
use std::str::FromStr;

use crate::Chord;

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Debug, thiserror::Error)]
#[error("could not parse chord sequence")]
pub struct ParseChordSequenceError;

impl FromStr for ChordSequence {
    type Err = ParseChordSequenceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: Result<Vec<_>, _> = s.split_whitespace().map(Chord::from_str).collect();

        if let Ok(chords) = res {
            return Ok(Self { chords });
        }

        Err(ParseChordSequenceError)
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
    fn test_from_str(chord_seq: ChordSequence, chords: &[&str]) {
        let chords1: Vec<Chord> = chord_seq.chords().cloned().collect();
        let chords2: Vec<Chord> = chords.iter().map(|c| Chord::from_str(c).unwrap()).collect();
        assert_eq!(chords1, chords2);
    }

    #[rstest(chord_seq, case("Z"), case("A Z"))]
    fn test_from_str_fail(chord_seq: &str) {
        assert!(ChordSequence::from_str(chord_seq).is_err());
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
    fn test_transpose(chord_seq1: ChordSequence, semitones: i8, chord_seq2: ChordSequence) {
        assert_eq!(chord_seq1.transpose(semitones), chord_seq2);
    }
}
