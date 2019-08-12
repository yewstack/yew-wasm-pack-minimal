## About

This template demonstrates the minimum code and tooling necessary for a frontend web app using [`Yew`](https://github.com/yewstack/yew), [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen), and [`wasm-pack`](https://github.com/rustwasm/wasm-pack).

Note: [`yew-wasm-pack-template`](https://github.com/yewstack/yew-wasm-pack-template) is the full-featured counterpart to this template, integrating many common web technologies.

## Usage

### 1) Install `wasm-pack`

Follow the `installation` link at [`wasm-pack`](https://github.com/rustwasm/wasm-pack).

### 2) Build

Enter `wasm-pack build --target web` from your project's root directory.

### 3) [optional] Test Run

Run a webserver from your project's root directory, such as with `python -m SimpleHTTPServer 8080`, to view the results.

### 4) Deploy

Access your generated build artifacts in ./pkg from your project's root directory.
