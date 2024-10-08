//! This crate implements a utility trait that converts any string to a
//! particular case style. The case names are majorly named after the answer
//! to the StackOverflow question [What are the different kinds of cases?]
//! (https://stackoverflow.com/questions/17326185/what-are-the-different-kinds-of-cases).
//! An identifier is a string consisting of alphanumeric characters and a few
//! separation markers, like the dash and the underscore, with a `word` being
//! defined as a lexicographical atomic component of the identifier.

#![deny(rust_2018_idioms)]

mod cases;
mod custom;

pub use cases::CaseStyles;
pub use cases_macro::add_case;
pub use custom::*;

/// Contains the characters that case insensitive separate identifier words.
pub const SEPARATION_CHARACTERS: &str = &"-_~,. ";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_case() {
        assert_eq!(
            "HelloWorld".to_split_case(),
            vec!["Hello".to_string(), "World".to_string()]
        );
        assert_eq!(
            "helloWorld".to_split_case(),
            vec!["hello".to_string(), "World".to_string()]
        );
        assert_eq!(
            "__helloWorld".to_split_case(),
            vec!["hello".to_string(), "World".to_string()]
        );
        assert_eq!(
            "AbcABC".to_split_case(),
            vec!["Abc".to_string(), "ABC".to_string()]
        );
        assert_eq!(
            "ABCAbc".to_split_case(),
            vec!["ABC".to_string(), "Abc".to_string()]
        );
    }

    #[test]
    fn camel_case() {
        assert_eq!("HelloWorld".to_camel_case(), "helloWorld");
        assert_eq!("HTTPRequest".to_camel_case(), "httpRequest");
    }

    #[test]
    fn constant_case() {
        assert_eq!("HelloWorld".to_constant_case(), "HELLO_WORLD");
    }

    #[test]
    fn flat_case() {
        assert_eq!("HelloWorld".to_flat_case(), "helloworld");
        assert!("flatcase".is_strict_flat_case());
        assert!(!"PascalCase".is_strict_flat_case());
    }

    #[test]
    fn kebab_case() {
        assert_eq!("HelloWorld".to_kebab_case(), "hello-world");
    }

    #[test]
    fn pascal_case() {
        assert_eq!("HelloWorld".to_pascal_case(), "HelloWorld");
        assert_eq!("HTTPRequest".to_pascal_case(), "HttpRequest");
        assert_eq!("HTTP-Request".to_pascal_case(), "HttpRequest");
        assert!("PascalCase".is_strict_pascal_case());
    }

    #[test]
    fn snake_case() {
        assert_eq!("HelloWorld".to_snake_case(), "hello_world");
    }
}
