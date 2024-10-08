# Casespaetzle

```
cargo test
```

This workspace consists of two member crates: [casespaetzle-macro](https://github.com/Anatoly03/casespaetzle/tree/master/casespaetzle-macro) provides the macro function `add_case!()`, and [casespaetzle](https://github.com/Anatoly03/casespaetzle/tree/master/casespaetzle), the main library endpoint.

The directory [casespaetzle-js](https://github.com/Anatoly03/casespaetzle/tree/master/casespaetzle-js) contains bindings that publish the package [to NPM](https://www.npmjs.com/package/casespaetzle) from the Rust-generated web assembly.
