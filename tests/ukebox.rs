use std::error::Error;
use std::process::Command; // Run programs

use assert_cmd::prelude::*; // Add methods on commands
use indoc::indoc;
use predicates::prelude::*; // Used for writing assertions
use rstest::rstest;

#[test]
fn test_no_args() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("ukebox")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE:"));

    Ok(())
}

#[test]
fn test_unknown_chord() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("ukebox")?;
    cmd.arg("chart");
    cmd.arg("blafoo");
    cmd.assert().failure().stderr(predicate::str::contains(
        "error: Invalid value for '<CHORD>': Could not parse chord name \"blafoo\"",
    ));

    Ok(())
}

#[test]
fn test_no_voicing_found() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("ukebox")?;
    cmd.arg("chart");
    cmd.arg("--max-span").arg("0");
    cmd.arg("C");
    cmd.assert()
        .success()
        .stdout("No matching chord voicing was found\n");

    Ok(())
}

#[rstest(min_fret, case("22"), case("foo"))]
fn test_invalid_min_fret(min_fret: &str) -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("ukebox")?;
    cmd.arg("chart");
    cmd.arg("--min-fret").arg(min_fret);
    cmd.arg("C");
    cmd.assert().failure().stderr(predicate::str::contains(
        "error: Invalid value for '--min-fret <FRET_ID>': must be a number between 0 and 21",
    ));

    Ok(())
}

#[rstest(max_fret, case("22"), case("foo"))]
fn test_invalid_max_fret(max_fret: &str) -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("ukebox")?;
    cmd.arg("chart");
    cmd.arg("--max-fret").arg(max_fret);
    cmd.arg("C");
    cmd.assert().failure().stderr(predicate::str::contains(
        "error: Invalid value for '--max-fret <FRET_ID>': must be a number between 0 and 21",
    ));

    Ok(())
}

#[rstest(max_span, case("6"), case("foo"))]
fn test_invalid_max_span(max_span: &str) -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("ukebox")?;
    cmd.arg("chart");
    cmd.arg("--max-span").arg(max_span);
    cmd.arg("C");
    cmd.assert().failure().stderr(predicate::str::contains(
        "error: Invalid value for '--max-span <FRET_COUNT>': must be a number between 0 and 5",
    ));

    Ok(())
}

#[test]
fn test_invalid_pattern() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("ukebox")?;
    cmd.arg("name");
    cmd.arg("blafoo");
    cmd.assert().failure().stderr(predicate::str::contains(
        "error: Invalid value for '<FRET_PATTERN>': Fret pattern has wrong format (should be something like 1234 or \"7 8 9 10\")",
    ));

    Ok(())
}

#[test]
fn test_unknown_pattern() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("ukebox")?;
    cmd.arg("name");
    cmd.arg("1234");
    cmd.assert()
        .success()
        .stdout("No matching chord was found\n");

    Ok(())
}

#[rstest(
    chord,
    chart,
    case(
        "C",
        indoc!("
            [C - C major]

            A  ||---|---|-o-|---|- C
            E o||---|---|---|---|- E
            C o||---|---|---|---|- C
            G o||---|---|---|---|- G
        ")
    ),
    case(
        "C#",
        indoc!("
            [C# - C# major]

            A  ||---|---|---|-o-|- C#
            E  ||-o-|---|---|---|- F
            C  ||-o-|---|---|---|- C#
            G  ||-o-|---|---|---|- G#
        ")
    ),
    case(
        "Db",
        indoc!("
            [Db - Db major]

            A  ||---|---|---|-o-|- Db
            E  ||-o-|---|---|---|- F
            C  ||-o-|---|---|---|- Db
            G  ||-o-|---|---|---|- Ab
        ")
    ),
)]
fn test_chart(chord: &str, chart: &'static str) -> Result<(), Box<dyn Error + 'static>> {
    let mut cmd = Command::cargo_bin("ukebox")?;
    cmd.arg("chart").arg(chord);
    cmd.assert().success().stdout(format!("{}\n", chart));

    Ok(())
}

#[rstest(
    chord,
    tuning,
    chart,
    case(
        "C",
        "C",
        indoc!("
            [C - C major]

            A  ||---|---|-o-|---|- C
            E o||---|---|---|---|- E
            C o||---|---|---|---|- C
            G o||---|---|---|---|- G
        ")
    ),
    case(
        "D",
        "D",
        indoc!("
            [D - D major]

            B   ||---|---|-o-|---|- D
            F# o||---|---|---|---|- F#
            D  o||---|---|---|---|- D
            A  o||---|---|---|---|- A
        ")
    ),
    case(
        "G",
        "G",
        indoc!("
            [G - G major]

            E  ||---|---|-o-|---|- G
            B o||---|---|---|---|- B
            G o||---|---|---|---|- G
            D o||---|---|---|---|- D
        ")
    ),
)]
fn test_tuning(
    chord: &str,
    tuning: &str,
    chart: &'static str,
) -> Result<(), Box<dyn Error + 'static>> {
    let mut cmd = Command::cargo_bin("ukebox")?;
    cmd.arg("chart");
    cmd.arg("--tuning").arg(tuning);
    cmd.arg(chord);
    cmd.assert().success().stdout(format!("{}\n", chart));

    Ok(())
}

#[rstest(
    chord,
    min_fret,
    chart,
    case(
        "C",
        "1",
        indoc!("
            [C - C major]

            A  -|-o-|---|---|---|- C
            E  -|-o-|---|---|---|- G
            C  -|---|-o-|---|---|- E
            G  -|---|---|-o-|---|- C
                  3
        ")
    ),
    case(
        "C",
        "3",
        indoc!("
            [C - C major]

            A  -|-o-|---|---|---|- C
            E  -|-o-|---|---|---|- G
            C  -|---|-o-|---|---|- E
            G  -|---|---|-o-|---|- C
                  3
        ")
    ),
    case(
        "C",
        "10",
        indoc!("
            [C - C major]

            A  -|-o-|---|---|---|- G
            E  -|---|---|-o-|---|- E
            C  -|---|---|-o-|---|- C
            G  -|---|---|-o-|---|- G
                 10
        ")
    ),
)]
fn test_min_fret(
    chord: &str,
    min_fret: &str,
    chart: &'static str,
) -> Result<(), Box<dyn Error + 'static>> {
    let mut cmd = Command::cargo_bin("ukebox")?;
    cmd.arg("chart");
    cmd.arg("--min-fret").arg(min_fret);
    cmd.arg(chord);
    cmd.assert().success().stdout(format!("{}\n", chart));

    Ok(())
}

#[rstest(
    chord,
    max_span,
    chart,
    case(
        "C#",
        "3",
        indoc!("
            [C# - C# major]

            A  -|-o-|---|---|---|- C#
            E  -|-o-|---|---|---|- G#
            C  -|---|-o-|---|---|- F
            G  -|---|---|-o-|---|- C#
                  4
        ")
    ),
    case(
        "C#",
        "5",
        indoc!("
            [C# - C# major]

            A  ||---|---|---|-o-|---|- C#
            E  ||-o-|---|---|---|---|- F
            C  ||-o-|---|---|---|---|- C#
            G  ||-o-|---|---|---|---|- G#
        ")
    ),
)]
fn test_max_span(
    chord: &str,
    max_span: &str,
    chart: &'static str,
) -> Result<(), Box<dyn Error + 'static>> {
    let mut cmd = Command::cargo_bin("ukebox")?;
    cmd.arg("chart");
    cmd.arg("--max-span").arg(max_span);
    cmd.arg(chord);
    cmd.assert().success().stdout(format!("{}\n", chart));

    Ok(())
}

#[rstest(
    chord,
    semitones,
    chart,
    case(
        "C",
        "0",
        indoc!("
            [C - C major]

            A  ||---|---|-o-|---|- C
            E o||---|---|---|---|- E
            C o||---|---|---|---|- C
            G o||---|---|---|---|- G
        ")
    ),
    case(
        "C",
        "+1",
        indoc!("
            [C# - C# major]

            A  ||---|---|---|-o-|- C#
            E  ||-o-|---|---|---|- F
            C  ||-o-|---|---|---|- C#
            G  ||-o-|---|---|---|- G#
        ")
    ),
    case(
        "C",
        "1",
        indoc!("
            [C# - C# major]

            A  ||---|---|---|-o-|- C#
            E  ||-o-|---|---|---|- F
            C  ||-o-|---|---|---|- C#
            G  ||-o-|---|---|---|- G#
        ")
    ),
    case(
        "D",
        "-1",
        indoc!("
            [Db - Db major]

            A  ||---|---|---|-o-|- Db
            E  ||-o-|---|---|---|- F
            C  ||-o-|---|---|---|- Db
            G  ||-o-|---|---|---|- Ab
        ")
    ),
)]
fn test_transpose(
    chord: &str,
    semitones: &str,
    chart: &'static str,
) -> Result<(), Box<dyn Error + 'static>> {
    let mut cmd = Command::cargo_bin("ukebox")?;
    cmd.arg("chart");
    cmd.arg("--transpose").arg(semitones);
    cmd.arg(chord);
    cmd.assert().success().stdout(format!("{}\n", chart));

    Ok(())
}

#[rstest(
    chord,
    min_fret,
    max_fret,
    chart,
    case(
        "C#",
        None,
        None,
        indoc!("
            [C# - C# major]

            A  ||---|---|---|-o-|- C#
            E  ||-o-|---|---|---|- F
            C  ||-o-|---|---|---|- C#
            G  ||-o-|---|---|---|- G#

            A  -|-o-|---|---|---|- C#
            E  -|-o-|---|---|---|- G#
            C  -|---|-o-|---|---|- F
            G  -|---|---|-o-|---|- C#
                  4

            A  -|---|---|-o-|---|- F
            E  -|---|---|---|-o-|- C#
            C  -|---|---|-o-|---|- G#
            G  -|-o-|---|---|---|- C#
                  6

            A  -|-o-|---|---|---|- F
            E  -|---|-o-|---|---|- C#
            C  -|-o-|---|---|---|- G#
            G  -|---|---|-o-|---|- F
                  8

            A  -|---|---|---|-o-|- G#
            E  -|---|-o-|---|---|- C#
            C  -|-o-|---|---|---|- G#
            G  -|---|---|-o-|---|- F
                  8
        ")
    ),
    case(
        "C#",
        Some("5"),
        None,
        indoc!("
            [C# - C# major]

            A  -|---|---|-o-|---|- F
            E  -|---|---|---|-o-|- C#
            C  -|---|---|-o-|---|- G#
            G  -|-o-|---|---|---|- C#
                  6

            A  -|-o-|---|---|---|- F
            E  -|---|-o-|---|---|- C#
            C  -|-o-|---|---|---|- G#
            G  -|---|---|-o-|---|- F
                  8

            A  -|---|---|---|-o-|- G#
            E  -|---|-o-|---|---|- C#
            C  -|-o-|---|---|---|- G#
            G  -|---|---|-o-|---|- F
                  8
        ")
    ),
    case(
        "C#",
        None,
        Some("10"),
        indoc!("
            [C# - C# major]

            A  ||---|---|---|-o-|- C#
            E  ||-o-|---|---|---|- F
            C  ||-o-|---|---|---|- C#
            G  ||-o-|---|---|---|- G#

            A  -|-o-|---|---|---|- C#
            E  -|-o-|---|---|---|- G#
            C  -|---|-o-|---|---|- F
            G  -|---|---|-o-|---|- C#
                  4

            A  -|---|---|-o-|---|- F
            E  -|---|---|---|-o-|- C#
            C  -|---|---|-o-|---|- G#
            G  -|-o-|---|---|---|- C#
                  6

            A  -|-o-|---|---|---|- F
            E  -|---|-o-|---|---|- C#
            C  -|-o-|---|---|---|- G#
            G  -|---|---|-o-|---|- F
                  8
        ")
    ),
    case(
        "C#",
        Some("5"),
        Some("10"),
        indoc!("
            [C# - C# major]

            A  -|---|---|-o-|---|- F
            E  -|---|---|---|-o-|- C#
            C  -|---|---|-o-|---|- G#
            G  -|-o-|---|---|---|- C#
                  6

            A  -|-o-|---|---|---|- F
            E  -|---|-o-|---|---|- C#
            C  -|-o-|---|---|---|- G#
            G  -|---|---|-o-|---|- F
                  8
        ")
    ),
)]
fn test_all(
    chord: &str,
    min_fret: Option<&str>,
    max_fret: Option<&str>,
    chart: &'static str,
) -> Result<(), Box<dyn Error + 'static>> {
    let mut cmd = Command::cargo_bin("ukebox")?;
    cmd.arg("chart").arg("--all");

    if let Some(fret) = min_fret {
        cmd.arg("--min-fret").arg(fret);
    }

    if let Some(fret) = max_fret {
        cmd.arg("--max-fret").arg(fret);
    }

    cmd.arg(chord);
    cmd.assert().success().stdout(format!("{}\n", chart));

    Ok(())
}

#[rstest(
    chart,
    names,
    case("0000", "Am7 - A minor 7th"),
    case("0003", "C - C major"),
    case("0013", "Csus4 - C suspended 4th\nFsus2 - F suspended 2nd"),
    case("10 10 10 10", "Gm7 - G minor 7th")
)]
fn test_name(chart: &str, names: &'static str) -> Result<(), Box<dyn Error + 'static>> {
    let mut cmd = Command::cargo_bin("ukebox")?;
    cmd.arg("name").arg(chart);
    cmd.assert().success().stdout(format!("{}\n", names));

    Ok(())
}

#[rstest(
    chart,
    tuning,
    names,
    case("0003", "C", "C - C major"),
    case("0003", "D", "D - D major"),
    case("0003", "G", "G - G major"),
    case("10 10 10 10", "C", "Gm7 - G minor 7th"),
    case("10 10 10 10", "D", "Am7 - A minor 7th"),
    case("10 10 10 10", "G", "Dm7 - D minor 7th")
)]
fn test_name_with_tuning(
    chart: &str,
    tuning: &str,
    names: &'static str,
) -> Result<(), Box<dyn Error + 'static>> {
    let mut cmd = Command::cargo_bin("ukebox")?;
    cmd.arg("name");
    cmd.arg("--tuning").arg(tuning);
    cmd.arg(chart);
    cmd.assert().success().stdout(format!("{}\n", names));

    Ok(())
}
