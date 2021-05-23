# ukebox

[![Crates.io](https://img.shields.io/crates/v/ukebox)](https://crates.io/crates/ukebox)
[![Documentation](https://docs.rs/ukebox/badge.svg)](https://docs.rs/ukebox)
[![travis](https://travis-ci.com/noeddl/ukebox.svg?branch=master)](https://travis-ci.com/noeddl/ukebox)
[![license](https://img.shields.io/crates/l/ukebox)](#license)
[![rustc](https://img.shields.io/badge/rustc-1.48+-lightgray.svg)](https://blog.rust-lang.org/2020/11/19/Rust-1.48.html)

`ukebox` is a ukulele chord toolbox for the command line written in Rust.

## Features

* shows you how to play a given chord on a ukulele by printing a **chord chart** in ASCII art
* presents the **chord name(s)** corresponding to a chord fingering given in [numeric chord notation](https://ukenut.com/compact-fretted-chord-notation/)
* supports **different ukulele tunings** (C, D and G)
* can present each chord in **different positions** along the fretbord
* allows you to **transpose** a chord by any number of semitones
* helps you find a good **voice leading** for a given chord sequence

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
    ukebox [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --tuning <TUNING>    Type of tuning to be used [default: C]  [possible values: C, D, G]

SUBCOMMANDS:
    chart         Chord chart lookup
    help          Prints this message or the help of the given subcommand(s)
    name          Chord name lookup
    voice-lead    Voice leading for a sequence of chords
```

When running the program with Rust, replace the command `ukebox` with `cargo run --release`, e.g. `cargo run --release -- chart G`.

### Chord chart lookup

Use the subcommand `chart` to look up the chart for a given chord name. By default, the first matching chord voicing is presented. Use the flag `--all` to get all possible voicings of the same chord. You can use additional options to further filter the result, e.g. by specifying a minimal or a maximal fret that should be involved in the chord voicing.

```
USAGE:
    ukebox chart [FLAGS] [OPTIONS] <chord>

FLAGS:
    -a, --all        Print out all voicings of <chord> that fulfill the given conditions
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --max-fret <FRET_ID>       Maximal fret up to which to play <chord> [default: 12]
        --max-span <FRET_COUNT>    Maximal span between the first and the last fret pressed down when playing <chord>
                                   [default: 4]
        --min-fret <FRET_ID>       Minimal fret (= minimal position) from which to play <chord> [default: 0]
        --transpose <SEMITONES>    Number of semitones to add (e.g. 1, +1) or to subtract (e.g. -1) [default: 0]
    -t, --tuning <TUNING>          Type of tuning to be used [default: C]  [possible values: C, D, G]

ARGS:
    <CHORD>    Name of the chord to be shown
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

A  -|---|-o-|---|---|- D
E  -|---|---|---|-o-|- B
C  -|---|---|---|-o-|- G
G  -|-o-|---|---|---|- B
      4
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

```
$ ukebox chart --transpose 1 C
[C# - C# major]

A  ||---|---|---|-o-|- C#
E  ||-o-|---|---|---|- F
C  ||-o-|---|---|---|- C#
G  ||-o-|---|---|---|- G#
```

```
$ ukebox chart --transpose -2 C
[Bb - Bb major]

A  ||-o-|---|---|---|- Bb
E  ||-o-|---|---|---|- F
C  ||---|-o-|---|---|- D
G  ||---|---|-o-|---|- Bb
```

```
$ ukebox chart --all --max-fret 5 C
[C - C major]

A  ||---|---|-o-|---|- C
E o||---|---|---|---|- E
C o||---|---|---|---|- C
G o||---|---|---|---|- G

A  ||---|---|-o-|---|- C
E o||---|---|---|---|- E
C  ||---|---|---|-o-|- E
G o||---|---|---|---|- G

A  ||---|---|-o-|---|- C
E  ||---|---|-o-|---|- G
C  ||---|---|---|-o-|- E
G o||---|---|---|---|- G

A  -|-o-|---|---|---|- C
E  -|-o-|---|---|---|- G
C  -|---|-o-|---|---|- E
G  -|---|---|-o-|---|- C
      3
```

### Chord name lookup

Use the subcommand `name` to look up the chord name(s) corresponding to a given chord fingering.

```
USAGE:
    ukebox name [OPTIONS] <FRET_PATTERN>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --tuning <TUNING>    Type of tuning to be used [default: C]  [possible values: C, D, G]

ARGS:
    <FRET_PATTERN>    A compact chart representing the finger positions of the chord to be looked up
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

### Voice leading

Use the subcommand `voice-lead` to get some inspiration for finding a good [voice leading](https://en.wikipedia.org/wiki/Voice_leading) for a given sequence of chords. In order to decide that one voice leading may better than the other, `ukebox` uses both the "semitone distance" between two voicings (to find good sounding transitions between voicings) as well as the distance between the fingerings to be used to play them (to make sure the transitions are also comfortably playable). This feature is still very experimental and will hopefully be improved some more in the future. For its implementation, I took a lot of inspiration from [these](http://www.petecorey.com/blog/2018/07/30/voice-leading-with-elixir/) [blog](http://www.petecorey.com/blog/2018/08/13/algorithmically-fingering-guitar-chords-with-elixir/) [articles](http://www.petecorey.com/blog/2018/08/27/computing-fingering-distance-with-dr-levenshtein/) by Pete Corey.

```
USAGE:
    ukebox voice-lead [OPTIONS] <CHORD_SEQUENCE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --max-fret <FRET_ID>       Maximal fret up to which to play <chord> [default: 12]
        --max-span <FRET_COUNT>    Maximal span between the first and the last fret pressed down when playing <chord>
                                   [default: 4]
        --min-fret <FRET_ID>       Minimal fret (= minimal position) from which to play <chord> [default: 0]
        --transpose <SEMITONES>    Number of semitones to add (e.g. 1, +1) or to subtract (e.g. -1) [default: 0]
    -t, --tuning <TUNING>          Type of tuning to be used [default: C]  [possible values: C, D, G]

ARGS:
    <CHORD_SEQUENCE>    Chord sequence
```

Some examples:

```
$ ukebox voice-lead "C F G"
[C - C major]

A  ||---|---|-3-|---|- C
E o||---|---|---|---|- E
C o||---|---|---|---|- C
G o||---|---|---|---|- G

[F - F major]

A  ||---|---|-3-|---|- C
E  ||-1-|---|---|---|- F
C o||---|---|---|---|- C
G  ||---|-2-|---|---|- A

[G - G major]

A  ||---|-2-|---|---|- B
E  ||---|---|-3-|---|- G
C  ||---|-1-|---|---|- D
G o||---|---|---|---|- G
```

```
$ ukebox voice-lead "C F G" --tuning D
[C - C major]

B   ||-1-|---|---|---|- C
F#  ||-1-|---|---|---|- G
D   ||---|-2-|---|---|- E
A   ||---|---|-3-|---|- C

[F - F major]

B   ||-1-|---|---|---|- C
F#  ||---|---|-4-|---|- A
D   ||---|---|-3-|---|- F
A   ||---|---|-2-|---|- C

[G - G major]

B  o||---|---|---|---|- B
F#  ||-1-|---|---|---|- G
D  o||---|---|---|---|- D
A   ||---|-2-|---|---|- B
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
