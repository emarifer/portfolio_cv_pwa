# A simple Landing Page Progressive Web Application developed with Rust/WebAssembly + Yew + Tailwindcss.

## To run the app:

Add the WebAssembly target:

```
$ rustup target add wasm32-unknown-unknown
```

In order to build, bundle and send your Rust WASM application to the web, you need to install Trunk:

```
$ cargo install trunk
```

Finally, since we use the Tailwind CSS framework, we will have to run in the root of the project:

```
npm i
```

Run the app:

```
$ trunk serve
```

Build the application by running:

```
$ trunk build --release
```
