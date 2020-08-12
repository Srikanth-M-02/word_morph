use enchant::{Broker, Dict};
use read_input::prelude::*;

fn word_possibilities(word: &str, dict: &Dict) -> Vec<String> {
    let mut words = vec![];
    for i in 0..word.len() {
        for letter in 'a'..'z' {
            let new_word = format!("{}{}{}", &word[..i], letter, &word[i + 1..]);
            if dict.check(&new_word).expect("Unable to check word") {
                words.push(new_word)
            }
        }
    }
    words
}

enum WordCheckResult<'a> {
    Correct,
    Lost(&'a str),
}

fn check_word(
    word: &String,
    completions: &[String],
    used_words: &[String],
) -> WordCheckResult<'static> {
    if word.is_empty() {
        WordCheckResult::Lost("")
    } else if !completions.contains(word) {
        WordCheckResult::Lost("Invalid word")
    } else if used_words.contains(word) {
        WordCheckResult::Lost("Already used word")
    } else {
        WordCheckResult::Correct
    }
}

fn is_sub<T: PartialEq>(mut haystack: &[T], needle: &[T]) -> bool {
    if needle.len() == 0 {
        return true;
    }
    while !haystack.is_empty() {
        if haystack.starts_with(needle) {
            return true;
        }
        haystack = &haystack[1..];
    }
    false
}

fn main() {
    let mut broker = Broker::new();
    const LANG: &str = "en_US";

    if let Ok(dict) = broker.request_dict(LANG) {
        let mut word: String = input().msg("Enter the first word: ").get();
        if !dict.check(&word).expect("Unable to check word") {
            println!("Invalid first word");
            return;
        }

        let mut used_words = vec![word.clone()];

        loop {
            let completions = word_possibilities(&word, &dict);

            if is_sub(&used_words, &completions) {
                println!("No more completions\nYou won");
                break;
            } else {
                let entry: String = input().msg("Enter the next word: ").get();
                match check_word(&entry, &completions, &used_words) {
                    WordCheckResult::Correct => {
                        used_words.push(entry.clone());
                        word = entry;
                    }
                    WordCheckResult::Lost(reason) => {
                        println!("{}\nYou lost", reason);
                        println!(
                            "You could have used {}",
                            completions
                                .iter()
                                .filter(|x| !used_words.contains(*x))
                                .map(|s| s.to_string())
                                .collect::<Vec<_>>()
                                .join(" or ")
                        );
                        break;
                    }
                }
            }
        }
    } else {
        eprintln!("en_US dictionary not found");
    }
}
