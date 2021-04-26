# Create the library
```
cargo generate --git https://github.com/rustwasm/wasm-pack-template -n my-lib
cd my-lib
wasm-pack build
```

# Create the frontend

```
create-react-app web
cd web
yarn start
yarn add -D react-app-rewired wasm-loader
```

In `config-overrides.js`:

```js
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
```

In `package.json`

```js
  "scripts": {
    "start": "react-app-rewired start"
  },
```

Also in `package.json` in `"dependencies"` field:

```json
    "my-lib": "../my-lib/pkg",
```

In any JS file:

```js
import('my-lib')
    .then((module) => module.greet())
    .catch(console.error);
```
