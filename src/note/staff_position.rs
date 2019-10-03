use crate::note::StaffSteps;
use std::ops::Add;

const STAFF_POSITION_COUNT: StaffSteps = 7;

/// The vertical position of the notehead on the staff (on a line or in a space).
/// We use the staff position of an enharmonic note to decide whether it is sharp
/// or flat.
///
/// For example, a note with pitch class `FSharp` can either be written as `F#`
/// (if its staff position is `FPos` or `Gb` (if its staff position is `GPos`).
///
/// https://en.wikipedia.org/wiki/Staff_(music)#Staff_positions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StaffPosition {
    CPos,
    DPos,
    EPos,
    FPos,
    GPos,
    APos,
    BPos,
}

impl From<StaffSteps> for StaffPosition {
    fn from(n: StaffSteps) -> Self {
        use StaffPosition::*;

        // Make sure we get a value between 0 and 6.
        let v = n % STAFF_POSITION_COUNT;

        // There does not seem to be a good way to turn integers into enum
        // variants without using external crates. Hardcoding the mapping
        // is not so elegant but at least readable.
        match v {
            0 => CPos,
            1 => DPos,
            2 => EPos,
            3 => FPos,
            4 => GPos,
            5 => APos,
            6 => BPos,
            // Because of the modulo, `v` will always be in the correct range.
            _ => unreachable!(),
        }
    }
}

impl Add<StaffSteps> for StaffPosition {
    type Output = Self;

    /// Get the staff position that is `n` positions higher than the current one.
    fn add(self, n: StaffSteps) -> Self {
        Self::from(self as StaffSteps + n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest_parametrize;
    use StaffPosition::*;

    #[rstest_parametrize(
        n,
        staff_position,
        case(0, CPos),
        case(1, DPos),
        case(2, EPos),
        case(3, FPos),
        case(4, GPos),
        case(5, APos),
        case(6, BPos)
    )]
    fn test_from_int(n: StaffSteps, staff_position: StaffPosition) {
        assert_eq!(StaffPosition::from(n), staff_position);
    }

    #[rstest_parametrize(
        staff_position,
        n,
        result,
        case(CPos, 0, CPos),
        case(CPos, 1, DPos),
        case(DPos, 2, FPos),
        case(CPos, 7, CPos),
        case(CPos, 8, DPos),
        case(CPos, 14, CPos)
    )]
    fn test_pitch_class_add_int(
        staff_position: StaffPosition,
        n: StaffSteps,
        result: StaffPosition,
    ) {
        assert_eq!(staff_position + n, result);
    }
}
