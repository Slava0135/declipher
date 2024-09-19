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
}
