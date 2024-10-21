# Casespaetzle

Extends the string prototype with case conversion methods, which are written in Rust. For a list of supported cases, see the [main README](https://github.com/cargotoly/casespaetzle).

```ts
import 'casespaetzle'
import assert from 'node:assert'

console.log('hello world'.toConstantCase()) // HELLO_WORLD
assert.strictEqual('http request'.toPascalCase(), 'HttpRequest')
```

If you want to contribute, refer to the [casespaetzle](https://github.com/cargotoly/casespaetzle/tree/master/casespaetzle) Rust module.
