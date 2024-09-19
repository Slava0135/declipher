#[derive(Debug)]
pub struct Cipher<'a> {
    text_alphabet: &'a str,
    key_alphabet: &'a str,
}

impl Cipher<'_> {
    pub fn encode(self, text: &str, key: &str) -> String {
        assert!(text.chars().all(|it| self.text_alphabet.contains(it)));
        assert!(key.chars().all(|it| self.key_alphabet.contains(it)));
        return String::from(text);
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
}