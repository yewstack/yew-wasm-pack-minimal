## About

This template demonstrates the minimum code and tooling necessary for a frontend web app with simple deployable artifacts consisting of one HTML file, one JavaScript file, and one WASM file, using [`Yew`](https://github.com/yewstack/yew), [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen), and [`wasm-pack`](https://github.com/rustwasm/wasm-pack).

Note: [`yew-wasm-pack-template`](https://github.com/yewstack/yew-wasm-pack-template) is the full-featured counterpart to this template, integrating many common web technologies.

## Usage

### 1) Install `Rust` and `wasm-pack`

Follow the instructions at https://www.rust-lang.org/tools/install and follow the `installation` link at [`wasm-pack`](https://github.com/rustwasm/wasm-pack).

### 2) Build

Enter `wasm-pack build --target web` from your project's root directory.

### 3) [temporary] Bundle

Enter `rollup ./main.js --format iife --file ./pkg/bundle.js` from your project's root directory.

Note: Until `wasm-pack` [RFC #6](https://github.com/rustwasm/rfcs/blob/master/text/006-local-js-dependencies.md) is implemented, there is no available option to [generate a single amalgamated JavaScript file](https://github.com/rustwasm/wasm-pack/issues/699).  In the interim, a bundler such as [`Rollup`](https://rollupjs.org/guide/en/#quick-start), must be used.

### 4) [optional] Test Run

Run a webserver from your project's root directory, such as with `python -m SimpleHTTPServer 8080`, to view the results.

### 5) Deploy

Access your generated build artifacts, `bundle.js` and `yew_wasm_pack_minimal_bg.wasm`, in ./pkg from your project's root directory.
