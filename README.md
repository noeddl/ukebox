# ukebox

`ukebox` is a command-line tool that shows you how to play a given chord on a ukulele by printing a chord diagram in ASCII art.

## Installation

`ukebox` is intended to be a stand-alone command-line application but for the time being you need Rust (version >= 1.37.0) to build and run the program. Get the source code by cloning the repo from Github.

```
$ git clone https://github.com/noeddl/ukebox
```

Binaries for different platforms and installation via `cargo` will be provided in upcoming releases.

## Usage

```
USAGE:
    ukebox [OPTIONS] <chord>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --min-fret <min-fret>    Minimal fret (= minimal position) from which to play <chord> [default: 0]

ARGS:
    <chord>    Name of the chord to be shown
```

Currently, `ukebox` can handle the following types of chords:

* major chords, e.g. `C`, `D#`, `Eb`
* minor chords, e.g. `Cm`, `D#m`, `Ebm`
* dominant 7th chords, e.g. `C7`, `D#7`, `Eb7`
* minor 7th chords, e.g. `Cm7`, `D#m7`, `Ebm7`

More types of chords will be supported in future versions.

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
$ ukebox G -f 3
[G - G major]

A  -|-o-|---|---|---|- D
E  -|---|---|-o-|---|- B
C  -|---|---|-o-|---|- G
G  -|---|---|-o-|---|- D
      5
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
