use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::fmt;
use std::process::Command; // Run programs
use ukebox::Frets;

struct TestConfig {
    start_index: usize,
    /// Distance to the previous chord shape.
    shape_dist: Frets,
    frets: [Frets; 4],
    note_indices: [usize; 4],
    base_fret: Frets,
    min_fret: Frets,
    lower_min_fret: Frets,
}

impl TestConfig {
    fn new(
        start_index: usize,
        shape_dist: Frets,
        frets: [Frets; 4],
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

        Self::new(self.start_index, self.shape_dist, frets, note_indices)
    }

    fn generate_diagram(&self, chord: &str, notes: &[&str]) -> String {
        let mut diagram = format!("[{} - {} major]\n\n", chord, chord);
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

    fn generate_tests_for_chord(&self, i: usize, note_names: &[&str]) -> (String, Vec<Test>) {
        let mut tests = Vec::new();
        let root = note_names.iter().cycle().nth(self.start_index + i).unwrap();

        let notes: Vec<&str> = self
            .note_indices
            .iter()
            .map(|j| *note_names.iter().cycle().nth(*j).unwrap())
            .collect();

        for j in self.lower_min_fret..self.min_fret + 1 {
            let diagram = self.generate_diagram(root, &notes);
            let test = Test {
                chord: root.to_string(),
                min_fret: j,
                diagram,
            };
            tests.push(test);
        }

        (root.to_string(), tests)
    }

    fn generate_tests(&mut self) -> Vec<Test> {
        let note_names = [
            "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
        ];
        let alt_names = [
            "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B",
        ];

        let mut tests = Vec::new();

        // Move upwards the fretboard using the given chord shape.
        for i in 0..12 {
            let (root, subtests) = self.generate_tests_for_chord(i, &note_names);
            tests.extend(subtests);

            if root.ends_with("#") {
                let (_root, subtests) = self.generate_tests_for_chord(i, &alt_names);
                tests.extend(subtests);
            }

            *self = self.next();
        }

        tests
    }
}

struct Test {
    chord: String,
    min_fret: Frets,
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
fn test_major_chords() -> Result<(), Box<dyn std::error::Error>> {
    let test_configs = vec![
        TestConfig::new(0, 1, [0, 0, 0, 3], [7, 0, 4, 0]),
        TestConfig::new(9, 2, [2, 1, 0, 0], [9, 1, 4, 9]),
        TestConfig::new(7, 1, [0, 2, 3, 2], [7, 2, 7, 11]),
        TestConfig::new(5, 1, [2, 0, 1, 0], [9, 0, 5, 9]),
        TestConfig::new(2, 2, [2, 2, 2, 0], [9, 2, 6, 9]),
    ];

    run_tests(test_configs)
}
