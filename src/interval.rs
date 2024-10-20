use std::str::FromStr;

use crate::{Semitones, StaffSteps};

/// Custom error for strings that cannot be parsed into intervals.
#[derive(Debug, thiserror::Error)]
#[error("could not parse interval name '{name}'")]
pub struct ParseIntervalError {
    name: String,
}

/// An interval is the difference between two notes.
///
/// https://en.wikipedia.org/wiki/Interval_(music)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interval {
    PerfectUnison,
    MajorSecond,
    MinorThird,
    MajorThird,
    PerfectFourth,
    DiminishedFifth,
    PerfectFifth,
    AugmentedFifth,
    MajorSixth,
    DiminishedSeventh,
    MinorSeventh,
    MajorSeventh,
    MinorNinth,
    MajorNinth,
    AugmentedNinth,
    PerfectEleventh,
    MajorThirteenth,
}

impl Interval {
    /// Return the number of semitones that the interval encompasses.
    pub fn to_semitones(self) -> Semitones {
        use Interval::*;

        match self {
            PerfectUnison => 0,
            MajorSecond => 2,
            MinorThird => 3,
            MajorThird => 4,
            PerfectFourth => 5,
            DiminishedFifth => 6,
            PerfectFifth => 7,
            AugmentedFifth => 8,
            MajorSixth => 9,
            DiminishedSeventh => 9,
            MinorSeventh => 10,
            MajorSeventh => 11,
            MinorNinth => 13,
            MajorNinth => 14,
            AugmentedNinth => 15,
            PerfectEleventh => 17,
            MajorThirteenth => 21,
        }
    }

    /// Return the interval's number. It corresponds to the number of
    /// staff positions that the intervall encompasses.
    pub fn to_number(self) -> StaffSteps {
        use Interval::*;

        match self {
            PerfectUnison => 1,
            MajorSecond => 2,
            MinorThird => 3,
            MajorThird => 3,
            PerfectFourth => 4,
            DiminishedFifth => 5,
            PerfectFifth => 5,
            AugmentedFifth => 5,
            MajorSixth => 6,
            DiminishedSeventh => 7,
            MinorSeventh => 7,
            MajorSeventh => 7,
            MinorNinth => 9,
            MajorNinth => 9,
            AugmentedNinth => 9,
            PerfectEleventh => 11,
            MajorThirteenth => 13,
        }
    }
}

impl FromStr for Interval {
    type Err = ParseIntervalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Interval::*;

        let name = s.to_string();

        let interval = match s {
            "P1" => PerfectUnison,
            "M2" => MajorSecond,
            "m3" => MinorThird,
            "M3" => MajorThird,
            "P4" => PerfectFourth,
            "d5" => DiminishedFifth,
            "P5" => PerfectFifth,
            "A5" => AugmentedFifth,
            "M6" => MajorSixth,
            "d7" => DiminishedSeventh,
            "m7" => MinorSeventh,
            "M7" => MajorSeventh,
            "m9" => MinorNinth,
            "M9" => MajorNinth,
            "A9" => AugmentedNinth,
            "P11" => PerfectEleventh,
            "M13" => MajorThirteenth,
            _ => return Err(ParseIntervalError { name }),
        };

        Ok(interval)
    }
}
