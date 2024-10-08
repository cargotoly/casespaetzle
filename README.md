# Crate: Identifier Cases

This crate implements a utility trait that converts any string to a particular case style. The case names are majorly named after the answer to the StackOverflow question [What are the different kinds of cases?](https://stackoverflow.com/questions/17326185/what-are-the-different-kinds-of-cases). An identifier is a string consisting of alphanumeric characters and a few separation markers, like the dash and the underscore, with a `word` being defined as a lexicographical atomic component of the identifier.

| Case Name | Example | Definition
|-|-|-|
| Split case | `["Hello", "World!"]` | Splits the different parts of an identifier into a vector of strings.
| Flat case | `helloworld` | Lowercase concatenation of split case.
| Kebab case | `hello-world` | Lowercase join with dash of split case.
| Camel case | `helloWorld` | Concatenation of lowercase words with the first letter capitalized. The first letter of the identifier is not capitalized.
| Pascal case | `HelloWorld` | Concatenation of lowercase words with the first letter capitalized.
| Snake case | `hello_world` | Lowercase join with underscore of split case.
| Constant case | `HELLO_WORLD` | Lowercase join with underscore of split case.

Specific sbbreviations may be converted unintuitively into the cases, like `OCaml` converting to `o-caml` and `OCaml` in dash and pascal case, but `Ocaml` converts to `ocaml`. In camel case, the first one converts to `oCaml`.

### Open for Contribution

- `TRAIN-CASE`, `COBOL-CASE`
- `HTTP-Header-Case` a case insensitive version of train case, which keeps uppercase words.
- `_undercoreNotation` (underscore and camel case)
- `HTTPRequest` a case insensitive version of camel case, which keeps uppercase words.
