use std::cmp::max;

use structopt::StructOpt;
use ukebox::{Chord, ChordChart, FretID, FretPattern, Semitones, Tuning, Voicing};

/// Maximal possible fret ID.
/// According to Wikipedia, the biggest ukulele type (baritone) has 21 frets.
const MAX_FRET_ID: FretID = 21;

/// Maximal span of frets.
/// Playing a chord that spans more than 5 frets seems anatomically impossible to me.
const MAX_SPAN: Semitones = 5;

/// Minimal number of frets to be shown in a chord chart.
const MIN_CHART_WIDTH: Semitones = 4;

#[derive(StructOpt)]
struct Ukebox {
    /// Type of tuning to be used
    #[structopt(short, long, global = true, value_name = "TUNING", default_value = "C", possible_values = &Tuning::variants())]
    tuning: Tuning,
    #[structopt(subcommand)]
    cmd: Subcommand,
}

#[derive(StructOpt)]
enum Subcommand {
    /// Chord chart lookup
    Chart {
        /// Print out all voicings of <chord> that fulfill the given conditions
        #[structopt(short, long)]
        all: bool,
        /// Minimal fret (= minimal position) from which to play <chord>
        #[structopt(long, value_name = "FRET_ID", default_value = "0", validator = validate_fret_id)]
        min_fret: FretID,
        /// Maximal fret up to which to play <chord>
        #[structopt(long, value_name = "FRET_ID", default_value = "12", validator = validate_fret_id)]
        max_fret: FretID,
        /// Maximal span between the first and the last fret pressed down when playing <chord>
        #[structopt(long, value_name = "FRET_COUNT", default_value = "4", validator = validate_span)]
        max_span: Semitones,
        /// Number of semitones to add (e.g. 1, +1) or to subtract (e.g. -1)
        #[structopt(
            long,
            value_name = "SEMITONES",
            allow_hyphen_values = true,
            default_value = "0"
        )]
        transpose: i8,
        /// Name of the chord to be shown
        #[structopt(value_name = "CHORD")]
        chord: Chord,
    },
    /// Chord name lookup
    Name {
        /// A compact chart representing the finger positions of the chord to be looked up
        #[structopt(value_name = "FRET_PATTERN")]
        fret_pattern: FretPattern,
    },
}

fn validate_fret_id(s: String) -> Result<(), String> {
    if let Ok(fret) = s.parse::<FretID>() {
        if fret <= MAX_FRET_ID {
            return Ok(());
        }
    }

    Err(String::from("must be a number between 0 and 21"))
}

fn validate_span(s: String) -> Result<(), String> {
    if let Ok(span) = s.parse::<Semitones>() {
        if span <= MAX_SPAN {
            return Ok(());
        }
    }

    Err(String::from("must be a number between 0 and 5"))
}

fn main() {
    let args = Ukebox::from_args();
    let tuning = args.tuning;

    match args.cmd {
        Subcommand::Chart {
            all,
            min_fret,
            max_fret,
            max_span,
            transpose,
            chord,
        } => {
            // Transpose chord.
            let chord = match transpose {
                // Subtract semitones (e.g. -1).
                t if t < 0 => chord - transpose.abs() as u8,
                // Add semitones (e.g. 1, +1).
                _ => chord + transpose as u8,
            };

            let mut voicings = chord
                .voicings(tuning)
                .filter(|v| {
                    v.get_min_fret() >= min_fret
                        && v.get_max_fret() <= max_fret
                        && v.get_span() < max_span
                })
                .peekable();

            if voicings.peek().is_none() {
                println!("No matching chord voicing was found");
            } else {
                println!("{}", format!("[{}]\n", chord));
            }

            for voicing in voicings {
                let width = max(max_span, MIN_CHART_WIDTH);
                let chart = ChordChart::new(voicing, width);
                println!("{}", chart);

                if !all {
                    break;
                }
            }
        }
        Subcommand::Name { fret_pattern } => {
            let voicing = Voicing::new(fret_pattern, tuning);
            let chords = voicing.get_chords();

            if chords.is_empty() {
                println!("No matching chord was found");
            }

            for chord in chords {
                println!("{}", chord);
            }
        }
    }
}
