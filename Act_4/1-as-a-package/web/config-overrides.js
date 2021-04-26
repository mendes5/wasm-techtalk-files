const path = require('path');

const wasmExtensionRegExp = /\.wasm$/;

module.exports = config => {
    config.resolve.extensions.push('.wasm');

    config.module.rules.forEach(rule => {
        (rule.oneOf || []).forEach(oneOf => {
            if ((oneOf.loader || []).indexOf('file-loader') >= 0)
                oneOf.exclude.push(wasmExtensionRegExp);
        });
    });

    config.module.rules.push({
        test: wasmExtensionRegExp,
        include: path.resolve(__dirname, 'src'),
        use: [{ loader: require.resolve('wasm-loader') }]
    });

    return config;
};