pub fn make_wordlist() -> Vec<String> {
    [
        "modern", "labor", "hello", "world", "water", "fire", "deep", "ice", "pie",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

pub fn string_is_uppercase(word: String) -> bool {
    word.chars().all(|c| c.is_uppercase())
}

pub fn string_is_lowercase(word: String) -> bool {
    word.chars().all(|c| c.is_lowercase())
}
