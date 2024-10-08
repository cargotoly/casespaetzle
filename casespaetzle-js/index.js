const wasm = await import("./pkg/casespaetzle.js")

for (const key of Object.keys(wasm).filter(s => s != '__wasm' && s != 'default')) {
    String.prototype[key] = function () {
        return wasm[key](this)
    }
}

export { }
