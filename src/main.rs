use std::str::FromStr;
use structopt::StructOpt;
use ukebox::chord::Chord;
use ukebox::ukulele::Ukulele;

#[derive(StructOpt)]
struct Cmd {
    chord: String,
}

fn main() {
    let args = Cmd::from_args();
    let mut uke = Ukulele::new();
    let chord = Chord::from_str(&args.chord).unwrap();
    uke.play(&chord, 0);

    println!("[{}]\n", chord);
    println!("{}", uke);
}
