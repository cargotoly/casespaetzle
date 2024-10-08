import { readFileSync, writeFileSync } from 'fs'

// Generate Type Definition File

const types = readFileSync('./pkg/casespaetzle.d.ts')
    .toString()
    .split('\n')
    .map(line => {
        if (line.startsWith('export function'))
            return '\t\t' + line.substring(16)
        else
            return '\t\t' + line
    })
    .join('\n')

writeFileSync('index.d.ts', `export {};

declare global {
\tinterface String {
${types}
\t}
}
`)

// Extend pkg JSON

// const pkg = JSON.parse(readFileSync('./pkg/package.json'))
// pkg['type'] = 'module'
// writeFileSync('./pkg/package.json', JSON.stringify(pkg, null, 2))
