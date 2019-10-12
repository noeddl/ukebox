/// Integration tests to make sure all possible combinations of user input
/// result in the correct output.
/// This is done by generating all combinations of command line arguments
/// and options together with their expected output. The real program is called
/// using the command line parameters and the actual output is compared to the
/// expected output.
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::fmt;
use std::process::Command; // Run programs
use ukebox::chord::ChordType;
use ukebox::chord::FretID;
use ukebox::note::Semitones;

/// A set of parameters to generate tests for all chords produced
/// by moving a specific chord shape along the fretboard.
struct TestConfig {
    chord_type: ChordType,
    /// Start index in the note vector (= root of the chord shape).
    start_index: usize,
    /// Distance to the previous chord shape.
    shape_dist: Semitones,
    frets: [FretID; 4],
    note_indices: [usize; 4],
    base_fret: FretID,
    min_fret: FretID,
    lower_min_fret: FretID,
}

impl TestConfig {
    fn new(
        chord_type: ChordType,
        start_index: usize,
        shape_dist: Semitones,
        frets: [FretID; 4],
        note_indices: [usize; 4],
    ) -> Self {
        let min_fret = *frets.iter().min().unwrap();
        let max_fret = *frets.iter().max().unwrap();

        let base_fret = match max_fret {
            max_fret if max_fret <= 4 => 1,
            _ => min_fret,
        };

        let lower_min_fret = match min_fret {
            fret if fret < shape_dist => 0,
            _ => min_fret - shape_dist,
        };

        Self {
            chord_type,
            start_index,
            shape_dist,
            frets,
            note_indices,
            base_fret,
            min_fret,
            lower_min_fret,
        }
    }

    /// Move all frets and notes one fret/semitone higher.
    fn next(&mut self) -> Self {
        let mut frets = self.frets;
        for f in frets.iter_mut() {
            *f += 1;
        }

        let mut note_indices = self.note_indices;
        for n in note_indices.iter_mut() {
            *n += 1;
        }

        Self::new(
            self.chord_type,
            self.start_index,
            self.shape_dist,
            frets,
            note_indices,
        )
    }

    fn generate_diagram(&self, title: &str, notes: &[&str]) -> String {
        let mut diagram = title.to_string();
        let roots = ["G", "C", "E", "A"];

        // Show a symbol for the nut if the chord is played on the lower
        // end of the fretboard. Indicate ongoing strings otherwise.
        let nut = match self.base_fret {
            1 => "||",
            _ => "-|",
        };

        for i in (0..4).rev() {
            let root = roots[i];
            let fret = self.frets[i];
            let note = notes[i];

            // Mark open strings with a special symbol.
            let sym = match fret {
                0 => "o",
                _ => " ",
            };

            // Create a line representing the string with the fret to be pressed.
            let mut string = "".to_owned();

            for i in self.base_fret..self.base_fret + 4 {
                let c = match fret {
                    fret if fret == i => "o",
                    _ => "-",
                };

                string.push_str(&format!("-{}-|", c));
            }

            let line = format!("{} {}{}{}- {}", root, sym, nut, string, note);
            diagram.push_str(&format!("{}\n", line));
        }

        // If the fretboard section shown does not include the nut,
        // indicate the number of the first fret shown.
        if self.base_fret > 1 {
            diagram.push_str(&format!("      {}\n", self.base_fret))
        }

        diagram
    }

    fn generate_tests_for_chord(&self, index: usize, note_names: &[&str]) -> (String, Vec<Test>) {
        let mut tests = Vec::new();
        let root = *note_names.iter().cycle().nth(index).unwrap();

        let notes: Vec<&str> = self
            .note_indices
            .iter()
            .map(|j| *note_names.iter().cycle().nth(*j).unwrap())
            .collect();

        for j in self.lower_min_fret..self.min_fret + 1 {
            let suffix = match self.chord_type {
                ChordType::Major => "",
                ChordType::Minor => "m",
                ChordType::DominantSeventh => "7",
                ChordType::MinorSeventh => "m7",
            };
            let chord = format!("{}{}", root, suffix);
            let title = format!("[{} - {} {}]\n\n", chord, root, self.chord_type);
            let diagram = self.generate_diagram(&title, &notes);
            let test = Test {
                chord,
                min_fret: j,
                diagram,
            };
            tests.push(test);
        }

        (root.to_string(), tests)
    }

    fn generate_tests(&mut self) -> Vec<Test> {
        use ChordType::*;

        let note_names = [
            "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
        ];
        let alt_names = [
            "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B",
        ];

        let mut tests = Vec::new();

        // Move upwards the fretboard using the given chord shape.
        for i in 0..13 {
            let index = self.start_index + i;
            let names = match (index % 12, self.chord_type) {
                // Bm has F#.
                (11, Minor) => note_names,
                // All other minor chords have flat notes.
                (_, Minor) => alt_names,
                // C7, F7.
                (0, DominantSeventh) => alt_names,
                (5, DominantSeventh) => alt_names,
                // Cm7, Fm7, Gm7.
                (0, MinorSeventh) => alt_names,
                (5, MinorSeventh) => alt_names,
                (7, MinorSeventh) => alt_names,
                (_, _) => note_names,
            };

            let (root, subtests) = self.generate_tests_for_chord(index, &names);
            tests.extend(subtests);

            if root.ends_with("#") {
                let (_root, subtests) = self.generate_tests_for_chord(index, &alt_names);
                tests.extend(subtests);
            }

            *self = self.next();
        }

        tests
    }
}

/// A set of command line arguments and options together with the
/// expected output (chord diagram) to be shown.
struct Test {
    chord: String,
    min_fret: FretID,
    diagram: String,
}

impl fmt::Display for Test {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!(
            "cargo run -- {} -f {}\n\n{}\n",
            self.chord, self.min_fret, self.diagram
        );

        write!(f, "{}", s)
    }
}

fn run_tests(test_configs: Vec<TestConfig>) -> Result<(), Box<dyn std::error::Error>> {
    for mut test_config in test_configs {
        for test in test_config.generate_tests() {
            // Run `cargo test -- --nocapture` to print all tests run.
            println!("{}", test);

            let mut cmd = Command::main_binary()?;
            cmd.arg(test.chord);

            if test.min_fret > 0 {
                cmd.arg("-f").arg(test.min_fret.to_string());
            }

            cmd.assert()
                .success()
                .stdout(predicate::str::contains(test.diagram));
        }
    }

    Ok(())
}

#[test]
fn test_no_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::main_binary()?;
    cmd.assert().failure().stderr(predicate::str::contains(
        "error: The following required arguments were not provided",
    ));

    Ok(())
}

#[test]
fn test_unknown_chord() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::main_binary()?;
    cmd.arg("blafoo");
    cmd.assert().failure().stderr(predicate::str::contains(
        "error: Invalid value for '<chord>': Could not parse chord name \"blafoo\"",
    ));

    Ok(())
}

#[test]
fn test_major_chords() -> Result<(), Box<dyn std::error::Error>> {
    let cq = ChordType::Major;

    let test_configs = vec![
        TestConfig::new(cq, 0, 1, [0, 0, 0, 3], [7, 0, 4, 0]),
        TestConfig::new(cq, 9, 2, [2, 1, 0, 0], [9, 1, 4, 9]),
        TestConfig::new(cq, 7, 1, [0, 2, 3, 2], [7, 2, 7, 11]),
        TestConfig::new(cq, 5, 1, [2, 0, 1, 0], [9, 0, 5, 9]),
        TestConfig::new(cq, 2, 2, [2, 2, 2, 0], [9, 2, 6, 9]),
    ];

    run_tests(test_configs)
}

#[test]
fn test_minor_chords() -> Result<(), Box<dyn std::error::Error>> {
    let cq = ChordType::Minor;

    let test_configs = vec![
        TestConfig::new(cq, 0, 1, [0, 3, 3, 3], [7, 3, 7, 0]),
        TestConfig::new(cq, 9, 2, [2, 0, 0, 0], [9, 0, 4, 9]),
        TestConfig::new(cq, 7, 1, [0, 2, 3, 1], [7, 2, 7, 10]),
        TestConfig::new(cq, 5, 1, [1, 0, 1, 3], [8, 0, 5, 0]),
        TestConfig::new(cq, 2, 2, [2, 2, 1, 0], [9, 2, 5, 9]),
    ];

    run_tests(test_configs)
}

#[test]
fn test_dominant_seventh_chords() -> Result<(), Box<dyn std::error::Error>> {
    let cq = ChordType::DominantSeventh;

    let test_configs = vec![
        TestConfig::new(cq, 0, 1, [0, 0, 0, 1], [7, 0, 4, 10]),
        TestConfig::new(cq, 9, 2, [0, 1, 0, 0], [7, 1, 4, 9]),
        TestConfig::new(cq, 7, 1, [0, 2, 1, 2], [7, 2, 5, 11]),
        TestConfig::new(cq, 4, 1, [1, 2, 0, 2], [8, 2, 4, 11]),
        TestConfig::new(cq, 2, 1, [2, 0, 2, 0], [9, 0, 6, 9]),
    ];

    run_tests(test_configs)
}

#[test]
fn test_minor_seventh_chords() -> Result<(), Box<dyn std::error::Error>> {
    let cq = ChordType::MinorSeventh;

    let test_configs = vec![
        TestConfig::new(cq, 1, 0, [1, 1, 0, 2], [8, 1, 4, 11]),
        TestConfig::new(cq, 9, 2, [0, 0, 0, 0], [7, 0, 4, 9]),
        TestConfig::new(cq, 7, 1, [0, 2, 1, 1], [7, 2, 5, 10]),
        TestConfig::new(cq, 4, 1, [0, 2, 0, 2], [7, 2, 4, 11]),
        TestConfig::new(cq, 2, 1, [2, 0, 1, 0], [9, 0, 5, 9]),
    ];

    run_tests(test_configs)
}
