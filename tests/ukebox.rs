use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::fmt;
use std::process::Command; // Run programs
use ukebox::Frets;

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

fn generate_diagram(chord: &str, base_fret: Frets, frets: &[Frets], notes: &[&str]) -> String {
    let mut diagram = format!("[{} - {} major]\n\n", chord, chord);
    let roots = ["G", "C", "E", "A"];

    // Show a symbol for the nut if the chord is played on the lower
    // end of the fretboard. Indicate ongoing strings otherwise.
    let nut = match base_fret {
        1 => "||",
        _ => "-|",
    };

    for i in (0..4).rev() {
        let root = roots[i];
        let fret = frets[i];
        let note = notes[i];

        // Mark open strings with a special symbol.
        let sym = match fret {
            0 => "o",
            _ => " ",
        };

        // Create a line representing the string with the fret to be pressed.
        let mut string = "".to_owned();

        for i in base_fret..base_fret + 4 {
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
    if base_fret > 1 {
        diagram.push_str(&format!("      {}\n", base_fret))
    }

    diagram
}

fn generate_tests() -> Vec<Test> {
    let note_names = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];
    let alt_names = [
        "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B",
    ];
    let mut frets = vec![0, 0, 0, 3];
    let mut note_indices = vec![7, 0, 4, 0];

    let mut tests = Vec::new();

    // Move upwards the fretboard using the given chord shape.
    for i in 0..12 {
        let min_fret = *frets.iter().min().unwrap();
        let base_fret = match min_fret {
            fret if fret < 2 => 1,
            _ => min_fret,
        };

        let root = note_names[i];

        let notes: Vec<&str> = note_indices
            .iter()
            .map(|j| *note_names.iter().cycle().nth(*j).unwrap())
            .collect();

        let diagram = generate_diagram(root, base_fret, &frets, &notes);
        let test = Test {
            chord: root.to_string(),
            min_fret,
            diagram,
        };
        tests.push(test);

        if root.ends_with("#") {
            let root = alt_names[i];

            let notes: Vec<&str> = note_indices
                .iter()
                .map(|j| *alt_names.iter().cycle().nth(*j).unwrap())
                .collect();

            let diagram = generate_diagram(root, base_fret, &frets, &notes);
            let test = Test {
                chord: root.to_string(),
                min_fret,
                diagram,
            };
            tests.push(test);
        }

        // Move all frets and notes one fret/semitone higher for the next iteration of the loop.
        frets = frets.iter().map(|f| *f + 1).collect();
        note_indices = note_indices.iter().map(|i| *i + 1).collect();
    }

    tests
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
    for test in generate_tests() {
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

    Ok(())
}
