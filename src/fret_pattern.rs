use std::convert::TryInto;
use std::slice::Iter;
use std::str::FromStr;

use crate::{FretID, STRING_COUNT};

/// Custom error for strings that cannot be parsed into a fret pattern.
#[derive(Debug, thiserror::Error)]
#[error("fret pattern has wrong format (should be something like 1234 or '7 8 9 10')")]
pub struct ParseFretPatternError;

/// A pattern of frets to press down for playing a chord.
/// Each index of the array corresponds to a ukulele string.
#[derive(Debug, Copy, Clone)]
pub struct FretPattern {
    frets: [FretID; STRING_COUNT],
}

impl FretPattern {
    pub fn iter(&self) -> Iter<'_, FretID> {
        self.frets.iter()
    }
}

impl From<[FretID; STRING_COUNT]> for FretPattern {
    fn from(frets: [FretID; STRING_COUNT]) -> Self {
        Self { frets }
    }
}

impl FromStr for FretPattern {
    type Err = ParseFretPatternError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Handle both patterns containing spaces such as "1 2 3 4" as well as patterns
        // without spaces such as "1234".
        let split: Vec<String> = match s.contains(' ') {
            true => s.split(' ').map(|c| c.to_string()).collect(),
            false => s.chars().map(|c| c.to_string()).collect(),
        };

        // Parse out numbers in the pattern.
        let fret_res: Result<Vec<FretID>, _> = split.iter().map(|s| s.parse()).collect();

        if let Ok(fret_vec) = fret_res {
            // Check for the correct number of frets.
            let res: Result<[FretID; STRING_COUNT], _> = fret_vec.try_into();
            if let Ok(frets) = res {
                return Ok(Self::from(frets));
            }
        }

        Err(ParseFretPatternError)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest(
        fret_pattern, frets,
        case("2220", [2, 2, 2, 0]),
        case("2 2 2 0", [2, 2, 2, 0]),
        case("7 8 9 10", [7, 8, 9, 10]),
    )]
    fn test_from_str(fret_pattern: FretPattern, frets: [FretID; STRING_COUNT]) {
        assert_eq!(fret_pattern.frets, frets);
    }

    #[rstest(s, case(""), case("Cm"), case("222"), case("22201"))]
    fn test_from_str_fail(s: &str) {
        assert!(FretPattern::from_str(s).is_err());
    }
}
