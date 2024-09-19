#[derive(Debug)]
pub struct Cipher<'a> {
    text_alphabet: &'a str,
    key_alphabet: &'a str,
}

impl Cipher<'_> {
    pub fn encode(self, text: &str, key: &str) -> String {
        assert!(text.chars().all(|it| self.text_alphabet.contains(it)));
        assert!(key.chars().all(|it| self.key_alphabet.contains(it)));
        if key.is_empty() {
            return String::from(text);
        }
        let text_index = str::find(self.text_alphabet, text.chars().nth(0).unwrap()).unwrap();
        let key_index = str::find(self.key_alphabet, key.chars().nth(0).unwrap()).unwrap();
        let combined_index = (text_index + key_index + 1) % self.text_alphabet.chars().count();
        return String::from(self.text_alphabet.chars().nth(combined_index).unwrap());
    }
}

mod tests {
    use super::*;

    const DEFAULT_CIPHER: Cipher<'_> = Cipher{text_alphabet: "abc", key_alphabet: "def"};

    #[test]
    #[should_panic]
    fn encode_wrong_text_alphabet() {
        DEFAULT_CIPHER.encode("?", "d");
    }

    #[test]
    #[should_panic]
    fn encode_wrong_key_alphabet() {
        DEFAULT_CIPHER.encode("a", "?");
    }

    #[test]
    fn encode_empty() {
        assert_eq!("a", DEFAULT_CIPHER.encode("a", ""));
    }

    #[test]
    fn encode_one() {
        assert_eq!("b", DEFAULT_CIPHER.encode("a", "d"));
        assert_eq!("c", DEFAULT_CIPHER.encode("a", "e"));
        assert_eq!("a", DEFAULT_CIPHER.encode("a", "f"));
    }
}