use crate::{FingerPosition, Voicing, FINGER_COUNT};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Fingering {
    finger_positions: [FingerPosition; FINGER_COUNT],
}

impl From<[FingerPosition; FINGER_COUNT]> for Fingering {
    fn from(finger_positions: [FingerPosition; FINGER_COUNT]) -> Self {
        Self { finger_positions }
    }
}

impl From<Voicing> for Fingering {
    fn from(voicing: Voicing) -> Self {
        let mut finger_positions = [(0, 0); FINGER_COUNT];

        let fingers_on_strings = voicing.get_fingering();

        for ((i, fret_id), finger) in voicing.frets().enumerate().zip(&fingers_on_strings) {
            if finger > &0 {
                let index = (finger - 1) as usize;
                let string_id = (i + 1) as u8;

                // For a barre chord, only keep track of the position of the upmost finger.
                if finger_positions[index] == (0, 0) {
                    finger_positions[index] = (string_id, fret_id);
                }
            }
        }

        Self { finger_positions }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{FretID, Tuning, STRING_COUNT};

    use super::*;

    #[rstest(
        frets, finger_positions,
        case([0, 0, 0, 0], [(0, 0), (0, 0), (0, 0), (0, 0)]),
        case([2, 2, 2, 0], [(1, 2), (2, 2), (3, 2), (0, 0)]),
        case([2, 2, 2, 2], [(1, 2), (0, 0), (0, 0), (0, 0)]),
    )]
    fn test_from_voicing(
        frets: [FretID; STRING_COUNT],
        finger_positions: [FingerPosition; FINGER_COUNT],
    ) {
        let voicing = Voicing::new(frets, Tuning::C);
        let fingering1 = Fingering::from(voicing);
        let fingering2 = Fingering::from(finger_positions);
        assert_eq!(fingering1, fingering2);
    }
}
