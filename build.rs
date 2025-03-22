//! Read the crates default wordlist and inject it into the binary as an array.
#![allow(clippy::unwrap_used, reason = "build script panics are fine")]
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use quote::quote;

// correct as of 5b3d7f8cbfa3b69ae2b917f2b9b53f20f5be1ad6
const WORDLIST_LEN: usize = 7776;

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

    let mut words = Vec::with_capacity(WORDLIST_LEN);

    if let Ok(lines) = read_lines(Path::new("wordlists/eff_large_wordlist.txt")) {
        for line in lines.map_while(Result::ok) {
            words.push(line.to_string());
        }
    }

    let output = quote! {
        static WORDLIST: &[&str] = &[#(#words,)*];
    };

    fs::write(&dest_path, output.to_string()).unwrap();
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=wordlists");
}
