#[macro_use]
extern crate lazy_static;

use std::collections::HashSet;

lazy_static! {
    static ref VOWELS: HashSet<&'static str> = {
        let mut hs = HashSet::new();
        hs.insert("yt");
        hs.insert("xr");
        hs.insert("a");
        hs.insert("e");
        hs.insert("i");
        hs.insert("o");
        hs.insert("u");
        hs
    };

    static ref VOWEL_LONG: usize = VOWELS.iter().map(|&s| s.len()).max().unwrap();
    static ref VOWEL_SHORT: usize = VOWELS.iter().map(|&s| s.len()).min().unwrap();

    static ref PHONEME_CLUSTERS: HashSet<&'static str> = {
        let mut hs = HashSet::new();
        hs.insert("thr");
        hs.insert("sch");
        hs.insert("ch");
        hs.insert("qu");
        hs.insert("th");
        hs
    };

    static ref PHONEME_LONG: usize = PHONEME_CLUSTERS.iter().map(|&s| s.len()).max().unwrap();
    static ref PHONEME_SHORT: usize = PHONEME_CLUSTERS.iter().map(|&s| s.len()).min().unwrap();
}


macro_rules! starts_with_x {
    ( $func_id:ident, $hs:ident, $short:ident, $long:ident ) => {
        fn $func_id(word: &str) -> (bool, Option<usize>) {
            (
                word.len() >= *$short &&
                    {
                        let max_cut = ::std::cmp::min(word.len(), *$long);
                        for chars_l in (*$short..(max_cut + 1)).rev() {
                            if $hs.contains(&word[..chars_l]) {
                                return (true, Some(chars_l));
                            }
                        }
                        false
                    },
                None,
            )
        }
    };
}

starts_with_x!(starts_with_vowel, VOWELS, VOWEL_SHORT, VOWEL_LONG);
starts_with_x!(
    starts_with_phoneme,
    PHONEME_CLUSTERS,
    PHONEME_SHORT,
    PHONEME_LONG
);

pub fn translate_word(word: &str) -> String {
    let mut output = String::with_capacity(word.len() + 2);
    let (sv, _) = starts_with_vowel(word);
    if sv {
        output.push_str(word);
        output.push_str("ay");
        return output;
    }
    let (sp, n) = starts_with_phoneme(word);
    if sp {
        // n is always some if sp
        let n = n.unwrap();
        output.push_str(&word[n..]);
        output.push_str(&word[..n]);
        output.push_str("ay");
        return output;
    }
    // special rule: 'qu' following consonant
    if word[1..3] == *"qu" {
        // we already know it doesn't start with a vowel, or we would have already returned
        output.push_str(&word[3..]);
        output.push_str(&word[..3]);
        output.push_str("ay");
        return output;
    }
    output.push_str(&word[1..]);
    output.push_str(&word[..1]);
    output.push_str("ay");
    output
}

pub fn translate(sentence: &str) -> String {
    let mut output = String::with_capacity(sentence.len() * 2);
    for word in sentence.split_whitespace() {
        output.push_str(&translate_word(word));
        output.push(' ');
    }
    output.trim().to_string()
}
