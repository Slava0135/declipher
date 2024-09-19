use cipher::Cipher;
use rand::{seq::SliceRandom, Rng};
use std::io::BufRead;

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
    let text: String = (0..length)
        .map(|_| {
            text_alphabet
                .chars()
                .nth(rng.gen_range(0..text_alphabet_len))
                .unwrap()
        })
        .collect();
    let key = words.choose(&mut rng).expect("No words to pick from!");
    let encoded = cipher.encode(&text, key);

    loop {
        println!();
        println!("-- TEXT ALPHABET --");
        print_alphabet(text_alphabet);
        println!("-- KEY ALPHABET --");
        print_alphabet(key_alphabet);
        println!();
        println!("encoded text:\t'{}'", encoded);
        println!("enter your guess: ");
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess!");
        let decoded = cipher.decode(&encoded, &guess.trim_end());
        println!("decoded text:\t'{}'", decoded);
        if decoded == text {
            println!("GOOD JOB!");
            break;
        }
        println!("Decoded: {:.1}%", diff::diff(&decoded, &text));
    }
}

fn print_alphabet(s: &str) {
    println!("\t{} ", s);
    print!("\t");
    for i in 1..=s.chars().count() {
        print!("{} ", i);
    }
    println!();
}
