use structopt::StructOpt;
use ukebox::{Chord, FretID, FretPattern, Tuning};

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
        /// Minimal fret (= minimal position) from which to play <chord>
        #[structopt(short = "f", long, default_value = "0")]
        min_fret: FretID,
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
            min_fret,
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
            let voicings = chord.get_voicings(min_fret, tuning);

            println!("{}", format!("[{}]\n", chord));
            println!("{}", voicings[0]);
        }
        Subcommand::Name { fret_pattern } => {
            let chords = fret_pattern.get_chords(tuning);

            if chords.is_empty() {
                println!("No matching chord was found");
            }

            for chord in chords {
                println!("{}", chord);
            }
        }
    }
}
