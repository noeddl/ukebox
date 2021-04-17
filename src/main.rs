use lazy_static::lazy_static;
use structopt::StructOpt;
use ukebox::{
    Chord, ChordChart, ChordSequence, FretID, FretPattern, Semitones, Tuning, Voicing,
    VoicingConfig, VoicingGraph,
};

/// Maximal possible fret ID.
/// According to Wikipedia, the biggest ukulele type (baritone) has 21 frets.
const MAX_FRET_ID: FretID = 21;

/// Maximal span of frets.
/// Playing a chord that spans more than 5 frets seems anatomically impossible to me.
const MAX_SPAN: Semitones = 5;

// See https://github.com/TeXitoi/structopt/issues/150
lazy_static! {
    static ref DEFAULT_CONFIG: VoicingConfig = VoicingConfig::default();
    static ref TUNING_STR: String = DEFAULT_CONFIG.tuning.to_string();
    static ref MIN_FRET_STR: String = DEFAULT_CONFIG.min_fret.to_string();
    static ref MAX_FRET_STR: String = DEFAULT_CONFIG.max_fret.to_string();
    static ref MAX_SPAN_STR: String = DEFAULT_CONFIG.max_span.to_string();
}

#[derive(StructOpt)]
struct Ukebox {
    /// Type of tuning to be used
    #[structopt(short, long, global = true, value_name = "TUNING", default_value = &TUNING_STR, possible_values = &Tuning::variants())]
    tuning: Tuning,
    #[structopt(subcommand)]
    cmd: Subcommand,
}

#[derive(StructOpt)]
enum Subcommand {
    /// Chord chart lookup
    Chart {
        /// Print out all voicings of <chord> that fulfill the given conditions
        #[structopt(short, long)]
        all: bool,
        #[structopt(flatten)]
        voicing_opts: VoicingOpts,
        /// Name of the chord to be shown    
        #[structopt(value_name = "CHORD")]
        chord: Chord,
    },
    /// Chord name lookup
    Name {
        /// A compact chart representing the finger positions of the chord to be looked up
        #[structopt(value_name = "FRET_PATTERN")]
        fret_pattern: FretPattern,
    },
    /// Voice leading for a sequence of chords
    VoiceLead {
        #[structopt(flatten)]
        voicing_opts: VoicingOpts,
        /// Chord sequence
        #[structopt(value_name = "CHORD_SEQUENCE")]
        chord_seq: ChordSequence,
    },
}

#[derive(StructOpt)]
pub struct VoicingOpts {
    /// Minimal fret (= minimal position) from which to play <chord>
    #[structopt(long, value_name = "FRET_ID", default_value = &MIN_FRET_STR, validator = validate_fret_id)]
    min_fret: FretID,
    /// Maximal fret up to which to play <chord>
    #[structopt(long, value_name = "FRET_ID", default_value = &MAX_FRET_STR, validator = validate_fret_id)]
    max_fret: FretID,
    /// Maximal span between the first and the last fret pressed down when playing <chord>
    #[structopt(long, value_name = "FRET_COUNT", default_value = &MAX_SPAN_STR, validator = validate_span)]
    max_span: Semitones,
    /// Number of semitones to add (e.g. 1, +1) or to subtract (e.g. -1)
    #[structopt(
        long,
        value_name = "SEMITONES",
        allow_hyphen_values = true,
        default_value = "0"
    )]
    transpose: i8,
}

fn validate_fret_id(s: String) -> Result<(), String> {
    if let Ok(fret) = s.parse::<FretID>() {
        if fret <= MAX_FRET_ID {
            return Ok(());
        }
    }

    Err(String::from("must be a number between 0 and 21"))
}

fn validate_span(s: String) -> Result<(), String> {
    if let Ok(span) = s.parse::<Semitones>() {
        if span <= MAX_SPAN {
            return Ok(());
        }
    }

    Err(String::from("must be a number between 0 and 5"))
}

fn main() {
    let args = Ukebox::from_args();
    let tuning = args.tuning;

    match args.cmd {
        Subcommand::Chart {
            all,
            voicing_opts,
            chord,
        } => {
            let chord = chord.transpose(voicing_opts.transpose);

            let config = VoicingConfig {
                tuning,
                min_fret: voicing_opts.min_fret,
                max_fret: voicing_opts.max_fret,
                max_span: voicing_opts.max_span,
            };

            let mut voicings = chord.voicings(config).peekable();

            if voicings.peek().is_none() {
                println!("No matching chord voicing was found");
            } else {
                println!("{}", format!("[{}]\n", chord));
            }

            for voicing in voicings {
                let chart = ChordChart::new(voicing, voicing_opts.max_span);
                println!("{}", chart);

                if !all {
                    break;
                }
            }
        }
        Subcommand::Name { fret_pattern } => {
            let voicing = Voicing::new(fret_pattern, tuning);
            let chords = voicing.get_chords();

            if chords.is_empty() {
                println!("No matching chord was found");
            }

            for chord in chords {
                println!("{}", chord);
            }
        }
        Subcommand::VoiceLead {
            voicing_opts,
            chord_seq,
        } => {
            let chord_seq = chord_seq.transpose(voicing_opts.transpose);

            let config = VoicingConfig {
                tuning,
                min_fret: voicing_opts.min_fret,
                max_fret: voicing_opts.max_fret,
                max_span: voicing_opts.max_span,
            };

            let mut voicing_graph = VoicingGraph::new(config);
            voicing_graph.add(&chord_seq);

            if let Some(path) = voicing_graph.find_best_path() {
                for voicing in path {
                    println!("{:?}", voicing);
                }
            }
        }
    }
}
