use structopt::StructOpt;
use ukebox::chord::Chord;
use ukebox::chord::FretID;
use ukebox::chord::Tuning;
use ukebox::diagram::FretPattern;

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
        Subcommand::Chart { min_fret, chord } => {
            let diagram = chord.get_diagram(min_fret, tuning);
            println!("{}", diagram);
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
