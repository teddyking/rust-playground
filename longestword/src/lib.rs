pub fn longest_word(sen: &str) -> String {
    let words = sen.split_whitespace();
    let mut longest = String::new();

    for word in words {
        let stripped_word = strip_non_alpha_numeric(word);
        if stripped_word.to_string().len() > longest.len() {
            longest = stripped_word.to_string();
        }
    }

    return longest;
}

fn strip_non_alpha_numeric(sen: &str) -> String {
    let mut stripped_string = String::new();

    for c in sen.chars() {
        if c.is_alphabetic() {
            stripped_string.push(c)
        }
    }

    stripped_string
}

#[cfg(test)]
mod tests {
    use super::*;

    mod longest_word {
        use super::*;

        #[test]
        fn with_a_simple_sentence() {
            let sen = "The cake is a lie";

            assert_eq!(longest_word(sen), "cake");
        }

        #[test]
        fn with_a_word_with_punctuation() {
            let sen = "The cake is a lie!!";

            assert_eq!(longest_word(sen), "cake");
        }

        #[test]
        fn with_two_longest_words() {
            let sen = "I love dogs!";

            assert_eq!(longest_word(sen), "love");
        }
    }

    mod strip_non_alpha_numeric {
        use super::*;

        #[test]
        fn strips_non_alpha_numeric_chars() {
            let sen = "LOL!";

            assert_eq!(strip_non_alpha_numeric(sen), "LOL");
        }
    }
}
