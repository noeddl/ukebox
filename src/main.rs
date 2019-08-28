use structopt::StructOpt;
use ukebox::ukulele::Ukulele;

#[derive(StructOpt)]
struct Cmd {
    chord: String,
}

fn main() {
    let args = Cmd::from_args();
    let mut uke = Ukulele::new();
    uke.play(&args.chord, 0);

    println!("[{}]\n", args.chord);
    println!("{}", uke);
}
