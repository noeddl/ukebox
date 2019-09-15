use structopt::StructOpt;
use ukebox::chord::Chord;
use ukebox::ukulele::Ukulele;
use ukebox::Frets;

#[derive(StructOpt)]
struct Cmd {
    #[structopt(short = "f", long, default_value = "0")]
    min_fret: Frets,

    chord: Chord,
}

fn main() {
    let args = Cmd::from_args();
    let mut uke = Ukulele::new();
    uke.play(&args.chord, args.min_fret);

    println!("[{}]\n", args.chord);
    println!("{}", uke);
}
