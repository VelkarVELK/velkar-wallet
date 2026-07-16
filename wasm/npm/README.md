# Velkar WASM SDK

An integration wrapper around [`velkar-wasm`](https://www.npmjs.com/package/velkar-wasm) module that uses [`websocket`](https://www.npmjs.com/package/websocket) W3C adaptor for WebSocket communication.

This is a Node.js module that provides bindings to the Velkar WASM SDK strictly for use in the Node.js environment. The web browser version of the SDK is available as part of official SDK releases at [https://github.com/velkarnet/velkar/releases](https://github.com/velkarnet/velkar/releases)

## Usage

Velkar NPM module exports include all WASM32 bindings.
```javascript
const velkar = require('velkar');
console.log(velkar.version());
```

## Documentation

Documentation is available at [https://velkar.aspectron.org/docs/](https://velkar.aspectron.org/docs/)


## Building from source & Examples

SDK examples as well as information on building the project from source can be found at [https://github.com/velkarnet/velkar/tree/master/wasm](https://github.com/velkarnet/velkar/tree/master/wasm)

## Releases

Official releases as well as releases for Web Browsers are available at [https://github.com/velkarnet/velkar/releases](https://github.com/velkarnet/velkar/releases).

Nightly / developer builds are available at: [https://aspectron.org/en/projects/velkar-wasm.html](https://aspectron.org/en/projects/velkar-wasm.html)

