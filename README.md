# ukebox

[![Crates.io](https://img.shields.io/crates/v/ukebox)](https://crates.io/crates/ukebox)
[![travis](https://travis-ci.com/noeddl/ukebox.svg?branch=master)](https://travis-ci.com/noeddl/ukebox)
[![license](https://img.shields.io/crates/l/ukebox)](#license)
[![rustc](https://img.shields.io/badge/rustc-1.37+-lightgray.svg)](https://blog.rust-lang.org/2019/08/15/Rust-1.37.0.html)

`ukebox` is a command-line tool that shows you how to play a given chord on a ukulele by printing a chord diagram in ASCII art.

## Installation

`ukebox` is intended to be a stand-alone command-line application but for the time being you need Rust (version >= 1.37.0) to build and run the program. The easiest way is to use `cargo`.

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
    ukebox [OPTIONS] <chord>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --min-fret <min-fret>    Minimal fret (= minimal position) from which to play <chord> [default: 0]
    -t, --tuning <tuning>        Type of tuning to be used [default: C]  [possible values: C, D, G]

ARGS:
    <chord>    Name of the chord to be shown
```

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

## Examples

When running the program with Rust, replace the command `ukebox` with `cargo run --`, e.g. `cargo run -- G`.

```
$ ukebox G
[G - G major]

A  ||---|-o-|---|---|- B
E  ||---|---|-o-|---|- G
C  ||---|-o-|---|---|- D
G o||---|---|---|---|- G
```

```
$ ukebox G --min-fret 3
[G - G major]

A  -|-o-|---|---|---|- D
E  -|---|---|-o-|---|- B
C  -|---|---|-o-|---|- G
G  -|---|---|-o-|---|- D
      5
```

```
$ ukebox G --tuning D
[G - G major]

B  o||---|---|---|---|- B
F#  ||-o-|---|---|---|- G
D  o||---|---|---|---|- D
A   ||---|-o-|---|---|- B
```

```
$ ukebox G --tuning D --min-fret 3
[G - G major]

B   -|-o-|---|---|---|- D
F#  -|---|---|-o-|---|- B
D   -|---|---|-o-|---|- G
A   -|---|---|-o-|---|- D
       3
```

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
