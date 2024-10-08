import test from 'node:test'
import assert from 'node:assert'
import './index.js'

test('case conversions', (t) => {
    assert.strictEqual('hello world'.toConstantCase(), 'HELLO_WORLD');
    assert.strictEqual('hello world'.toCamelCase(), 'helloWorld');
});
