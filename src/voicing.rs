use std::{
    cmp::{max, min, Ordering},
    convert::{TryFrom, TryInto},
    fmt,
    slice::Iter,
};

use itertools::Itertools;

use crate::{
    Chord, Distance, Fingering, FretID, FretPattern, Note, PitchClass, Tuning, UkeString,
    FINGER_COUNT, STRING_COUNT,
};

/// A chord voicing.
///
/// The voicing of a chord describes the order of the individual notes within
/// the chord. The same chord can be voiced in different ways, i.e. there are
/// several ways to play the same chord on the ukulele.
/// https://en.wikipedia.org/wiki/Voicing_(music)
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Voicing {
    uke_strings: [UkeString; STRING_COUNT],
}

impl Voicing {
    // Create a Voicing instance from a set of frets and a tuning.
    // As there is no information about a certain chord for which
    // the voicing is created, the computed `note`s in the resulting
    // `UkeString`s will by default be sharp (for notes that can be sharp
    // or flat).
    pub fn new(fret_pattern: impl Into<FretPattern>, tuning: Tuning) -> Self {
        let fret_pattern = fret_pattern.into();

        let uke_strings: Vec<UkeString> = tuning
            .roots()
            .zip(fret_pattern.iter())
            .map(|(root, &fret)| (root, fret, root + fret))
            .collect();

        Self {
            uke_strings: uke_strings.try_into().unwrap(),
        }
    }

    pub fn uke_strings(&self) -> Iter<'_, UkeString> {
        self.uke_strings.iter()
    }

    pub fn roots(&self) -> impl Iterator<Item = Note> + '_ {
        self.uke_strings.iter().map(|(r, _f, _n)| *r)
    }

    pub fn frets(&self) -> impl Iterator<Item = FretID> + '_ {
        self.uke_strings.iter().map(|(_r, f, _n)| *f)
    }

    pub fn notes(&self) -> impl Iterator<Item = Note> + '_ {
        self.uke_strings.iter().map(|(_r, _f, n)| *n)
    }

    /// Return the overall number of strings pressed down when playing
    /// this voicing.
    pub fn count_pressed_strings(&self) -> usize {
        self.frets().filter(|&f| f > 0).count()
    }

    /// Return the lowest fret at which a string is pressed down.
    pub fn get_min_pressed_fret(&self) -> FretID {
        self.frets().filter(|&x| x > 0).min().unwrap_or_default()
    }

    /// Return the lowest fret involved in playing the chord voicing
    /// (is 0 if the chord is open).
    pub fn get_min_fret(&self) -> FretID {
        self.frets().min().unwrap()
    }

    pub fn get_max_fret(&self) -> FretID {
        self.frets().max().unwrap()
    }

    pub fn get_span(&self) -> FretID {
        let max_fret = self.get_max_fret();

        match max_fret {
            // Special case [0, 0, 0, 0]: span is zero.
            0 => 0,
            _ => max_fret - self.get_min_pressed_fret() + 1,
        }
    }

    /// Return `true` if the voicing contains all the notes needed
    /// to play the given `chord`.
    pub fn spells_out(&self, chord: &Chord) -> bool {
        self.notes()
            .sorted()
            .dedup()
            .eq(chord.played_notes().sorted().dedup())
    }

    pub fn get_chords(&self) -> Vec<Chord> {
        let mut chords = vec![];

        let mut pitches: Vec<PitchClass> = self.notes().map(|n| n.pitch_class).collect();
        pitches.sort();
        pitches.dedup();

        // Rotate pitch class list and collect all matching chords.
        // For example, try [C, DSharp, GSharp], [DSharp, GSharp, C], [GSharp, C, FSharp].
        for _ in 0..pitches.len() {
            if let Ok(chord) = Chord::try_from(&pitches[..]) {
                chords.push(chord);
            }

            pitches.rotate_left(1);
        }

        chords.sort();
        chords
    }

    /// Return `true` if the current voicing requires the player to play a barre chord.
    /// For this, I took some inspiration from
    /// https://github.com/hyvyys/chord-fingering/blob/master/src/barre.js
    pub fn has_barre(&self) -> bool {
        let min_fret = self.get_min_pressed_fret();
        let mut min_fret_count = 0;
        let mut pressed_frets = vec![];

        for fret in self.frets() {
            // Open strings may not appear below pressed strings.
            if fret == 0 {
                if min_fret_count > 0 {
                    return false;
                }
            } else {
                if fret == min_fret {
                    min_fret_count += 1;
                }
                pressed_frets.push(fret);
            }
        }

        // 0232 and 2323 should not be treated as having a barre.
        // Same with 1313 and 0141.
        for i in 1..4 {
            let alternating_frets = pressed_frets
                .iter()
                .zip([min_fret, min_fret + i].iter().cycle())
                .filter(|(f1, f2)| f1 == f2)
                .count();

            if alternating_frets == pressed_frets.len() {
                return false;
            }
        }

        // 0111 can be played with fingering 0123.
        if min_fret_count < STRING_COUNT && min_fret_count == pressed_frets.len() {
            return false;
        }

        min_fret_count >= 2
    }

    /// Compute a fingering for the current voicing, i.e. assign the player's
    /// fingers to the positions on the fretboard that have to be pressed down.
    /// The return value is an array of numbers representing the fingers
    /// on the strings (represented by the indexes of the array).
    /// This assumes that each chord voicing has a unique fingering (which is
    /// not true in reality - often several fingerings are possible). My fingering
    /// strategy here is based on my own way to play certain chords. For example,
    /// I tend to avoid barre chords if possible, e.g. I play the G major chord
    /// as 0132 and not as 0121.
    pub fn fingers_on_strings(&self) -> [u8; STRING_COUNT] {
        // Total number of strings on which we need to place our fingers.
        let pressed_strings = self.count_pressed_strings();

        // Determine the range of frets to be considered.
        let max_fret = self.get_max_fret();
        let min_fret = match pressed_strings {
            // 0000
            0 => 1,
            // e.g. 0007
            1 if max_fret > 3 => max_fret,
            // e.g. 0003
            1 => 1,
            // e.g. 2003
            2 if max_fret == 3 => 1,
            _ => self.get_min_pressed_fret(),
        };

        let mut fingering = [0; STRING_COUNT];

        // Current finger (can have values 1 to 4).
        let mut finger = 1;
        // Last finger that has been assigned to a string.
        let mut prev_finger = 1;
        // Number of strings with fingers on them up to this point.
        let mut used_strings = 0;

        for fret_id in min_fret..max_fret + 1 {
            for (i, f) in self.frets().enumerate() {
                if f == fret_id {
                    fingering[i] = finger as u8;
                    used_strings += 1;
                    if (!self.has_barre() || finger > 1) && finger < FINGER_COUNT {
                        finger += 1;
                    }
                }
            }

            let remaining_fingers = FINGER_COUNT - finger;
            let remaining_strings = pressed_strings - used_strings;

            // If no finger has been used on the current fret, prepare to use
            // the next finger on the next fret if there are enough fingers left.
            if finger == prev_finger && remaining_fingers >= remaining_strings {
                finger += 1;
            }

            prev_finger = finger;
        }

        fingering
    }

    /// Return the distance in semitones between this and another voicing.
    /// It's computed by simply summing up the distances between the frets that
    /// are pressed down on the same string when moving from one voicing to the other.
    /// Inspired by http://www.petecorey.com/blog/2018/07/30/voice-leading-with-elixir/
    pub fn semitone_distance(&self, other: Self) -> u8 {
        self.frets()
            .zip(other.frets())
            .map(|(f1, f2)| max(f1, f2) - min(f1, f2))
            .sum()
    }

    pub fn fingering_distance(&self, other: Self) -> u8 {
        let l_fingering = Fingering::from(*self);
        let r_fingering = Fingering::from(other);

        l_fingering.distance(r_fingering)
    }

    pub fn distance(&self, other: Self) -> Distance {
        let semitone_distance = self.semitone_distance(other);
        let fingering_distance = self.fingering_distance(other);

        Distance::new(semitone_distance, fingering_distance)
    }
}

impl PartialOrd for Voicing {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Voicing {
    fn cmp(&self, other: &Self) -> Ordering {
        // In order to iterate over frets in reversed order, we could implement
        // DoubleEndedIterator ... or use this hack.
        let frets1: Vec<FretID> = self.frets().collect();
        let frets2: Vec<FretID> = other.frets().collect();

        match self
            .get_min_pressed_fret()
            .cmp(&other.get_min_pressed_fret())
        {
            Ordering::Equal => frets1.iter().rev().cmp(frets2.iter().rev()),
            other => other,
        }
    }
}

/// The default implementation is used to easily create an "empty" start and end node
/// in the voicing graph.
impl Default for Voicing {
    fn default() -> Self {
        Self::new([0, 0, 0, 0], Tuning::C)
    }
}

impl fmt::Debug for Voicing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let roots = self.roots().map(|r| r.to_string()).collect::<Vec<String>>();
        let frets = self.frets().collect::<Vec<FretID>>();
        let notes = self.notes().map(|n| n.to_string()).collect::<Vec<String>>();

        f.debug_struct("Voicing")
            .field("roots", &roots)
            .field("frets", &frets)
            .field("notes", &notes)
            .finish()
    }
}

impl From<&[UkeString]> for Voicing {
    fn from(uke_strings: &[UkeString]) -> Self {
        Self {
            // Let's assume that all the Vecs coming in here have the correct size.
            uke_strings: uke_strings.try_into().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest(
        frets1, frets2,
        case([0, 0, 0, 0], [0, 0, 0, 1]),
        case([0, 3, 3, 3], [0, 0, 3, 6]),
        case([0, 0, 8, 6], [0, 7, 8, 6]),
    )]
    fn test_compare(frets1: [FretID; STRING_COUNT], frets2: [FretID; STRING_COUNT]) {
        let voicing1 = Voicing::new(frets1, Tuning::C);
        let voicing2 = Voicing::new(frets2, Tuning::C);
        assert!(voicing1 < voicing2);
    }

    #[rstest(
        frets, count,
        case([0, 0, 0, 0], 0),
        case([0, 0, 0, 3], 1),
        case([1, 1, 1, 1], 4),
        case([1, 2, 3, 4], 4),
    )]
    fn test_count_pressed_strings(frets: [FretID; STRING_COUNT], count: usize) {
        let voicing = Voicing::new(frets, Tuning::C);
        assert_eq!(voicing.count_pressed_strings(), count);
    }

    #[rstest(
        frets, min_pressed_fret, min_fret, max_fret, span,
        // Special case [0, 0, 0, 0]: no string is pressed down.
        case([0, 0, 0, 0], 0, 0, 0, 0),
        case([1, 1, 1, 1], 1, 1, 1, 1),
        case([2, 0, 1, 3], 1, 0, 3, 3),
        case([5, 5, 5, 6], 5, 5, 6, 2),
        case([3, 0, 0, 12], 3, 0, 12, 10),
    )]
    fn test_get_min_max_fret_and_span(
        frets: [FretID; STRING_COUNT],
        min_pressed_fret: FretID,
        min_fret: FretID,
        max_fret: FretID,
        span: u8,
    ) {
        let voicing = Voicing::new(frets, Tuning::C);
        assert_eq!(voicing.get_min_pressed_fret(), min_pressed_fret);
        assert_eq!(voicing.get_min_fret(), min_fret);
        assert_eq!(voicing.get_max_fret(), max_fret);
        assert_eq!(voicing.get_span(), span);
    }

    #[rstest(
        frets, chord, spells_out,
        case([0, 0, 0, 3], "C", true), // G C E C
        case([5, 4, 3, 3], "C", true), // C E G C
        case([0, 2, 0, 3], "C", false), // G D E C
        case([0, 0, 3, 0], "C", false), // G C G C
        case([0, 3, 3, 3], "Cm", true), // G Eb G C
        case([1, 1, 1, 4], "C#", true), // G# C# F C#
    )]
    fn test_spells_out(frets: [FretID; STRING_COUNT], chord: Chord, spells_out: bool) {
        let voicing = Voicing::new(frets, Tuning::C);
        assert_eq!(voicing.spells_out(&chord), spells_out);
    }

    #[rstest(
        frets, chord, tuning,
        case([0, 0, 0, 3], "C", Tuning::C),
        case([0, 0, 0, 3], "D", Tuning::D),
        case([2, 2, 2, 0], "D", Tuning::C),
    )]
    fn test_get_chords(frets: [FretID; STRING_COUNT], chord: Chord, tuning: Tuning) {
        let voicing = Voicing::new(frets, tuning);
        let chords = voicing.get_chords();
        assert_eq!(chords, vec![chord]);
    }

    #[rstest(
        frets,
        case([1, 2, 3, 4]),
    )]
    fn test_get_chords_fail(frets: [FretID; STRING_COUNT]) {
        let voicing = Voicing::new(frets, Tuning::C);
        assert!(voicing.get_chords().is_empty());
    }

    #[rstest(
        frets, has_barre,
        // No fingered strings.
        case([0, 0, 0, 0], false),
        // One fingered string.
        case([1, 0, 0, 0], false),
        case([0, 1, 0, 0], false),
        case([0, 0, 1, 0], false),
        case([0, 0, 0, 1], false),
        // Two fingered strings.
        case([1, 1, 0, 0], false),
        case([2, 1, 0, 0], false),
        case([1, 0, 1, 0], false),
        case([2, 0, 1, 0], false),
        case([1, 0, 0, 1], false),
        case([0, 1, 1, 0], false),
        case([0, 1, 0, 1], false),
        case([0, 0, 1, 1], false),
        // Three fingered strings without barre.
        case([1, 1, 1, 0], false),
        case([1, 1, 0, 1], false),
        case([1, 0, 1, 1], false),
        case([0, 1, 1, 1], false),
        case([0, 1, 2, 1], false),
        case([0, 1, 4, 1], false),
        case([0, 2, 1, 2], false),
        case([0, 3, 2, 1], false),
        // Three fingered strings with barre.
        case([0, 2, 1, 1], true),
        // Four fingered strings without barre.
        case([1, 2, 1, 2], false),
        case([1, 3, 1, 3], false),
        case([2, 4, 1, 3], false),
        case([3, 3, 3, 1], false),
        case([1, 2, 2, 2], false),
        // Four fingered strings with barre.
        case([1, 1, 1, 1], true),
        case([1, 1, 1, 2], true),
        case([1, 1, 1, 3], true),
        case([1, 1, 1, 4], true),
        case([1, 1, 2, 1], true),
        case([1, 2, 1, 1], true),
        case([1, 3, 3, 1], true),
        case([2, 1, 1, 1], true),
        case([3, 1, 1, 1], true),
        case([3, 2, 1, 1], true),
        case([3, 1, 2, 1], true),
        case([3, 1, 1, 3], true),
    )]
    fn test_has_barre(frets: [FretID; STRING_COUNT], has_barre: bool) {
        let voicing = Voicing::new(frets, Tuning::C);
        assert_eq!(voicing.has_barre(), has_barre);
    }

    #[rstest(
        frets, fingering,
        // No fingered strings.
        case([0, 0, 0, 0], [0, 0, 0, 0]),
        // One fingered string.
        case([2, 0, 0, 0], [2, 0, 0, 0]),
        case([0, 0, 0, 3], [0, 0, 0, 3]),
        case([0, 0, 0, 7], [0, 0, 0, 1]),
        case([9, 0, 0, 0], [1, 0, 0, 0]),
        case([0, 0, 0, 10], [0, 0, 0, 1]),
        // Two fingered strings.
        case([2, 0, 1, 0], [2, 0, 1, 0]),
        case([2, 0, 0, 3], [2, 0, 0, 3]),
        // Three fingered strings without barre.
        case([2, 2, 2, 0], [1, 2, 3, 0]),
        case([0, 2, 3, 2], [0, 1, 3, 2]),
        case([1, 0, 1, 3], [1, 0, 2, 4]),
        case([1, 1, 0, 4], [1, 2, 0, 4]),
        case([3, 0, 3, 1], [3, 0, 4, 1]),
        case([11, 0, 10, 12], [2, 0, 1, 3]),
        // Three fingered strings with barre.
        case([0, 4, 3, 3], [0, 2, 1, 1]),
        // Four fingered strings without barre.
        case([2, 3, 2, 3], [1, 3, 2, 4]),
        case([2, 3, 5, 3], [1, 2, 4, 3]),
        case([2, 4, 1, 3], [2, 4, 1, 3]),
        case([3, 3, 3, 1], [2, 3, 4, 1]),
        case([1, 4, 4, 4], [1, 2, 3, 4]),
        case([11, 12, 10, 12], [2, 3, 1, 4]),
        // Four fingered strings with barre.
        case([3, 3, 3, 3], [1, 1, 1, 1]),
        case([2, 2, 2, 3], [1, 1, 1, 2]),
        case([4, 2, 2, 2], [3, 1, 1, 1]),
        case([4, 2, 3, 2], [3, 1, 2, 1]),
        case([1, 1, 1, 4], [1, 1, 1, 4]),
        case([3, 2, 1, 1], [3, 2, 1, 1]),
        case([4, 3, 2, 2], [3, 2, 1, 1]),
        case([3, 1, 1, 3], [3, 1, 1, 4]),
        case([1, 3, 3, 1], [1, 3, 4, 1]),
        // Fingering across a span of five frets.
        case([0, 0, 1, 5], [0, 0, 1, 4]),
        case([3, 0, 1, 5], [3, 0, 1, 4]),
        case([3, 5, 1, 5], [2, 3, 1, 4]),
    )]
    fn test_fingers_on_strings(frets: [FretID; STRING_COUNT], fingering: [FretID; STRING_COUNT]) {
        let voicing = Voicing::new(frets, Tuning::C);
        assert_eq!(voicing.fingers_on_strings(), fingering);
    }

    #[rstest(
        frets1, frets2, dist,
        case([0, 0, 0, 0], [0, 0, 0, 0], 0),
        case([0, 0, 0, 3], [2, 0, 1, 3], 3),
        case([2, 0, 1, 3], [0, 0, 0, 3], 3),
        case([0, 0, 0, 3], [2, 0, 1, 0], 6),
        case([3, 2, 1, 1], [5, 4, 3, 3], 8),
        case([3, 2, 1, 1], [0, 0, 0, 3], 8),
    )]
    fn test_semitone_distance(
        frets1: [FretID; STRING_COUNT],
        frets2: [FretID; STRING_COUNT],
        dist: u8,
    ) {
        let voicing1 = Voicing::new(frets1, Tuning::C);
        let voicing2 = Voicing::new(frets2, Tuning::C);
        assert_eq!(voicing1.semitone_distance(voicing2), dist);
    }
}
