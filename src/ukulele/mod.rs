#![allow(clippy::module_inception)]
mod streng;
mod ukulele;

pub use self::streng::Streng;
pub use self::ukulele::Ukulele;

use crate::Frets;

/// Number of frets shown on the fretboard chart.
pub const CHART_WIDTH: Frets = 4;
