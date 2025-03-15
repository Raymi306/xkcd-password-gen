mod config;
mod consts;
mod password_maker;
mod test_helpers;
mod types;
mod word_transformer;

use config::ConfigBuilder;
use consts::DEFAULT_SYMBOL_ALPHABET;
use password_maker::PasswordMaker;

use std::env;
use std::process::ExitCode;

use getopts::Options;
use rand::rngs::ThreadRng;

fn main() -> ExitCode {
    let default_symbol_alphabet_help: String = format!(
        "CHOICES, default=\"{}\"",
        DEFAULT_SYMBOL_ALPHABET
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>()
            .join("")
    );

    let args: Vec<String> = env::args().collect();
    let program_name = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "");
    opts.optopt("c", "count", "how many passwords to make", "NUM, default=1");
    opts.optopt("w", "word-count", "number of words", "NUM, default=4");
    opts.optopt(
        "m",
        "word-min-length",
        "minimum length of a chosen word",
        "NUM, default=4",
    );
    opts.optopt(
        "M",
        "word-max-length",
        "maximum length of a chosen word",
        "NUM, default=11",
    );
    opts.optopt(
        "W",
        "word-transformation",
        "transformation to apply to the selected words",
        "TYPE, default=alternating-lower-upper",
    );
    opts.optopt(
        "b",
        "digits-before",
        "number of digits to prepend",
        "NUM, default=2",
    );
    opts.optopt(
        "a",
        "digits-after",
        "number of digits to append",
        "NUM, default=2",
    );
    opts.optopt("T", "padding-type", "how to pad", "TYPE, default=fixed");
    opts.optopt("l", "padding-length", "how much to pad", "NUM, default=2");
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

    let matches_maybe = opts.parse(&args[1..]);

    if let Err(failure) = matches_maybe {
        println!("{}", failure);
        return ExitCode::FAILURE;
    }

    let matches = matches_maybe.unwrap();

    if matches.opt_present("h") || args.len() == 1 {
        let brief = format!("Usage: {} [options]", program_name);
        println!("{}", opts.usage(&brief));
        println!("types are case insensitive");
        println!("\nWORD TRANSFORMATIONS:");
        println!("    lower                   (correct horse battery staple)");
        println!("    upper                   (CORRECT HORSE BATTERY STAPLE)");
        println!("    capitalize-first        (Correct Horse Battery Staple)");
        println!("    capitalize-last         (correcT horsE batterY staplE)");
        println!("    capitalize-not-first    (cORRECT hORSE bATTERY sTAPLE)");
        println!("    alternating-lower-upper (correct HORSE battery STAPLE)");
        println!("    alternating-upper-lower (CORRECT horse BATTERY staple)");
        println!("    random-upper-lower      (correct HORSE battery staple)");
        println!("\nPADDING TYPES:");
        println!("    fixed    (add padding-length padding-characters to front and back)");
        println!("    adaptive (if unpadded password is less than padding-length, pad to length)");
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
        .separator_character(matches.opt_str("separator"));

    let config_result = config_builder.build();

    if let Ok(config) = config_result {
        let mut maker = PasswordMaker::<ThreadRng>::new(config);
        let result = maker.create_passwords();
        for password in result.unwrap() {
            println!("{}", password);
        }
        return ExitCode::SUCCESS;
    }
    ExitCode::FAILURE
}
