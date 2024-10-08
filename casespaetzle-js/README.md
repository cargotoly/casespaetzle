# Casespaetzle

Extends the string prototype with case conversion methods, which are written in Rust.

```ts
import 'casespaetzle'
import assert from 'node:assert'

console.log('hello world'.toConstantCase()) // HELLO_WORLD
assert.strictEqual('http request'.toPascalCase(), 'HttpRequest')
```

If you want to contribute, refer to the [casespaetzle](https://github.com/Anatoly03/casespaetzle/tree/master/casespaetzle) module.
