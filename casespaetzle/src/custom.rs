use crate::{add_case, SplitCase};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Capitalize the first letter of an identifier.
pub fn capitalize(mut out: String) -> String {
    out[0..1].make_ascii_uppercase();
    out
}

/// Turn the first letter of an identifier lowercase.
pub fn decapitalize(mut out: String) -> String {
    out[0..1].make_ascii_lowercase();
    out
}

add_case! {
    /// The flat case (`flatcase`) conversion concatenates the
    /// words of an identifier into lowercase letters without
    /// separators.
    ///
    /// An identifier is in flatcase, if it is lowercase and
    /// consists of one word.
    fn flat_case(&self) -> String {
        self.to_split_case().join("").to_lowercase()
    }
}

add_case! {
    /// The kebab case or dash case (`dash-case`) conversion
    /// joins the words of an identifier into lowercase letters
    /// with a dash.
    ///
    /// An identifier is in kebab case, if it is lowercase and
    /// the words are separated by one dash.
    fn kebab_case(&self) -> String {
        self.to_split_case().join("-").to_lowercase()
    }
}

add_case! {
    /// The snake case (`snake_case`) conversion joins the words
    /// of an identifier with the underscore. The resulting word
    /// is lowercase.
    ///
    /// An identifier is in snake case, if it is lowercase and
    /// the words are separated by one dash.
    fn snake_case(&self) -> String {
        self.to_split_case().join("_").to_lowercase()
    }
}

add_case! {
    /// The pascal case or capital camel case (`PascalCase`) conversion
    /// joins the words of an identifier without separation characters.
    /// Each word is capitalized.
    ///
    /// An identifier is in pascal case, if the first letter is
    /// in capital case and there are no separation symbols.
    fn pascal_case(&self) -> String {
        self.to_split_case()
            .iter()
            .map(|s| s.to_lowercase().clone())
            .map(capitalize)
            .collect::<Vec<String>>()
            .join("")
    }
}

add_case! {
    /// The camel case (`camelCase`) conversion joins the words
    /// of an identifier without separation characters. Each,
    /// except the first word, will be capitalized.
    ///
    /// An identifier is in camel case, if the first letter is
    /// lower case and there are no separation symbols.
    fn camel_case(&self) -> String {
        let s = self.to_split_case()
            .iter()
            .map(|s| s.to_lowercase().clone())
            .map(capitalize)
            .collect::<Vec<String>>()
            .join("");

        decapitalize(s)
    }
}

add_case! {
    /// The constant case (`UPPER_CASE`) conversion joins the words
    /// of an identifier with the underscore. The resulting word
    /// is uppercase.
    ///
    /// An identifier is in constant case, if it is uppercase and
    /// the words are separated by one underscore.
    fn constant_case(&self) -> String {
        self.to_split_case().join("_").to_uppercase()
    }
}
