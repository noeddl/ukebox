use structopt::StructOpt;
use ukebox::chord::Chord;
use ukebox::chord::FretID;
use ukebox::chord::Tuning;

#[derive(StructOpt)]
struct Cmd {
    #[structopt(short = "f", long, default_value = "0")]
    /// Minimal fret (= minimal position) from which to play <chord>
    min_fret: FretID,
    /// Type of tuning to be used
    #[structopt(short, long, default_value = "C", possible_values = &Tuning::variants())]
    tuning: Tuning,
    /// Name of the chord to be shown
    chord: Chord,
}

fn main() {
    let args = Cmd::from_args();
    let diagram = args.chord.get_diagram(args.min_fret, args.tuning);
    println!("{}", diagram);
}
