# ukebox

[![Crates.io](https://img.shields.io/crates/v/ukebox)](https://crates.io/crates/ukebox)
[![Documentation](https://docs.rs/ukebox/badge.svg)](https://docs.rs/ukebox)
[![Continuous integration](https://github.com/noeddl/ukebox/actions/workflows/ci.yml/badge.svg)](https://github.com/noeddl/ukebox/actions/workflows/ci.yml)
[![license](https://img.shields.io/crates/l/ukebox)](#license)
[![rustc](https://img.shields.io/badge/rustc-1.74+-lightgray.svg)](https://blog.rust-lang.org/2023/12/07/Rust-1.74.1.html)

`ukebox` is a ukulele chord toolbox for the command line written in Rust.

## Table of contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [Chord chart lookup](#chord-chart-lookup)
  - [Chord name lookup](#chord-name-lookup)
  - [Voice leading](#voice-leading)
- [Supported chord types](#supported-chord-types)
- [Development](#development)
  - [Pre-commit hooks](#pre-commit-hooks)
- [License](#license)
- [Contribution](#contribution)

## Features

* shows you how to play a given chord on a ukulele by printing a **chord chart** in ASCII art
* presents the **chord name(s)** corresponding to a chord fingering given in [numeric chord notation](https://ukenut.com/compact-fretted-chord-notation/)
* supports **different ukulele tunings** (C, D and G)
* can present each chord in **different positions** along the fretbord
* allows you to **transpose** a chord by any number of semitones
* helps you find a good **voice leading** for a given chord sequence

## Installation

Archives of precompiled binaries for each [release](https://github.com/noeddl/ukebox/releases) of `ukebox` are available for Windows, macOS and Linux.

Alternatively, `ukebox` can be installed with `cargo`.

```
$ cargo install ukebox
```

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
    chords        List all supported chord types and symbols
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

Run `ukebox chords` to get a list of the chord types and symbols currently supported.

```
$ ukebox chords
Supported chord types and symbols

The root note C is used as an example.

C major - C, Cmaj, CM
C major 7th - Cmaj7, CM7
C major 9th - Cmaj9, CM9
C major 11th - Cmaj11, CM11
C major 13th - Cmaj13, CM13
C major 6th - C6, Cmaj6, CM6
C 6th/9th - C6/9, Cmaj6/9, CM6/9
C dominant 7th - C7, Cdom
C dominant 9th - C9
C dominant 11th - C11
C dominant 13th - C13
C dominant 7th flat 9th - C7b9
C dominant 7th sharp 9th - C7#9
C dominant 7th flat 5th - C7b5, C7dim5
C suspended 4th - Csus4, Csus
C suspended 2nd - Csus2
C dominant 7th suspended 4th - C7sus4, C7sus
C dominant 7th suspended 2nd - C7sus2
C minor - Cm, Cmin
C minor 7th - Cm7, Cmin7
C minor/major 7th - CmMaj7, CmM7, CminMaj7
C minor 6th - Cm6, Cmin6
C minor 9th - Cm9, Cmin9
C minor 11th - Cm11, Cmin11
C minor 13th - Cm13, Cmin13
C diminished - Cdim, Co
C diminished 7th - Cdim7, Co7
C half-diminished 7th - Cm7b5, Cø, Cø7
C 5th - C5
C augmented - Caug, C+
C augmented 7th - Caug7, C+7, C7#5
C augmented major 7th - CaugMaj7, C+M7
C added 9th - Cadd9, Cadd2
C added 4th - Cadd4
```

## Development

### Pre-commit hooks

To automatically enforce coding conventions, Git hooks can simplify the process. Pre-commit hooks run before each commit, checking conditions or modifying files (e.g., using rustfmt to format code). The .githooks folder contains a pre-commit script that runs the Rust linter clippy and formats the code with rustfmt. You need to install these tools and configure Git to use the hooks in .githooks.

```
$ rustup component add clippy
$ rustup component add rustfmt
$ git config core.hooksPath .githooks
```

Now, the hooks will run every time you commit. If a problem is found, the commit process is aborted with a message. Resolve clippy warnings manually, add changed files with git add, and rerun git commit until no errors remain.

To ignore clippy lints for specific blocks, use #[allow(LINT_NAME)], or globally with #![allow(LINT_NAME)] at the top of lib.rs. Prevent rustfmt from formatting a block by adding #[rustfmt::skip].

To bypass pre-commit hooks for a commit, add the -n option to git commit.

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
