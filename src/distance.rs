use std::iter::Sum;
use std::ops::Add;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// The distance between two voicings combining semitone distance
/// fingering distance as a tuple.
pub struct Distance(u8, u8);

impl Distance {
    pub fn new(semitone_distance: u8, fingering_distance: u8) -> Self {
        Self(semitone_distance, fingering_distance)
    }

    pub fn semitone_distance(&self) -> u8 {
        self.0
    }
}

impl Add for Distance {
    type Output = Self;

    fn add(self, other: Distance) -> Self {
        Distance(self.0 + other.0, self.1 + other.1)
    }
}

impl<'a> Sum<&'a Self> for Distance {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self(0, 0), |a, b| Self(a.0 + b.0, a.1 + b.1))
    }
}
