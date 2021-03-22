use std::cmp::max;

use structopt::StructOpt;
use ukebox::{Chord, ChordChart, FretID, FretPattern, Semitones, Tuning, Voicing};

#[derive(StructOpt)]
struct Ukebox {
    /// Type of tuning to be used
    #[structopt(short, long, global = true, default_value = "C", possible_values = &Tuning::variants())]
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
        #[structopt(short = "f", long, default_value = "0")]
        min_fret: FretID,
        /// Maximal fret up to which to play <chord>
        #[structopt(long, default_value = "15")]
        max_fret: FretID,
        /// Maximal span between the first and the last fret pressed down when playing <chord>
        #[structopt(long, default_value = "4")]
        max_span: Semitones,
        /// Number of semitones to add (e.g. 1, +1) or to subtract (e.g. -1)
        #[structopt(long, allow_hyphen_values = true, default_value = "0")]
        transpose: i8,
        /// Name of the chord to be shown
        chord: Chord,
    },
    /// Chord name lookup
    Name {
        /// A compact chart representing the finger positions of the chord to be looked up
        fret_pattern: FretPattern,
    },
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

            println!("{}", format!("[{}]\n", chord));

            let voicings = chord.voicings(tuning).filter(|v| {
                v.get_min_fret() >= min_fret
                    && v.get_max_fret() <= max_fret
                    && v.get_span() < max_span
            });

            for voicing in voicings {
                let width = max(max_span, 4);
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
