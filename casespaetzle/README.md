# Casespaetzle

This crate implements a utility trait that converts any string to a case. The case names are majorly named after the answers to the StackOverflow question [What are the different kinds of cases?](https://stackoverflow.com/questions/17326185/what-are-the-different-kinds-of-cases). An identifier is a string consisting of (mostly) alphanumeric characters and a few separation markers, like the dash and the underscore, with a `word` being defined as a lexicographical atomic component of the identifier.

| Trait | Example | Definition
|-|-|-|
| SplitCase | `["Hello", "World!"]` | Splits the different parts of an identifier into a vector of strings.
| FlatCase | `helloworld` | Lowercase concatenation of split case.
| KebabCase | `hello-world` | Lowercase join with dash of split case.
| CamelCase | `helloWorld` | Concatenation of lowercase words with the first letter capitalized. The first letter of the identifier is not capitalized.
| PascalCase | `HelloWorld` | Concatenation of lowercase words with the first letter capitalized.
| SnakeCase | `hello_world` | Lowercase join with underscore of split case.
| ConstantCase | `HELLO_WORLD` | Lowercase join with underscore of split case.

Camel and pascal case abbreviations may be converted unintuitively into the cases, like `OCaml` converting to `o-caml` and `OCaml` in dash and pascal case, but `Ocaml` converts to `ocaml`. In camel case, the first one converts to `oCaml`.

This package re-exports the macro `add_case`, for its' documentation, refer [to the crate](https://github.com/Anatoly03/casespaetzle/tree/master/casespaetzle-macro) on github.

### Open for Contribution

- `TRAIN-CASE`, `COBOL-CASE`
- `HTTP-Header-Case` a case insensitive version of train case, which keeps uppercase words.
- `_undercoreNotation` (underscore and camel case)
- `HTTPRequest` a case insensitive version of camel case, which keeps uppercase words.
