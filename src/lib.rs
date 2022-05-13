pub struct AlphaCounter {
    number: usize,
    alphabet: Vec<char>,
    length: usize,
}

impl AlphaCounter {
    pub fn lower(number: usize) -> Self {
        Self::custom(number, "abcdefghijklmnopqrstuvwxyz")
    }

    pub fn upper(number: usize) -> Self {
        Self::custom(number, "ABCDEFGHIJKLMNOPQRSTUVWXYZ")
    }

    pub fn custom(number: usize, alphabet: &str) -> Self {
        let alphabet: Vec<char> = alphabet.chars().collect();
        let length = alphabet.len();
        Self {
            number,
            alphabet,
            length,
        }
    }

    pub fn to_string(&mut self) -> String {
        self.recurse(self.number)
    }

    pub fn recurse(&mut self, x: usize) -> String {
        let d = x / self.length;
        let r = x % self.length;
        let t = self.alphabet[r];
        if d == 0 {
            format!("{t}")
        } else {
            format!("{}{t}", self.recurse(d - 1))
        }
    }
}

impl core::iter::Iterator for AlphaCounter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.to_string();
        self.number += 1;
        Some(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lower() {
        assert_eq!(
            AlphaCounter::lower(0).take(30).collect::<Vec<String>>(),
            vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p",
            "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "aa", "ab", "ac", "ad"],
        );
    }

    #[test]
    fn upper() {
        assert_eq!(
            AlphaCounter::upper(0).take(30).collect::<Vec<String>>(),
            vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P",
            "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "AA", "AB", "AC", "AD"],
        );
    }

    #[test]
    fn custom() {
        assert_eq!(
            AlphaCounter::custom(0, "abc").take(30).collect::<Vec<String>>(),
            vec!["a", "b", "c", "aa", "ab", "ac", "ba", "bb", "bc", "ca", "cb", "cc", "aaa", "aab",
            "aac", "aba", "abb", "abc", "aca", "acb", "acc", "baa", "bab", "bac", "bba", "bbb",
            "bbc", "bca", "bcb", "bcc"],
        );
    }
}
