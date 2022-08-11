# wasm-mazes

Generate mazes with WebAssembly directly in your browser to pdf files for easy
printing, no server needed.

Site is online [here](http://mauri.codes/wasm-mazes/).

## How to run in debug mode

Install [Rust](https://www.rust-lang.org/tools/install),
[wasmpack](https://github.com/rustwasm/wasm-pack) and
[Node.js](https://nodejs.org/en/) with npm.

Then following command will build the project and open it in a new browser tab.
Auto-reloads when the project changes.

```sh
npm start
```

## How to build in release mode

Builds the project and places it into the `dist` folder.

```sh
npm run build
```
