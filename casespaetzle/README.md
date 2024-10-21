# Casespaetzle

This crate implements a utility trait that converts any string to a case. The case names are majorly named after the answers to the StackOverflow question [What are the different kinds of cases?](https://stackoverflow.com/questions/17326185/what-are-the-different-kinds-of-cases). An identifier is a string consisting of (mostly) alphanumeric characters and a few separation markers, like the dash and the underscore, with a `word` being defined as a lexicographical atomic component of the identifier.

For a list of supported cases, see the [main README](https://github.com/cargotoly/casespaetzle).

This package re-exports the macro `add_case`, for its' documentation, refer [to the crate](https://github.com/cargotoly/casespaetzle/tree/master/casespaetzle-macro) on github.
