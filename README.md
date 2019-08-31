# ukebox

A Ukulele chord finder in Rust.

## Usage

```
cargo run -- [CHORD]
```

ukebox shows you how to play `CHORD` on a ukulele by printing a fretboard in ASCII art, e.g. for `cargo run -- G` the output looks like this:

```
[G]

A  ||---+-●-+---+---+ B
E  ||---+---+-●-+---+ G
C  ||---+-●-+---+---+ D
G ○||---+---+---+---+ G
```
