use std::convert::TryInto;
use std::str::FromStr;

use structopt::clap::arg_enum;

use crate::{Interval, Note, Semitones, STRING_COUNT};

// Using clap's `arg_enum` macro allows the specification of all Tuning
// variants as `possible_values` for the CLI `--tuning` option.
arg_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Tuning {
        C,
        D,
        G,
    }
}

impl Tuning {
    pub fn get_semitones(self) -> Semitones {
        match self {
            Self::C => 0,
            Self::D => 2,
            Self::G => 7,
        }
    }

    pub fn get_interval(self) -> Interval {
        match self {
            Self::C => Interval::PerfectUnison,
            Self::D => Interval::MajorSecond,
            Self::G => Interval::PerfectFifth,
        }
    }

    pub fn get_roots(self) -> [Note; STRING_COUNT] {
        let interval = self.get_interval();

        ["G", "C", "E", "A"]
            .iter()
            .map(|c| Note::from_str(c).unwrap() + interval)
            .collect::<Vec<Note>>()
            .try_into()
            .unwrap()
    }
}
