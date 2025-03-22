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
use consts::DEFAULT_COUNT;
use consts::DEFAULT_DIGITS_AFTER;
use consts::DEFAULT_DIGITS_BEFORE;
use consts::DEFAULT_PADDING_LENGTH_ADAPTIVE;
use consts::DEFAULT_PADDING_LENGTH_FIXED;
use consts::DEFAULT_SYMBOL_ALPHABET;
use consts::DEFAULT_WORD_COUNT;
use consts::DEFAULT_WORD_MAX_LENGTH;
use consts::DEFAULT_WORD_MIN_LENGTH;
use password_maker::PasswordMaker;
use types::PaddingType;
use types::RngType;
use types::WordTransformationType;

#[expect(
    clippy::too_many_lines,
    reason = "As long as it is fairly simple and readable..."
)]
fn main() -> ExitCode {
    let default_symbol_alphabet_help: String = format!(
        "CHOICES, default=\"{}\"",
        DEFAULT_SYMBOL_ALPHABET
            .into_iter()
            .map(String::from)
            .collect::<String>()
    );

    let args: Vec<String> = env::args().collect();
    let program_name = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "");
    opts.optopt(
        "c",
        "count",
        "how many passwords to make",
        &format!("NUM, default={DEFAULT_COUNT}"),
    );
    opts.optopt(
        "w",
        "word-count",
        "number of words",
        &format!("NUM, default={DEFAULT_WORD_COUNT}"),
    );
    opts.optopt(
        "m",
        "word-min-length",
        "minimum length of a chosen word",
        &format!("NUM, default={DEFAULT_WORD_MIN_LENGTH}"),
    );
    opts.optopt(
        "M",
        "word-max-length",
        "maximum length of a chosen word",
        &format!("NUM, default={DEFAULT_WORD_MAX_LENGTH}"),
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
        &format!("NUM, default={DEFAULT_DIGITS_BEFORE}"),
    );
    opts.optopt(
        "a",
        "digits-after",
        "number of digits to append",
        &format!("NUM, default={DEFAULT_DIGITS_AFTER}"),
    );
    opts.optopt(
        "T",
        "padding-type",
        "how to pad",
        &format!("TYPE, default={}", &PaddingType::default()),
    );
    opts.optopt(
        "l",
        "padding-length",
        "how much to pad",
        &format!("NUM, default={DEFAULT_PADDING_LENGTH_FIXED} for fixed, {DEFAULT_PADDING_LENGTH_ADAPTIVE} for adaptive"),
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

    if matches.opt_present("h") || args.len() == 1 || !matches.free.is_empty() {
        let brief = format!("Usage: {program_name} [options]");
        println!("{}", opts.usage(&brief));
        println!("types are case insensitive");
        // TODO make less brittle
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
        println!("    adaptive (if unpadded password is less than padding-length, pad to length)");
        println!("\nRNG TYPES:");
        println!("    os-rng (the system's native secure RNG)");
        println!("    csprng (a reasonably secure userspace RNG)");
        return ExitCode::SUCCESS;
    }

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
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
