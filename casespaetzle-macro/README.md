# Casespaetzle Macro

This module is used by the [casespaetzle](https://github.com/Anatoly03/casespaetzle/tree/master/casespaetzle) crate. This module only exports the macro `add_case!()`. In this macro you provide a function which constructs a trait for a specific case variant. The trait name is in pascal case, while the provided function has to be in strict snake case.

```rs
use util_cases::{SplitCase, add_case};

add_case! {
    /// The joke case (`jOkE cAsE`) conversion documentation.
    fn joke_case(&self) -> String {
        self.to_split_case()
            .into_iter()
            .map(|s| s.char_indices()
                .map(|(idx, c)| {
                    if idx & 1 == 0 { c.to_ascii_lowercase() }
                    else { c.to_ascii_uppercase() }
                })
                .collect::<Vec<char>>()
                .into_iter()
                .collect()
            )
            .collect::<Vec<String>>()
            .join(" ")
    }
}

pub use JokeCase;

assert_eq!("Hello World".to_joke_case(), "hElLo wOrLd");
```