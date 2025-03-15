use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn read_lines<T>(filename: T) -> io::Result<io::Lines<io::BufReader<File>>>
where
    T: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("wordlist.rs");

    let mut string_builder_inner = Vec::with_capacity(7776); // number of words in list
    let mut words: Option<String> = None;

    if let Ok(lines) = read_lines(Path::new("wordlists/eff_large_wordlist.txt")) {
        for line in lines.map_while(Result::ok) {
            string_builder_inner.push(format!("\"{}\"", line));
        }
        words = Some(string_builder_inner.join(", "));
    }

    let result = [
        "static WORDLIST: &[&str] = &[".to_owned(),
        words.unwrap(),
        "];".to_owned(),
    ]
    .join("");

    fs::write(&dest_path, result).unwrap();
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=wordlists");
}
