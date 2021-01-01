# ukebox

[![Crates.io](https://img.shields.io/crates/v/ukebox)](https://crates.io/crates/ukebox)
[![Documentation](https://docs.rs/ukebox/badge.svg)](https://docs.rs/ukebox)
[![travis](https://travis-ci.com/noeddl/ukebox.svg?branch=master)](https://travis-ci.com/noeddl/ukebox)
[![license](https://img.shields.io/crates/l/ukebox)](#license)
[![rustc](https://img.shields.io/badge/rustc-1.48+-lightgray.svg)](https://blog.rust-lang.org/2020/11/19/Rust-1.48.html)

`ukebox` is a ukulele chord finder for the command line written in Rust.

## Features

* shows you how to play a given chord on a ukulele by printing a **chord chart** in ASCII art
* presents the **chord name(s)** corresponding to a chord fingering given in [numeric chord notation](https://ukenut.com/compact-fretted-chord-notation/)
* supports **different ukulele tunings** (C, D and G)
* can present **different fingerings** of the same chord along the fretbord

## Installation

`ukebox` is intended to be a stand-alone command-line application but for the time being you need [Rust](https://www.rust-lang.org/) to build and run the program. The easiest way to install `ukebox` is to use `cargo`.

```
$ cargo install ukebox
```

Alternatively, get the source code by cloning the repo from Github.

```
$ git clone https://github.com/noeddl/ukebox
```

Downloadable binaries for different platforms will be provided in upcoming releases.

## Usage

```
USAGE:
    ukebox <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    chart    Chord chart lookup
    help     Prints this message or the help of the given subcommand(s)
    name     Chord name lookup
```

When running the program with Rust, replace the command `ukebox` with `cargo run --release`, e.g. `cargo run --release chart G`.

### Chord chart lookup

Use the subcommand `chart` to look up the chart for a given chord name.

```
USAGE:
    ukebox chart [OPTIONS] <chord>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --min-fret <min-fret>    Minimal fret (= minimal position) from which to play <chord> [default: 0]
    -t, --tuning <tuning>        Type of tuning to be used [default: C]  [possible values: C, D, G]

ARGS:
    <chord>    Name of the chord to be shown
```

Some examples:

```
$ ukebox chart G
[G - G major]

A  ||---|-o-|---|---|- B
E  ||---|---|-o-|---|- G
C  ||---|-o-|---|---|- D
G o||---|---|---|---|- G
```

```
$ ukebox chart --tuning D G
[G - G major]

B  o||---|---|---|---|- B
F#  ||-o-|---|---|---|- G
D  o||---|---|---|---|- D
A   ||---|-o-|---|---|- B
```

```
$ ukebox chart --min-fret 3 G
[G - G major]

A  -|-o-|---|---|---|- D
E  -|---|---|-o-|---|- B
C  -|---|---|-o-|---|- G
G  -|---|---|-o-|---|- D
      5
```

```
$ ukebox chart --tuning D --min-fret 3 G
[G - G major]

B   -|-o-|---|---|---|- D
F#  -|---|---|-o-|---|- B
D   -|---|---|-o-|---|- G
A   -|---|---|-o-|---|- D
       3
```

### Chord name lookup

Use the subcommand `name` to look up the chord name(s) corresponding to a given chord fingering.

```
USAGE:
    ukebox name [OPTIONS] <fret-pattern>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --tuning <tuning>    Type of tuning to be used [default: C]  [possible values: C, D, G]

ARGS:
    <fret-pattern>    A compact chart representing the finger positions of the chord to be looked up
```

Some examples:

```
$ ukebox name 2220
D - D major
```

```
$ ukebox name --tuning D 2220
E - E major
```

```
$ ukebox name 0233
Csus2 - C suspended 2nd
Gsus4 - G suspended 4th
```

If the fret pattern contains fret numbers greater than 9 you have to add spaces between the fret numbers and embed them in quotes:

```
$ ukebox name "7 7 7 10"
G - G major
```

## Supported chord types

Currently, `ukebox` can handle the following types of chords:

* major chords, e.g. `C`, `D#`, `Eb`
* minor chords, e.g. `Cm`, `D#m`, `Ebm`
* suspended second chords, e.g. `Csus2`, `D#sus2`, `Ebsus2`
* suspended fourth chords, e.g. `Csus4`, `D#sus4`, `Ebsus4`
* augmented triads, e.g. `Caug`, `D#aug`, `Ebaug`
* diminished triads, e.g. `Cdim`, `D#dim`, `Ebdim`
* dominant 7th chords, e.g. `C7`, `D#7`, `Eb7`
* minor 7th chords, e.g. `Cm7`, `D#m7`, `Ebm7`
* major 7th chords, e.g. `Cmaj7`, `D#maj7`, `Ebmaj7`
* minor/major 7th chords, e.g. `CmMaj7`, `D#mMaj7`, `EbmMaj7`
* augmented 7th chords, e.g. `Caug7`, `D#aug7`, `Ebaug7`
* augmented major 7th chords, e.g. `CaugMaj7`, `D#augMaj7`, `EbaugMaj7`
* diminished 7th chords, e.g. `Cdim7`, `D#dim7`, `Ebdim7`
* half-diminished 7th chords, e.g. `Cm7b5`, `D#m7b5`, `Ebm7b5`

More types of chords will be supported in future versions (see [#35](https://github.com/noeddl/ukebox/issues/35)).

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
