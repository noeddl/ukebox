use structopt::StructOpt;
use ukebox::chord::Chord;
use ukebox::chord::FretID;
use ukebox::chord::Tuning;
use ukebox::diagram::FretPattern;
use std::error::Error;
use std::str::FromStr;

#[derive(StructOpt)]
struct Cmd {
    #[structopt(short = "f", long, default_value = "0")]
    /// Minimal fret (= minimal position) from which to play <chord>
    min_fret: FretID,
    /// Type of tuning to be used
    #[structopt(short, long, default_value = "C", possible_values = &Tuning::variants())]
    tuning: Tuning,
    /// Reverse chord lookup (e.g. retrieve the chord name "C" when writing "-r 0003")
    #[structopt(short, long)]
    reverse: bool,
    /// Name of the chord to be shown
    chord: String,
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Cmd::from_args();

    if args.reverse {
        let fret_pattern = FretPattern::from_str(&args.chord)?;
        let chord = fret_pattern.get_chord(args.tuning)?;
        println!("{}", chord);
    } else {
        let chord = Chord::from_str(&args.chord)?;
        let diagram = chord.get_diagram(args.min_fret, args.tuning);
        println!("{}", diagram);
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: Invalid value for \'<chord>\': {}", err);
        std::process::exit(1);
    }
}
