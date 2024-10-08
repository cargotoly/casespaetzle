# Casespaetzle

Extends the string prototype with case conversion methods, which are written in Rust.

```ts
import 'casespaetzle'
import assert from 'node:assert'

assert.strictEqual('http request'.to_pascal_case(), 'HttpRequest')
console.log('http request'.to_pascal_case())
```

If you want to contribute, refer to the [casespaetzle](https://github.com/Anatoly03/casespaetzle/tree/master/casespaetzle) module.
