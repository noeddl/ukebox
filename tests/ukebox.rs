/// Integration tests to make sure all possible combinations of user input
/// result in the correct output.
/// This is done by generating all combinations of command line arguments
/// and options together with their expected output. The real program is called
/// using the command line parameters and the actual output is compared to the
/// expected output.
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use rstest::rstest;
use std::collections::HashMap;
use std::fmt;
use std::process::Command; // Run programs
use std::str::FromStr;
use ukebox::chord::ChordType;
use ukebox::chord::FretID;
use ukebox::chord::Tuning;
use ukebox::note::Note;
use ukebox::note::Semitones;

/// A set of parameters to generate tests for all chords produced
/// by moving a specific chord shape along the fretboard.
struct TestConfig {
    chord_type: ChordType,
    tuning: Tuning,
    /// Start index in the note vector (= root of the chord shape).
    start_index: usize,
    /// Distance to the previous chord shape.
    shape_dist: Semitones,
    frets: [FretID; 4],
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
        tuning: Tuning,
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
            tuning,
            start_index,
            shape_dist,
            frets,
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

        Self::new(
            self.chord_type,
            self.start_index,
            self.shape_dist,
            frets,
            self.tuning,
        )
    }

    fn generate_diagram(&self, title: &str, notes: &[&str]) -> String {
        let mut diagram = title.to_string();
        let roots = ["G", "C", "E", "A"];
        let interval = self.tuning.get_interval();

        // Show a symbol for the nut if the chord is played on the lower
        // end of the fretboard. Indicate ongoing strings otherwise.
        let nut = match self.base_fret {
            1 => "||",
            _ => "-|",
        };

        let root_width = self.tuning.get_root_width();

        for i in (0..4).rev() {
            let root = Note::from_str(roots[i]).unwrap() + interval;
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

            let root_str = format!("{:width$}", root.to_string(), width = root_width);
            let line = format!("{} {}{}{}- {}", root_str, sym, nut, string, note);
            diagram.push_str(&format!("{}\n", line));
        }

        // If the fretboard section shown does not include the nut,
        // indicate the number of the first fret shown.
        if self.base_fret > 1 {
            diagram.push_str(&format!(
                "{:width$}\n",
                self.base_fret,
                width = root_width + 6
            ));
        }

        diagram
    }

    fn generate_tests_for_chord(&self, index: usize, note_names: &[&str]) -> (String, Vec<Test>) {
        let mut tests = Vec::new();
        let root = *note_names.iter().cycle().nth(index).unwrap();
        let semitones = self.tuning.get_semitones() as usize;

        // Root notes G, C, E, A.
        let roots = [7, 0, 4, 9];

        let notes: Vec<&str> = roots
            .iter()
            .zip(self.frets.iter())
            .map(|(root, fret)| {
                *note_names
                    .iter()
                    .cycle()
                    .nth(*root as usize + *fret as usize + semitones)
                    .unwrap()
            })
            .collect();

        for j in self.lower_min_fret..self.min_fret + 1 {
            let suffix = get_suffix(self.chord_type);
            let chord = format!("{}{}", root, suffix);
            let title = format!("[{} - {} {}]\n\n", chord, root, self.chord_type);
            let diagram = self.generate_diagram(&title, &notes);
            let test = Test {
                chord,
                tuning: self.tuning,
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

        let rename = |name_list: [&'static str; 12], index, name| {
            let mut names = name_list;
            names[index] = name;
            names
        };

        let mut tests = Vec::new();

        let start_index = self.start_index + self.tuning.get_semitones() as usize;

        // Move upwards the fretboard using the given chord shape.
        for i in 0..13 {
            let index = start_index + i;

            let names = match (index % 12, self.chord_type) {
                // Bm has F#.
                (11, Minor) => note_names,
                // All other minor chords have flat notes.
                (_, Minor) => alt_names,
                // The default for diminished chords is to have flat notes.
                (_, Diminished) => alt_names,
                // Fsus4 has Bb.
                (5, SuspendedFourth) => alt_names,
                // C7, F7.
                (0, DominantSeventh) => alt_names,
                (5, DominantSeventh) => alt_names,
                // Cm7, Fm7, Gm7.
                (0, MinorSeventh) => alt_names,
                (5, MinorSeventh) => alt_names,
                (7, MinorSeventh) => alt_names,
                // CmMaj7, FmMaj7, GmMaj7.
                (0, MinorMajorSeventh) => alt_names,
                (5, MinorMajorSeventh) => alt_names,
                (7, MinorMajorSeventh) => rename(note_names, 10, "Bb"),
                // Caug7, Faug7.
                (0, AugmentedSeventh) => rename(note_names, 10, "Bb"),
                (5, AugmentedSeventh) => rename(note_names, 3, "Eb"),
                // C#dim7, F#dim7.
                (1, DiminishedSeventh) => rename(note_names, 10, "Bb"),
                (6, DiminishedSeventh) => rename(note_names, 3, "Eb"),
                (_, DiminishedSeventh) => alt_names,
                // Half-diminished chords.
                (_, HalfDiminishedSeventh) => alt_names,
                // Default: Use sharp notes.
                (_, _) => note_names,
            };

            let (root, subtests) = self.generate_tests_for_chord(index, &names);
            tests.extend(subtests);

            if root.ends_with("#") {
                let names = match (index % 12, self.chord_type) {
                    // Bbaug has F#.
                    (10, Augmented) => rename(alt_names, 6, "F#"),
                    // Bbaug7 has F#.
                    (10, AugmentedSeventh) => rename(alt_names, 6, "F#"),
                    // BbaugMaj7 has F#.
                    (10, AugmentedMajorSeventh) => rename(alt_names, 6, "F#"),
                    (_, _) => alt_names,
                };

                let (_root, subtests) = self.generate_tests_for_chord(index, &names);
                tests.extend(subtests);
            }

            *self = self.next();
        }

        tests
    }

    fn generate_reverse_tests(&mut self) -> Vec<ReverseTest> {
        let note_names = [
            "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
        ];

        let mut tests = Vec::new();

        let start_index = self.start_index + self.tuning.get_semitones() as usize;

        // Move upwards the fretboard using the given chord shape.
        for i in 0..13 {
            let index = start_index + i;
            let root = *note_names.iter().cycle().nth(index).unwrap();
            let suffix = get_suffix(self.chord_type);
            let chord = format!("{}{}", root, suffix);
            let title = format!("{} - {} {}", chord, root, self.chord_type);
            let fret_str = frets2string(self.frets);

            let test = ReverseTest { fret_str, title };

            tests.push(test);

            *self = self.next();
        }

        tests
    }
}

fn get_suffix(chord_type: ChordType) -> &'static str {
    use ChordType::*;

    match chord_type {
        Major => "",
        Minor => "m",
        SuspendedSecond => "sus2",
        SuspendedFourth => "sus4",
        Augmented => "aug",
        Diminished => "dim",
        DominantSeventh => "7",
        MinorSeventh => "m7",
        MajorSeventh => "maj7",
        MinorMajorSeventh => "mMaj7",
        AugmentedSeventh => "aug7",
        AugmentedMajorSeventh => "augMaj7",
        DiminishedSeventh => "dim7",
        HalfDiminishedSeventh => "m7b5",
    }
}

fn frets2string(frets: [FretID; 4]) -> String {
    // Determine whether to add a space between fret ids. This is only needed if the pattern
    // contains ids consisting of more than one digit, e.g. [7, 7, 7, 10] is converted to
    // "7 7 7 10".
    let space = match frets.iter().filter(|n| **n > 9).count() {
        0 => "",
        _ => " ",
    };

    frets
        .iter()
        .map(|n| format!("{}{}", n, space))
        .collect::<String>()
        .trim()
        .to_string()
}

/// A set of command line arguments and options together with the
/// expected output (chord diagram) to be shown.
struct Test {
    chord: String,
    tuning: Tuning,
    min_fret: FretID,
    diagram: String,
}

impl fmt::Display for Test {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!(
            "cargo run --release -- chart -t {} -f {} {}\n\n{}\n",
            self.tuning, self.min_fret, self.chord, self.diagram
        );

        write!(f, "{}", s)
    }
}

struct ReverseTest {
    fret_str: String,
    title: String,
}

fn run_tests(test_configs: Vec<TestConfig>) -> Result<(), Box<dyn std::error::Error>> {
    for mut test_config in test_configs {
        for test in test_config.generate_tests() {
            // Run `cargo test -- --nocapture` to print all tests run.
            println!("{}", test);

            let mut cmd = Command::cargo_bin("ukebox")?;

            cmd.arg("chart");
            cmd.arg("-t").arg(test.tuning.to_string());

            if test.min_fret > 0 {
                cmd.arg("-f").arg(test.min_fret.to_string());
            }

            cmd.arg(test.chord);

            cmd.assert().success().stdout(format!("{}\n", test.diagram));
        }
    }

    Ok(())
}

fn run_reverse_tests(test_configs: Vec<TestConfig>) -> Result<(), Box<dyn std::error::Error>> {
    let mut tests = HashMap::new();

    for mut test_config in test_configs {
        for test in test_config.generate_reverse_tests() {
            tests
                .entry((test.fret_str, test_config.tuning))
                .or_insert(Vec::new())
                .push(test.title);
        }
    }

    for ((fret_str, tuning), mut titles) in tests {
        let notes = vec!['C', 'D', 'E', 'F', 'G', 'A', 'B'];
        // Sort titles by chord names/notes so that C comes before A etc.
        titles.sort_by(|a, b| {
            notes
                .iter()
                .position(|&x| x == a.chars().next().unwrap())
                .cmp(&notes.iter().position(|&x| x == b.chars().next().unwrap()))
        });
        titles.dedup();
        let title = titles.join("\n");

        let s = format!(
            "cargo run --release -- name -t {} {} \n\n{}\n",
            tuning, fret_str, title
        );

        // Run `cargo test -- --nocapture` to print all tests run.
        println!("{}", s);

        let mut cmd = Command::cargo_bin("ukebox")?;

        cmd.arg("name");
        cmd.arg("-t").arg(tuning.to_string());
        cmd.arg(format!("{}", fret_str));

        cmd.assert().success().stdout(format!("{}\n", title));
    }

    Ok(())
}

#[test]
fn test_no_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ukebox")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE:"));

    Ok(())
}

#[test]
fn test_unknown_chord() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ukebox")?;
    cmd.arg("chart");
    cmd.arg("blafoo");
    cmd.assert().failure().stderr(predicate::str::contains(
        "error: Invalid value for '<chord>': Could not parse chord name \"blafoo\"",
    ));

    Ok(())
}

#[test]
fn test_invalid_pattern() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ukebox")?;
    cmd.arg("name");
    cmd.arg("blafoo");
    cmd.assert().failure().stderr(predicate::str::contains(
        "error: Invalid value for '<fret-pattern>': Fret pattern has wrong format (should be something like 1234 or \"7 8 9 10\")",
    ));

    Ok(())
}

#[test]
fn test_unknown_pattern() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ukebox")?;
    cmd.arg("name");
    cmd.arg("1234");
    cmd.assert()
        .success()
        .stdout("No matching chord was found\n");

    Ok(())
}

fn get_major_chord_config(tuning: Tuning) -> Vec<TestConfig> {
    let ct = ChordType::Major;

    vec![
        TestConfig::new(ct, 0, 1, [0, 0, 0, 3], tuning),
        TestConfig::new(ct, 9, 2, [2, 1, 0, 0], tuning),
        TestConfig::new(ct, 7, 1, [0, 2, 3, 2], tuning),
        TestConfig::new(ct, 5, 1, [2, 0, 1, 0], tuning),
        TestConfig::new(ct, 2, 2, [2, 2, 2, 0], tuning),
    ]
}

fn get_minor_chord_config(tuning: Tuning) -> Vec<TestConfig> {
    let ct = ChordType::Minor;

    vec![
        TestConfig::new(ct, 0, 1, [0, 3, 3, 3], tuning),
        TestConfig::new(ct, 9, 2, [2, 0, 0, 0], tuning),
        TestConfig::new(ct, 7, 1, [0, 2, 3, 1], tuning),
        TestConfig::new(ct, 5, 1, [1, 0, 1, 3], tuning),
        TestConfig::new(ct, 2, 2, [2, 2, 1, 0], tuning),
    ]
}

fn get_suspended_second_chord_config(tuning: Tuning) -> Vec<TestConfig> {
    let ct = ChordType::SuspendedSecond;

    vec![
        TestConfig::new(ct, 0, 1, [0, 2, 3, 3], tuning),
        TestConfig::new(ct, 10, 1, [3, 0, 1, 1], tuning),
        TestConfig::new(ct, 7, 1, [0, 2, 3, 0], tuning),
        TestConfig::new(ct, 5, 1, [0, 0, 1, 3], tuning),
        TestConfig::new(ct, 2, 2, [2, 2, 0, 0], tuning),
    ]
}

fn get_suspended_fourth_chord_config(tuning: Tuning) -> Vec<TestConfig> {
    let ct = ChordType::SuspendedFourth;

    vec![
        TestConfig::new(ct, 0, 1, [0, 0, 1, 3], tuning),
        TestConfig::new(ct, 9, 2, [2, 2, 0, 0], tuning),
        TestConfig::new(ct, 7, 1, [0, 2, 3, 3], tuning),
        TestConfig::new(ct, 5, 1, [3, 0, 1, 1], tuning),
        TestConfig::new(ct, 2, 2, [0, 2, 3, 0], tuning),
    ]
}

fn get_augmented_chord_config(tuning: Tuning) -> Vec<TestConfig> {
    let ct = ChordType::Augmented;

    vec![
        TestConfig::new(ct, 0, 0, [1, 0, 0, 3], tuning),
        TestConfig::new(ct, 9, 2, [2, 1, 1, 0], tuning),
        TestConfig::new(ct, 8, 0, [1, 0, 0, 3], tuning),
        //TestConfig::new(ct, 7, 0, [0, 3, 3, 2], tuning),
        TestConfig::new(ct, 5, 2, [2, 1, 1, 0], tuning),
        TestConfig::new(ct, 4, 0, [1, 0, 0, 3], tuning),
        TestConfig::new(ct, 1, 2, [2, 1, 1, 0], tuning),
    ]
}

fn get_diminished_chord_config(tuning: Tuning) -> Vec<TestConfig> {
    let ct = ChordType::Diminished;

    vec![
        TestConfig::new(ct, 2, 2, [7, 5, 4, 5], tuning),
        TestConfig::new(ct, 10, 2, [3, 1, 0, 1], tuning),
        TestConfig::new(ct, 7, 1, [0, 1, 3, 1], tuning),
        TestConfig::new(ct, 6, 0, [2, 0, 2, 0], tuning),
        TestConfig::new(ct, 3, 2, [2, 3, 2, 0], tuning),
    ]
}

fn get_dominant_seventh_chord_config(tuning: Tuning) -> Vec<TestConfig> {
    let ct = ChordType::DominantSeventh;

    vec![
        TestConfig::new(ct, 0, 1, [0, 0, 0, 1], tuning),
        TestConfig::new(ct, 9, 2, [0, 1, 0, 0], tuning),
        TestConfig::new(ct, 7, 1, [0, 2, 1, 2], tuning),
        TestConfig::new(ct, 4, 1, [1, 2, 0, 2], tuning),
    ]
}

fn get_minor_seventh_chord_config(tuning: Tuning) -> Vec<TestConfig> {
    let ct = ChordType::MinorSeventh;

    vec![
        TestConfig::new(ct, 1, 0, [1, 1, 0, 2], tuning),
        TestConfig::new(ct, 9, 2, [0, 0, 0, 0], tuning),
        TestConfig::new(ct, 7, 1, [0, 2, 1, 1], tuning),
        TestConfig::new(ct, 4, 1, [0, 2, 0, 2], tuning),
    ]
}

fn get_major_seventh_chord_config(tuning: Tuning) -> Vec<TestConfig> {
    let ct = ChordType::MajorSeventh;

    vec![
        TestConfig::new(ct, 0, 1, [0, 0, 0, 2], tuning),
        TestConfig::new(ct, 10, 1, [3, 2, 1, 0], tuning),
        TestConfig::new(ct, 9, 0, [1, 1, 0, 0], tuning),
        TestConfig::new(ct, 7, 1, [0, 2, 2, 2], tuning),
        TestConfig::new(ct, 4, 1, [1, 3, 0, 2], tuning),
    ]
}

fn get_minor_major_seventh_chord_config(tuning: Tuning) -> Vec<TestConfig> {
    let ct = ChordType::MinorMajorSeventh;

    vec![
        TestConfig::new(ct, 1, 0, [1, 1, 0, 3], tuning),
        TestConfig::new(ct, 10, 2, [3, 1, 1, 0], tuning),
        TestConfig::new(ct, 9, 0, [1, 0, 0, 0], tuning),
        TestConfig::new(ct, 7, 1, [0, 2, 2, 1], tuning),
        TestConfig::new(ct, 4, 2, [0, 3, 0, 2], tuning),
    ]
}

fn get_augmented_seventh_chord_config(tuning: Tuning) -> Vec<TestConfig> {
    let ct = ChordType::AugmentedSeventh;

    vec![
        TestConfig::new(ct, 0, 1, [1, 0, 0, 1], tuning),
        TestConfig::new(ct, 9, 2, [0, 1, 1, 0], tuning),
        TestConfig::new(ct, 7, 1, [0, 3, 1, 2], tuning),
        TestConfig::new(ct, 4, 1, [1, 2, 0, 3], tuning),
    ]
}

fn get_augmented_major_seventh_chord_config(tuning: Tuning) -> Vec<TestConfig> {
    let ct = ChordType::AugmentedMajorSeventh;

    vec![
        TestConfig::new(ct, 0, 1, [1, 0, 0, 2], tuning),
        TestConfig::new(ct, 10, 1, [3, 2, 2, 0], tuning),
        TestConfig::new(ct, 9, 0, [1, 1, 1, 0], tuning),
        TestConfig::new(ct, 7, 1, [0, 3, 2, 2], tuning),
        TestConfig::new(ct, 4, 2, [1, 3, 0, 3], tuning),
    ]
}

fn get_diminished_seventh_chord_config(tuning: Tuning) -> Vec<TestConfig> {
    let ct = ChordType::DiminishedSeventh;

    vec![
        TestConfig::new(ct, 1, 2, [0, 1, 0, 1], tuning),
        TestConfig::new(ct, 10, 2, [0, 1, 0, 1], tuning),
        TestConfig::new(ct, 7, 2, [0, 1, 0, 1], tuning),
        TestConfig::new(ct, 4, 2, [0, 1, 0, 1], tuning),
    ]
}

fn get_half_diminished_seventh_chord_config(tuning: Tuning) -> Vec<TestConfig> {
    let ct = ChordType::HalfDiminishedSeventh;

    vec![
        TestConfig::new(ct, 1, 2, [0, 1, 0, 2], tuning),
        TestConfig::new(ct, 10, 2, [1, 1, 0, 1], tuning),
        TestConfig::new(ct, 7, 2, [0, 1, 1, 1], tuning),
        TestConfig::new(ct, 4, 2, [0, 2, 0, 1], tuning),
    ]
}

#[rstest(
    tuning,
    case::c_tuning(Tuning::C),
    case::d_tuning(Tuning::D),
    case::g_tuning(Tuning::G)
)]
fn test_reverse_chords(tuning: Tuning) -> Result<(), Box<dyn std::error::Error>> {
    let mut test_configs = get_major_chord_config(tuning);
    test_configs.extend(get_minor_chord_config(tuning));
    test_configs.extend(get_suspended_second_chord_config(tuning));
    test_configs.extend(get_suspended_fourth_chord_config(tuning));
    test_configs.extend(get_augmented_chord_config(tuning));
    test_configs.extend(get_diminished_chord_config(tuning));
    test_configs.extend(get_dominant_seventh_chord_config(tuning));
    test_configs.extend(get_minor_seventh_chord_config(tuning));
    test_configs.extend(get_major_seventh_chord_config(tuning));
    test_configs.extend(get_minor_major_seventh_chord_config(tuning));
    test_configs.extend(get_augmented_seventh_chord_config(tuning));
    test_configs.extend(get_augmented_major_seventh_chord_config(tuning));
    test_configs.extend(get_diminished_seventh_chord_config(tuning));
    test_configs.extend(get_half_diminished_seventh_chord_config(tuning));

    run_reverse_tests(test_configs)
}

#[rstest(
    tuning,
    case::c_tuning(Tuning::C),
    case::d_tuning(Tuning::D),
    case::g_tuning(Tuning::G)
)]
fn test_major_chords(tuning: Tuning) -> Result<(), Box<dyn std::error::Error>> {
    let test_configs = get_major_chord_config(tuning);

    run_tests(test_configs)
}

#[rstest(
    tuning,
    case::c_tuning(Tuning::C),
    case::d_tuning(Tuning::D),
    case::g_tuning(Tuning::G)
)]
fn test_minor_chords(tuning: Tuning) -> Result<(), Box<dyn std::error::Error>> {
    let test_configs = get_minor_chord_config(tuning);

    run_tests(test_configs)
}

#[rstest(
    tuning,
    case::c_tuning(Tuning::C),
    case::d_tuning(Tuning::D),
    case::g_tuning(Tuning::G)
)]
fn test_suspended_second_chords(tuning: Tuning) -> Result<(), Box<dyn std::error::Error>> {
    let test_configs = get_suspended_second_chord_config(tuning);

    run_tests(test_configs)
}

#[rstest(
    tuning,
    case::c_tuning(Tuning::C),
    case::d_tuning(Tuning::D),
    case::g_tuning(Tuning::G)
)]
fn test_suspended_fourth_chords(tuning: Tuning) -> Result<(), Box<dyn std::error::Error>> {
    let test_configs = get_suspended_fourth_chord_config(tuning);

    run_tests(test_configs)
}

#[rstest(
    tuning,
    case::c_tuning(Tuning::C),
    case::d_tuning(Tuning::D),
    case::g_tuning(Tuning::G)
)]
fn test_augmented_chords(tuning: Tuning) -> Result<(), Box<dyn std::error::Error>> {
    let test_configs = get_augmented_chord_config(tuning);

    run_tests(test_configs)
}

#[rstest(
    tuning,
    case::c_tuning(Tuning::C),
    case::d_tuning(Tuning::D),
    case::g_tuning(Tuning::G)
)]
fn test_diminished_chords(tuning: Tuning) -> Result<(), Box<dyn std::error::Error>> {
    let test_configs = get_diminished_chord_config(tuning);

    run_tests(test_configs)
}

#[rstest(
    tuning,
    case::c_tuning(Tuning::C),
    case::d_tuning(Tuning::D),
    case::g_tuning(Tuning::G)
)]
fn test_dominant_seventh_chords(tuning: Tuning) -> Result<(), Box<dyn std::error::Error>> {
    let test_configs = get_dominant_seventh_chord_config(tuning);

    run_tests(test_configs)
}

#[rstest(
    tuning,
    case::c_tuning(Tuning::C),
    case::d_tuning(Tuning::D),
    case::g_tuning(Tuning::G)
)]
fn test_minor_seventh_chords(tuning: Tuning) -> Result<(), Box<dyn std::error::Error>> {
    let test_configs = get_minor_seventh_chord_config(tuning);

    run_tests(test_configs)
}

#[rstest(
    tuning,
    case::c_tuning(Tuning::C),
    case::d_tuning(Tuning::D),
    case::g_tuning(Tuning::G)
)]
fn test_major_seventh_chords(tuning: Tuning) -> Result<(), Box<dyn std::error::Error>> {
    let test_configs = get_major_seventh_chord_config(tuning);

    run_tests(test_configs)
}

#[rstest(
    tuning,
    case::c_tuning(Tuning::C),
    case::d_tuning(Tuning::D),
    case::g_tuning(Tuning::G)
)]
fn test_minor_major_seventh_chords(tuning: Tuning) -> Result<(), Box<dyn std::error::Error>> {
    let test_configs = get_minor_major_seventh_chord_config(tuning);

    run_tests(test_configs)
}

#[rstest(
    tuning,
    case::c_tuning(Tuning::C),
    case::d_tuning(Tuning::D),
    case::g_tuning(Tuning::G)
)]
fn test_augmented_seventh_chords(tuning: Tuning) -> Result<(), Box<dyn std::error::Error>> {
    let test_configs = get_augmented_seventh_chord_config(tuning);

    run_tests(test_configs)
}

#[rstest(
    tuning,
    case::c_tuning(Tuning::C),
    case::d_tuning(Tuning::D),
    case::g_tuning(Tuning::G)
)]
fn test_augmented_major_seventh_chords(tuning: Tuning) -> Result<(), Box<dyn std::error::Error>> {
    let test_configs = get_augmented_major_seventh_chord_config(tuning);

    run_tests(test_configs)
}

#[rstest(
    tuning,
    case::c_tuning(Tuning::C),
    case::d_tuning(Tuning::D),
    case::g_tuning(Tuning::G)
)]
fn test_diminished_seventh_chords(tuning: Tuning) -> Result<(), Box<dyn std::error::Error>> {
    let test_configs = get_diminished_seventh_chord_config(tuning);

    run_tests(test_configs)
}

#[rstest(
    tuning,
    case::c_tuning(Tuning::C),
    case::d_tuning(Tuning::D),
    case::g_tuning(Tuning::G)
)]
fn test_half_diminished_seventh_chords(tuning: Tuning) -> Result<(), Box<dyn std::error::Error>> {
    let test_configs = get_half_diminished_seventh_chord_config(tuning);

    run_tests(test_configs)
}
