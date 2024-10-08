//! This module contains the CaseStyle trait, which acts as a middle-trait
//! between custom case implementations and primitive types.

use crate::SEPARATION_CHARACTERS;

/// Defines methods to convert an identifier into various case styles
/// based on the implemented word splitter method.
pub trait SplitCase {
    /// Splits an identifier into atomic words. A word can be either uppercase (`ABC`)
    /// capitalized (`Abc`), or lowercase (`abc`). This method will panic if it has to
    /// work with case insensitive characters other than a few separation markers.
    fn to_split_case(&self) -> Vec<String>;
}

/// Implements for `SplitCase` for the [common trait](https://www.reddit.com/r/rust/comments/zfgo1f/common_trait_for_str_string_string_arcstring/)
/// which all string types share.
impl<T: AsRef<str>> SplitCase for T {
    fn to_split_case(&self) -> Vec<String> {
        let identifier = self.as_ref().to_string();
        let separation = identifier
            .split(|c| SEPARATION_CHARACTERS.contains(c))
            .map(ToString::to_string)
            .collect::<Vec<String>>();
        let mut vec = vec![];

        for element in separation {
            let mut buffer = String::from("");

            // The separation characters split the word by "obvious" splits, this
            // will split any input in `dash-case` and `snake_case` into two words,
            // but the individual word components have to be split again based on
            // their case. The following needs to be considered:
            //
            // - `Dashed-SnakeCase` equives `dashed-snake-case`
            // - `HTTPRequest` equives `http-request`
            // - `ECMAScript` equives `ecma-script`
            // - `camelsLoveOCaml` equives `camels-love-o-caml`,
            //   ambigious with semantic `..-ocaml`
            //
            // From these observations, here are the rules of the algorithm
            // ranked by priority:
            // 1. All lower case letters following one lower case letter must be part
            //    of the same word.
            // 2. A capital letter following lower case letters belongs to the following
            //    word.

            for (idx, c) in element.char_indices() {
                const DEFAULT: char = '?';

                let previous_letter = buffer.chars().last().unwrap_or(DEFAULT);
                let next_letter = element.chars().nth(idx + 1).unwrap_or(DEFAULT);

                match c {
                    // If we're an uppercase letter and the next letter is lowercase, we start
                    // a new word from this letter.
                    // This covers `...Aa` and `aA...`
                    c if c.is_ascii_uppercase() && (previous_letter.is_ascii_lowercase() || next_letter.is_ascii_lowercase()) => {
                        vec.push(buffer);
                        buffer = c.to_string();
                    }
                    // If we're an uppercase letter and the next letter keeps case, continue
                    // writing to buffer.
                    // This covers `...AA`
                    c if c.is_ascii_uppercase() /*&& next_letter.is_ascii_uppercase()*/ => {
                        buffer += c.to_string().as_str();
                    }
                    // We land here if we are in a series of lowercase letters. In this case,
                    // we keep writing into the buffer
                    // This covers `...aa`
                    c if c.is_ascii_lowercase() => {
                        buffer += c.to_string().as_str();
                    }
                    // NOTE: The case `...aA` does not need coverage.
                    // If the letter has no case, panic.
                    _ => panic!(
                        "Identifier expected to consist of upper or lowercase letters, got `...{}{}{}...`",
                        previous_letter, c, next_letter
                    ),
                }
            }

            if !buffer.is_empty() {
                vec.push(buffer);
            }
        }

        vec.into_iter().filter(|s| !s.is_empty()).collect()
    }
}

// TODO: implement (split case should return self): impl<'a, T: AsRef<&'a str>, U: Iter<T>> SplitCase for U
