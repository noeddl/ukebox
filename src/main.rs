use structopt::StructOpt;
use ukebox::chord::Chord;
use ukebox::chord::FretID;
use ukebox::chord::Tuning;
use ukebox::diagram::FretPattern;

#[derive(StructOpt)]
enum Ukebox {
    /// Chord chart lookup
    Chart {
        /// Minimal fret (= minimal position) from which to play <chord>
        #[structopt(short = "f", long, default_value = "0")]
        min_fret: FretID,
        /// Type of tuning to be used
        #[structopt(short, long, default_value = "C", possible_values = &Tuning::variants())]
        tuning: Tuning,
        /// Name of the chord to be shown
        chord: Chord,
    },
    /// Chord name lookup
    Name {
        /// Type of tuning to be used
        #[structopt(short, long, default_value = "C", possible_values = &Tuning::variants())]
        tuning: Tuning,
        /// A compact chart representing the finger positions of the chord to be looked up
        fret_pattern: FretPattern,
    }
}

fn main() {
    match Ukebox::from_args() {
        Ukebox::Chart { min_fret, tuning, chord } => {
            let diagram = chord.get_diagram(min_fret, tuning);
            println!("{}", diagram);
        },
        Ukebox::Name { tuning, fret_pattern } => {
            let chords = fret_pattern.get_chords(tuning);
            for chord in chords {
                println!("{}", chord);
            }
        }
    }
}
