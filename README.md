# Casespaetzle

[NPM](https://npmjs.com/package/casespaetzle) | [Cargo](https://crates.io/crates/casespaetzle)

| Trait | Example | Definition
|-|-|-|
| SplitCase | `["Hello", "World!"]` | Splits the different parts of an identifier into a vector of strings.
| FlatCase | `helloworld` | Lowercase concatenation. The resulting identifier loses all information on word case and separations.
| KebabCase | `hello-world` | Lowercase join with dash.
| HttpHeaderCase | `Hello-World`, `HTTP-Header-Case` | Join with dash. The first letter of every word is capitalized. This case is case sensitive. Uppercase sequences of letters will be preserved.
| CamelCase | `helloWorld` | Concatenation of lowercase words with the first letter capitalized. The first letter of the identifier is not capitalized.
| PascalCase | `HelloWorld` | Concatenation of lowercase words with the first letter capitalized.
| SnakeCase | `hello_world` | Lowercase join with underscore.
| ConstantCase | `HELLO_WORLD` | Uppercase join with underscore.

This workspace consists of two member crates: [casespaetzle-macro](https://github.com/cargotoly/casespaetzle/tree/master/casespaetzle-macro) provides the macro function `add_case!()`, and [casespaetzle](https://github.com/cargotoly/casespaetzle/tree/master/casespaetzle), the main library endpoint.

The directory [casespaetzle-js](https://github.com/cargotoly/casespaetzle/tree/master/casespaetzle-js) contains bindings that publish the package [to NPM](https://www.npmjs.com/package/casespaetzle) from the Rust-generated web assembly.

### The Algorithm

```js
wonderful-HTTPRequestIdentifier-HELLO
// → wonderful; HTTP; Request; Identifier; HELLO
```

To define a case we first define the term 'word', which is an atomic sequence of characters contained within the identifier. A word is any identifier, which cannot be split into smaller words. The split algorithm works with consideration of three rules.

- There is a set of 'separation characters', which split any identifier with highest priority. These characters will separate words but never be part of one. Currently this set is `-_~,.` and the space character. `a-b → a ; b`
- A capital letter following a lowercase letter, belongs to the following word. `ABc → A ; Bc`
- All lowercase letters, must be part of the same word.

There are three different types of cases, that hold a variable amount of information regarding the words. Cases with full information can be converted into one another without information loss.

- **Full Information Preservation**: All case types, that join words with separation characters without affecting the case of individual letters, can still be used case sensitive, which means, they can be converted into any case, just like the original identifier.
- **Little Information Preservation**: All case types, that join words with letter capitalization, loses information concatenating two uppercase words, or in case words are preserved, their case is lost. `ABC-DEF` would become either `ABCDEF` or `AbcDef`. The raw structure of words can be maintained in a few cases, but some words lose information on separation or case.
- **No Information Preservation**: The only case of this type is `flatcase`, which loses all information on the original identifier.

The identifiers `OCaml` and `HTTPRequest` will be intuitively converted in the words `O`, `Caml` and `HTTP`, `Request` respectively. If any case with full or little information preservation is used, the individual words will be preserved, however with little information preservation, the words lose its' case sensivity.

### Open for Contribution

```
cargo test
```

- `TRAIN-CASE`, `COBOL-CASE`
- `_undercoreNotation` (underscore and camel case)
- `HTTPRequest` a case insensitive version of camel case, which keeps uppercase words.
