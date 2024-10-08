use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn, Meta};

/// This is a minimal snake case to pascal case converter.
fn snake_to_pascal<T: AsRef<str>>(v: T) -> String {
    let mut chars = v.as_ref().chars();

    let mut st = String::new();
    let mut preceeds_char = false;

    while let Some(c) = chars.next() {
        match (c, preceeds_char) {
            ('_', true) => match chars.next() {
                Some('_') => {
                    st += c.to_string().as_ref();
                    preceeds_char = false;
                }
                Some(cc) => st += cc.to_ascii_uppercase().to_string().as_ref(),
                _ => st += c.to_string().as_ref(),
            },
            (_, false) if c != '_' => {
                st += c.to_ascii_uppercase().to_string().as_ref();
                preceeds_char = true;
            }
            _ => st += c.to_string().as_ref(),
        }
    }

    st
}

/// The case generator macro expects a documented trait method
/// and generates case conversions and assertion methods.
///
/// ### Usage
///
/// ```rs
/// use casespaetzle::{SplitCase, add_case};
///
/// add_case! {
///     /// The joke case (`jOkE cAsE`) conversion documentation.
///     fn joke_case(&self) -> String {
///         self.to_split_case()
///             .into_iter()
///             .map(|s| s.char_indices()
///                 .map(|(idx, c)| {
///                     if idx & 1 == 0 { c.to_ascii_lowercase() }
///                     else { c.to_ascii_uppercase() }
///                 })
///                 .collect::<Vec<char>>()
///                 .into_iter()
///                 .collect()
///             )
///             .collect::<Vec<String>>()
///             .join(" ")
///     }
/// }
///
/// pub use JokeCase;
///
/// assert_eq!("Hello World".to_joke_case(), "hElLo wOrLd");
/// ```
#[proc_macro]
pub fn add_case(item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);

    let ident = input.sig.ident;

    // The trait name will be in pascal case.
    let trait_name = format_ident!("{}", snake_to_pascal(ident.to_string()));

    // The string-like trait name should be human readable though.
    // TODO consider an algorithm that trims and removes consecutive underscores.
    let trait_name_str = ident.to_string().replace("_", " ").to_string();

    // The conversion method name follows the prefix `to_`
    let convert_method_name = format_ident!("to_{}", ident);
    let convert_method_name_str = format!("`{convert_method_name}`");

    // The verification method name follows the prefix `is_strict_`
    let check_method_name = format_ident!("is_strict_{}", ident);
    let check_method_name_str = format!("`{check_method_name}`");

    input.sig.ident = convert_method_name.clone();

    let docs = input
        .attrs
        .iter()
        .filter(|attr| matches!(&attr.meta, Meta::NameValue(mnv) if mnv.path.is_ident("doc")))
        .map(|attr| quote!(#attr))
        .fold(proc_macro2::TokenStream::new(), |mut acc, f| {
            acc.extend(f);
            acc
        });

    quote! {
        pub trait #trait_name : SplitCase {
            #docs
            ///
            /// The method
            #[doc = #convert_method_name_str]
            /// will return a string in
            #[doc = #trait_name_str]
            /// according to the definition of the case construction.
            ///
            /// ```rs
            /// use casespaetzle::*;
            ///
            /// assert_eq!("Hello World".to_camel_case(), "helloWorld");
            /// assert_eq!("Hello-World".to_pascal_case(), "HelloWorld");
            /// ```
            #input

            #docs
            ///
            /// The method
            #[doc = #check_method_name_str]
            /// will return true for every identifier in
            #[doc = #trait_name_str]
            /// , if the construction function
            #[doc = #convert_method_name_str]
            /// matches case sensitive on the identifier.
            ///
            /// ```rs
            /// use casespaetzle::*;
            ///
            /// assert!("helloWorld".is_strict_camel_case());
            /// assert!("HttpRequest".is_strict_pascal_case());
            /// assert!(!"hello world".is_strict_flat_case());
            /// ```
            fn #check_method_name (&self) -> bool {
                &self.to_split_case().join("") == &self.#convert_method_name()
            }
        }

        impl<T: SplitCase> #trait_name for T {}

        #[cfg(target_arch = "wasm32")]
        #[wasm_bindgen]
        pub fn #convert_method_name(value: String) -> String {
            value.#convert_method_name()
        }
    }
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn case_convert() {
        assert_eq!(snake_to_pascal("_hello"), "_Hello".to_owned());
        assert_eq!(snake_to_pascal("hello_world"), "HelloWorld".to_owned());
        assert_eq!(snake_to_pascal("___abc__def"), "___Abc_Def".to_owned());
    }
}
