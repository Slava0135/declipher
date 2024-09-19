use cipher::Cipher;
use rand::{seq::SliceRandom, Rng};
use std::{cmp, io::BufRead};

mod cipher;
mod diff;

fn main() {
    let words_file =
        std::fs::File::open("words.dat").expect("Failed to load file with words 'words.dat'!");
    let mut words = Vec::new();
    for word in std::io::BufReader::new(words_file).lines() {
        if let Ok(word) = word {
            words.push(word);
        }
    }

    let mut rng = rand::thread_rng();
    let key_alphabet = "abcdefghijklmnopqrstuvwxyz";
    let text_alphabet = "abcdefghijklmnopqrstuvwxyz";
    let cipher = Cipher {
        key_alphabet: &key_alphabet,
        text_alphabet: &text_alphabet,
    };

    let length = rng.gen_range(20..=30);
    let text_alphabet_len = text_alphabet.chars().count();
    let mut text = String::new();
    let mut chars_without_space = 0;
    for _ in 0..length {
        if chars_without_space > 0 && rng.gen_bool(chars_without_space as f64 / 16.0) || chars_without_space >= 8 {
            text.push(' ');
            chars_without_space = 0;
        } else {
            let next = text_alphabet
                .chars()
                .nth(rng.gen_range(0..text_alphabet_len))
                .unwrap();
            text.push(next);
            chars_without_space += 1;
        }
    }

    let text = text;
    let key = words.choose(&mut rng).expect("No words to pick from!");
    let encoded = cipher.encode(&text, key);

    loop {
        println!();
        for i in 1..=cmp::max(text_alphabet.chars().count(), key_alphabet.chars().count()) {
            print!("{:02} ", i);
        }
        println!();
        print_alphabet(text_alphabet);
        print_alphabet(key_alphabet);
        println!();
        println!("> encoded text:\t'{}'", encoded);
        println!("> enter your guess: ");
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess!");
        let guess = guess.trim_end();
        if guess.chars().all(|it| key_alphabet.contains(it)) {
            let decoded = cipher.decode(&encoded, &guess);
            println!("> decoded text:\t'{}'", decoded);
            if decoded == text {
                println!("> GOOD JOB!");
                break;
            }
            println!("> progress: {:.1}%", 100.0 * diff::diff(&decoded, &text));
        } else {
            println!("> error: invalid string");
        }
    }
}

fn print_alphabet(s: &str) {
    s.chars().for_each(|it| print!("{:02} ", it));
    println!();
}
