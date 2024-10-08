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

    let ident = input.sig.ident; // func_name
    let ident_pascal = format_ident!("{}", snake_to_pascal(ident.to_string())); // TraitName
    let ident_js = format_ident!("to{}", ident_pascal.to_string()); // toTraitName
    let ident_to = format_ident!("to_{}", ident.to_string()); // to_trait_name
    let ident_to_str = format!("{ident_to}"); // to_trait_name
    let ident_is_strict = format_ident!("is_strict_{}", ident.to_string()); // to_trait_name
    let ident_is_strict_str = format!("{ident_is_strict}"); // to_trait_name
    // TODO consider an algorithm that trims and removes consecutive underscores.
    let trait_name_human = ident.to_string().replace("_", " ").to_string(); // trait name

    // The output function is prefixed with `to_`
    input.sig.ident = ident_to.clone();

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
        pub trait #ident_pascal : SplitCase {
            #docs
            ///
            /// The method
            #[doc = #ident_to_str]
            /// will return a string in
            #[doc = #trait_name_human]
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
            #[doc = #ident_is_strict_str]
            /// will return true for every identifier in
            #[doc = #trait_name_human]
            /// , if the construction function
            #[doc = #ident_to_str]
            /// matches case sensitive on the identifier.
            ///
            /// ```rs
            /// use casespaetzle::*;
            ///
            /// assert!("helloWorld".is_strict_camel_case());
            /// assert!("HttpRequest".is_strict_pascal_case());
            /// assert!(!"hello world".is_strict_flat_case());
            /// ```
            fn #ident_is_strict (&self) -> bool {
                &self.to_split_case().join("") == &self.#ident_to()
            }
        }

        impl<T: SplitCase> #ident_pascal for T {}

        #[cfg(target_arch = "wasm32")]
        #[wasm_bindgen]
        pub fn #ident_js(value: String) -> String {
            value.#ident_to()
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
