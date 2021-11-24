const configuration = {
    extensions: ['ts', 'tsx'],
    files: ['tests/**/*.spec.ts'],
    require: ['ts-node/register/transpile-only'],
    timeout: '1m',
}

if (parseInt(process.versions.napi, 10) < 4) {
    configuration.compileEnhancements = false
}

export default configuration