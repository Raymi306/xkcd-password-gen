use xkcd_password_gen::PasswordMaker;
use xkcd_password_gen::config::ConfigBuilder;
use xkcd_password_gen::consts::DEFAULT_SYMBOL_ALPHABET;

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
    opts.optopt("c", "count", "how many passwords to make", "NUM, default=4");
    opts.optopt("w", "word-count", "number of words", "NUM, default=4");
    opts.optopt(
        "",
        "word-min-length",
        "minimum length of a chosen word",
        "NUM, default=4",
    );
    opts.optopt(
        "",
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
        "",
        "digits-before",
        "number of digits to prepend",
        "NUM, default=2",
    );
    opts.optopt(
        "",
        "digits-after",
        "number of digits to append",
        "NUM, default=2",
    );
    opts.optopt("", "padding-type", "how to pad", "TYPE, default=fixed");
    opts.optopt("", "padding-length", "how much to pad", "NUM, default=2");
    opts.optopt(
        "",
        "padding-character",
        "list of characters to choose from",
        &default_symbol_alphabet_help,
    );
    opts.optopt(
        "",
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
        println!("    lower");
        println!("    upper");
        println!("    capitalize-first");
        println!("    capitalize-last");
        println!("    capitalize-not-first");
        println!("    alternating-lower-upper");
        println!("    alternating-upper-lower");
        println!("    random-upper-lower");
        println!("\nPADDING TYPES:");
        println!("    fixed");
        println!("    adaptive");
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
