#![allow(clippy::suspicious_arithmetic_impl)]
use crate::note::Semitones;
use std::ops::Add;
use std::ops::Sub;

/// Number of pitch classes.
const PITCH_CLASS_COUNT: Semitones = 12;

/// A pitch class is "a set of all pitches that are a whole number of octaves
/// apart, e.g., the pitch class C consists of the Cs in all octaves."
/// https://en.wikipedia.org/wiki/Pitch_class
///
/// Our 12 pitch classes are represented with integers from 0 to 11.
/// Values > 11 will be used to model retrieval of the same pitch class in a
/// higher octave.
/// For example, pitch class 12 is the same as pitch class 0 and corresponds
/// to the pitch class of C.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PitchClass {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}

impl From<Semitones> for PitchClass {
    /// Convert an integer into a pitch class.
    ///
    /// To model the fact that e.g. all instances of the note `C` in different
    /// octaves belong to the same pitch class, each integer is placed in the
    /// range of potential pitch classes (between 0 and 11).
    /// For example, 12, 24, 36, etc. all correspond to pitch class 0.
    fn from(n: Semitones) -> Self {
        use PitchClass::*;

        // Make sure we get a value between 0 and 11.
        let v = n % PITCH_CLASS_COUNT;

        // There does not seem to be a good way to turn integers into enum
        // variants without using external crates. Hardcoding the mapping
        // is not so elegant but at least readable.
        match v {
            0 => C,
            1 => CSharp,
            2 => D,
            3 => DSharp,
            4 => E,
            5 => F,
            6 => FSharp,
            7 => G,
            8 => GSharp,
            9 => A,
            10 => ASharp,
            11 => B,
            // Because of the modulo, `v` will always be in the correct range.
            _ => unreachable!(),
        }
    }
}

impl Add<Semitones> for PitchClass {
    type Output = Self;

    /// Get the pitch class that is `n` semitones higher than the current
    /// pitch class.
    fn add(self, n: Semitones) -> Self {
        let v = self as Semitones + n;
        Self::from(v)
    }
}

impl Sub for PitchClass {
    type Output = Semitones;

    /// Get the difference between two pitch classes in number of frets
    /// or semitones.
    ///
    /// `self` is assumed to always be higher as `other` with a difference
    /// of at most one octave.
    ///
    /// Examples:
    /// * D - C: both pitch classes are assumed to be in the same octave, D being
    ///          higher than C. The difference is 2.
    /// * D - A: D is higher than A, the difference is 5.
    fn sub(self, other: Self) -> Semitones {
        let d = self as i8 - other as i8;

        let diff = match d {
            d if d >= 0 => d,
            _ => d + PITCH_CLASS_COUNT as i8,
        };

        diff as Semitones
    }
}

impl Sub<Semitones> for PitchClass {
    type Output = Self;

    /// Get the pitch class that is `n` semitones lower than the current
    /// pitch class.
    fn sub(self, n: Semitones) -> Self {
        Self::from(self - Self::from(n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use PitchClass::*;

    #[rstest(
        n,
        pitch_class,
        case(0, C),
        case(1, CSharp),
        case(2, D),
        case(3, DSharp),
        case(4, E),
        case(5, F),
        case(6, FSharp),
        case(7, G),
        case(8, GSharp),
        case(9, A),
        case(10, ASharp),
        case(11, B),
        case(12, C),
        case(13, CSharp),
        case(24, C),
        case(127, G),
        case(255, DSharp)
    )]
    fn test_from_int(n: Semitones, pitch_class: PitchClass) {
        assert_eq!(PitchClass::from(n), pitch_class);
    }

    #[rstest(
        pitch_class,
        n,
        result,
        case(C, 0, C),
        case(C, 1, CSharp),
        case(C, 10, ASharp),
        case(C, 12, C),
        case(C, 13, CSharp),
        case(C, 24, C)
    )]
    fn test_add_int(pitch_class: PitchClass, n: Semitones, result: PitchClass) {
        assert_eq!(pitch_class + n, result);
    }

    #[rstest(
        pc1,
        pc2,
        n,
        case(C, C, 0),
        case(D, C, 2),
        case(D, A, 5),
        case(C, CSharp, 11)
    )]
    fn test_sub_self(pc1: PitchClass, pc2: PitchClass, n: Semitones) {
        assert_eq!(pc1 - pc2, n);
    }

    #[rstest(
        pc1,
        n,
        pc2,
        case(C, 0, C),
        case(D, 2, C),
        case(D, 5, A),
        case(C, 11, CSharp),
        case(C, 12, C),
        case(C, 13, B)
    )]
    fn test_sub_int(pc1: PitchClass, n: Semitones, pc2: PitchClass) {
        assert_eq!(pc1 - n, pc2);
    }
}
