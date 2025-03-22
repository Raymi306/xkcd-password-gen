//! Create memorable passwords.
//!
//! Use custom configurations, or roll with the defaults.
use std::env;
use std::process::ExitCode;

use getopts::Options;
use rand::rngs::OsRng;
use rand::rngs::ThreadRng;

mod config;
mod consts;
mod password_maker;
mod test_helpers;
mod types;
mod word_transformer;

use config::ConfigBuilder;
use consts::default;
use password_maker::PasswordMaker;
use types::PaddingType;
use types::RngType;
use types::WordTransformationType;

/// The entrypoint.
///
/// Here, we define the program's CLI arguments.
/// We use the [`getopts` library](https://docs.rs/getopts/latest/getopts/) to accomplish this.
/// We check which arguments the user passed in and create a [`config::Config`].
/// Finally, we generate passwords using the specified configuration.
#[expect(
    clippy::too_many_lines,
    reason = "As long as it is fairly simple and readable..."
)]
fn main() -> ExitCode {
    // TODO this ought to be const
    let default_symbol_alphabet_help: String = format!(
        "CHOICES, default=\"{}\"",
        default::SYMBOL_ALPHABET.into_iter().collect::<String>()
    );

    let args: Vec<String> = env::args().collect();
    let program_name = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "");
    opts.optopt(
        "c",
        "count",
        "how many passwords to make",
        &format!("NUM, default={}", default::COUNT),
    );
    opts.optopt(
        "w",
        "word-count",
        "number of words",
        &format!("NUM, default={}", default::WORD_COUNT),
    );
    opts.optopt(
        "m",
        "word-min-length",
        "minimum length of a chosen word",
        &format!("NUM, default={}", default::WORD_MIN_LENGTH),
    );
    opts.optopt(
        "M",
        "word-max-length",
        "maximum length of a chosen word",
        &format!("NUM, default={}", default::WORD_MAX_LENGTH),
    );
    opts.optopt(
        "W",
        "word-transformation",
        "transformation to apply to the selected words",
        &format!("TYPE, default={}", &WordTransformationType::default()),
    );
    opts.optopt(
        "b",
        "digits-before",
        "number of digits to prepend",
        &format!("NUM, default={}", default::DIGITS_BEFORE),
    );
    opts.optopt(
        "a",
        "digits-after",
        "number of digits to append",
        &format!("NUM, default={}", default::DIGITS_AFTER),
    );
    opts.optopt(
        "T",
        "padding-type",
        "how to apply padding",
        &format!("TYPE, default={}", &PaddingType::default()),
    );
    opts.optopt(
        "l",
        "padding-length",
        "how much to pad",
        &format!(
            "NUM, default={} for fixed, {} for adaptive",
            default::PADDING_LENGTH_FIXED,
            default::PADDING_LENGTH_ADAPTIVE
        ),
    );
    opts.optopt(
        "p",
        "padding-character",
        "list of characters to choose from",
        &default_symbol_alphabet_help,
    );
    opts.optopt(
        "s",
        "separator",
        "list of characters to choose from",
        &default_symbol_alphabet_help,
    );
    opts.optopt(
        "r",
        "rng",
        "method of random number generation",
        &format!("TYPE, default={}", &RngType::default()),
    );

    let matches = match opts.parse(&args[1..]) {
        Ok(v) => v,
        Err(failure) => {
            eprintln!("{failure}");
            return ExitCode::FAILURE;
        }
    };

    // if the help flag is present or if there are unused arguments, display a help message.
    if matches.opt_present("h") || !matches.free.is_empty() {
        let brief = format!("Usage: {program_name} [options]");
        println!("{}", opts.usage(&brief));
        println!("types are case insensitive");
        // TODO make less brittle, see crate::types
        println!("\nWORD TRANSFORMATIONS:");
        println!("    none");
        println!("    lower                   (correct horse battery staple)");
        println!("    upper                   (CORRECT HORSE BATTERY STAPLE)");
        println!("    capitalize-first        (Correct Horse Battery Staple)");
        println!("    capitalize-last         (correcT horsE batterY staplE)");
        println!("    capitalize-not-first    (cORRECT hORSE bATTERY sTAPLE)");
        println!("    alternating-lower-upper (correct HORSE battery STAPLE)");
        println!("    alternating-upper-lower (CORRECT horse BATTERY staple)");
        println!("    random-upper-lower      (correct HORSE battery staple)");
        println!("\nPADDING TYPES:");
        println!("    none");
        println!("    fixed    (add padding-length padding-characters to front and back)");
        #[rustfmt::skip]
        println!("    adaptive (if unpadded password is less than padding-length, append padding-characters to meet length)");
        println!("\nRNG TYPES:");
        println!("    os-rng (the system's native secure RNG)");
        println!("    csprng (a reasonably secure userspace RNG)");
        return ExitCode::SUCCESS;
    }

    // TODO this boilerplate could be reduced
    let config_builder = ConfigBuilder::new()
        .count(matches.opt_str("count"))
        .word_count(matches.opt_str("word-count"))
        .word_min_length(matches.opt_str("word-min-length"))
        .word_max_length(matches.opt_str("word-max-length"))
        .word_transformation(matches.opt_str("word-transformation"))
        .digits_before(matches.opt_str("digits-before"))
        .digits_after(matches.opt_str("digits-after"))
        .padding_type(matches.opt_str("padding-type"))
        .padding_length(matches.opt_str("padding-length"))
        .padding_character(matches.opt_str("padding-character"))
        .separator_character(matches.opt_str("separator"))
        .rng_type(matches.opt_str("rng"));

    match config_builder.build() {
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
        Ok(config) => {
            let result = match config.rng_type {
                RngType::OsRng => PasswordMaker::<OsRng>::new(config).create_passwords(),
                RngType::Csprng => PasswordMaker::<ThreadRng>::new(config).create_passwords(),
            };
            for password in result {
                println!("{password}");
            }
            ExitCode::SUCCESS
        }
    }
}
