use std::slice::Iter;
use std::str::FromStr;

use crate::Chord;

pub struct ChordSequence {
    chords: Vec<Chord>,
}

impl ChordSequence {
    pub fn chords(&self) -> Iter<'_, Chord> {
        self.chords.iter()
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
}
