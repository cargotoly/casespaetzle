import test from 'node:test'
import assert from 'node:assert'
import './index.js'

test('case conversions', (t) => {
    assert.strictEqual('hello world'.to_constant_case(), 'HELLO_WORLD');
    assert.strictEqual('hello world'.to_camel_case(), 'helloWorld');
});
