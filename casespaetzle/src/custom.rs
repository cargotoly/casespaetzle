use crate::{add_case, SplitCase};

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
        let capitalize = |s: &String| {
            let mut out = s.to_lowercase().clone();
            out[0..1].make_ascii_uppercase();
            out
        };

        (&self.to_split_case())
            .into_iter()
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
        let capitalize = |s: &String| {
            let mut out = s.to_lowercase().clone();
            out[0..1].make_ascii_uppercase();
            out
        };

        let mut s = (&self.to_split_case())
            .into_iter()
            .map(capitalize)
            .collect::<Vec<String>>()
            .join("");

        s[0..1].make_ascii_lowercase();
        s
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
