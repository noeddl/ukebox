use crate::note::Semitones;
use crate::STRING_COUNT;
use std::cmp::Ordering;
use std::fmt;
use std::ops::Add;
use std::ops::Sub;
use std::str::FromStr;

/// Custom error for strings that cannot be parsed into fret IDs.
#[derive(Debug)]
pub struct ParseFretIDError {
    name: String,
}

impl fmt::Display for ParseFretIDError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse fret ID \"{}\"", self.name)
    }
}

/// Type for the number of frets (corresponds to the number of semitones)
/// to move from one note or pitch class to another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FretID(u8);

impl fmt::Display for FretID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for FretID {
    type Err = ParseFretIDError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id: u8 = match s.parse() {
            Ok(id) => id,
            Err(_) => {
                return Err(ParseFretIDError {
                    name: s.to_string(),
                })
            }
        };

        Ok(Self(id))
    }
}

impl From<u8> for FretID {
    fn from(id: u8) -> Self {
        Self(id)
    }
}

impl PartialEq<u8> for FretID {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u8> for FretID {
    fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl Add for FretID {
    type Output = Self;

    fn add(self, other: FretID) -> Self {
        Self::from(self.0 + other.0)
    }
}

impl Add<Semitones> for FretID {
    type Output = Self;

    fn add(self, other: Semitones) -> Self {
        Self::from(self.0 + other)
    }
}

impl Sub<Semitones> for FretID {
    type Output = Self;

    fn sub(self, other: Semitones) -> Self {
        Self::from(self.0 - other)
    }
}

impl FretID {
    pub fn to_semitones(self) -> Semitones {
        self.0 as Semitones
    }
}

//#[derive(Debug, Clone, Copy)]
pub type FretPattern = [FretID; STRING_COUNT];
