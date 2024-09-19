#[derive(Debug)]
pub struct Cipher<'a> {
    text_alphabet: &'a str,
    key_alphabet: &'a str,
}

impl Cipher<'_> {

    pub fn encode(self, text: &str, key: &str) -> String {
        fn go(text_index: usize, key_index: usize, text_alphabet_len: usize) -> usize {
            (text_index + key_index + 1) % text_alphabet_len
        }
        return self.common(text, key, go);
    }

    pub fn decode(self, text: &str, key: &str) -> String {
        fn go(text_index: usize, key_index: usize, text_alphabet_len: usize) -> usize {
            (text_alphabet_len + text_index - (key_index + 1)) % text_alphabet_len
        }
        return self.common(text, key, go);
    }

    fn common(self, text: &str, key: &str, transform: fn(text_index: usize, key_index: usize, text_alphabet_len: usize) -> usize) -> String {
        assert!(text.chars().all(|it| it.is_whitespace() || self.text_alphabet.contains(it)));
        assert!(key.chars().all(|it| it.is_whitespace() || self.key_alphabet.contains(it)));
        if key.is_empty() {
            return String::from(text);
        }
        let mut result = String::new();
        let mut index = 0;
        let key_len = key.chars().count();
        let text_alphabet_len = self.text_alphabet.chars().count();
        for text_ch in text.chars() {
            if text_ch.is_whitespace() {
                result.push(text_ch);
            } else {
                let text_index = str::find(self.text_alphabet, text_ch).unwrap();
                let key_ch = key.chars().nth(index % key_len).unwrap();
                let key_index = str::find(self.key_alphabet, key_ch).unwrap();
                let combined_index = transform(text_index, key_index, text_alphabet_len);
                result.push(self.text_alphabet.chars().nth(combined_index).unwrap());
            }
            index += 1;
        }
        return result;
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

    #[test]
    fn encode_many() {
        let text = "abc";
        assert_eq!("bca", DEFAULT_CIPHER.encode(text, "d"));
        assert_eq!("baa", DEFAULT_CIPHER.encode(text, "de"));
        assert_eq!("bac", DEFAULT_CIPHER.encode(text, "def"));
    }

    #[test]
    fn encode_ignore_spaces() {
        assert_eq!("bca  bca bca", DEFAULT_CIPHER.encode("abc  abc abc", "d"))
    }

    #[test]
    fn encode_decode() {
        let text_cases = ["a", "b", "bbb", "bababac", "cacacbbbaabb"];
        let keys = ["d", "dd", "feed", "def", "fdedededfffedd"];
        for text in text_cases {
            for key in keys {
                let encoded = DEFAULT_CIPHER.encode(text, key);
                let decoded = DEFAULT_CIPHER.decode(&encoded, key);
                assert_eq!(text, decoded);
            }
        }
    }

}