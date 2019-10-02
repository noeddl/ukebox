use structopt::StructOpt;
use ukebox::chord::Chord;
use ukebox::Frets;

#[derive(StructOpt)]
struct Cmd {
    #[structopt(short = "f", long, default_value = "0")]
    /// Minimal fret (= minimal position) from which to play <chord>
    min_fret: Frets,
    /// Name of the chord to be shown
    chord: Chord,
}

fn main() {
    let args = Cmd::from_args();
    let diagram = args.chord.get_diagram(args.min_fret);
    println!("{}", diagram);
}
