# Casespaetzle Macro

> **DO NOT USE THIS CRATE DIRECTLY.** Instead, use [casespaetzle](https://crates.io/crates/casespaetzle). This crate only servers as a procedural macro library for its' parent crate and is not meant for direct use. If you would wish to use this crate raw, see below.

This module is used by the [casespaetzle](https://github.com/cargotoly/casespaetzle/tree/master/casespaetzle) crate. This module only exports the macro `add_case!()`. In this macro you provide a function which constructs a trait for a specific case variant. The trait name is in pascal case, while the provided function has to be in strict snake case.

```rs
use casespaetzle::{SplitCase, add_case};

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

### Raw Usage

If you insist on using this crate raw (without the parent crate `casespaetzle`), you will have to implement the following trait in your code:

```rs
pub trait SplitCase {
    fn to_split_case(&self) -> Vec<String>;
}
```

This is the trait acts as a 'middleware' between stdlib and primitive type implementations and all cases added with the macro. The crate [casespaetzle](https://github.com/Anatoly03/casespaetzle/tree/master/casespaetzle) expands it with default cases and implements this trait for string types.
