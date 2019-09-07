use structopt::StructOpt;
use ukebox::chord::Chord;
use ukebox::ukulele::Ukulele;

#[derive(StructOpt)]
struct Cmd {
    chord: Chord,
}

fn main() {
    let args = Cmd::from_args();
    let mut uke = Ukulele::new();
    uke.play(&args.chord, 0);

    println!("[{}]\n", args.chord);
    println!("{}", uke);
}
