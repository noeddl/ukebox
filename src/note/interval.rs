use crate::note::Semitones;
use crate::note::StaffSteps;
use std::fmt;
use std::str::FromStr;

/// Custom error for strings that cannot be parsed into intervals.
#[derive(Debug)]
pub struct ParseIntervalError {
    name: String,
}

impl fmt::Display for ParseIntervalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse interval name \"{}\"", self.name)
    }
}

/// An interval is the difference between two notes.
/// https://en.wikipedia.org/wiki/Interval_(music)
#[derive(Debug, Clone, Copy)]
pub enum Interval {
    PerfectUnison,
    MajorSecond,
    MinorThird,
    MajorThird,
    PerfectFifth,
    MinorSeventh,
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
            PerfectFifth => 7,
            MinorSeventh => 10,
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
            PerfectFifth => 5,
            MinorSeventh => 7,
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
            "P5" => PerfectFifth,
            "m3" => MinorThird,
            "M3" => MajorThird,
            "m7" => MinorSeventh,
            _ => return Err(ParseIntervalError { name }),
        };

        Ok(interval)
    }
}
