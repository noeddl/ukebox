use crate::note::Interval;
use crate::note::Semitones;
use std::fmt;
use std::str::FromStr;

/// Custom error for strings that cannot be parsed into tunings.
#[derive(Debug)]
pub struct ParseTuningError {
    name: String,
}

impl fmt::Display for ParseTuningError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse tuning name \"{}\"", self.name)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Tuning {
    C,
    D,
    G,
}

impl Tuning {
    pub fn get_semitones(&self) -> Semitones {
        match self {
            Self::C => 0,
            Self::D => 2,
            Self::G => 7,
        }
    }

    pub fn get_interval(&self) -> Interval {
        match self {
            Self::C => Interval::PerfectUnison,
            Self::D => Interval::MajorSecond,
            Self::G => Interval::PerfectFifth,
        }
    }
}

impl FromStr for Tuning {
    type Err = ParseTuningError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tuning = match s {
            "C" => Tuning::C,
            "D" => Tuning::D,
            "G" => Tuning::G,
            _ => return Err(ParseTuningError { name: s.to_string() }),
        };

        Ok(tuning)
    }
}
