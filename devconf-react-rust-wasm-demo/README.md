# React Rust WASM integration

## Prerequisites
Install Rust with Cargo

Install wasm pack

```
cargo install wasm-pack
```

## Step 1

Create a React application using:

```
npx create-react-app devconf-react-rust-wasm-demo
```

## Step 2

Go into the directory folder and create a Rust library that we will eventually compile to WASM using:

```
cd devconf-react-rust-wasm-demo
cargo new rust-wasm --lib
```

## Step 3
Add to package.json file

```
build:wasm": "cd rust-wasm && wasm-pack build --target web --out-dir rust-wasm-bundle",
```

## Step 4

```
npm i ./rust-wasm/rust-wasm-bundle
```

## Step 5

Write methods in lib.rs decorated with #[wasm_bindgen]

## Step 6

Run build command 

```
npm run build:wasm
```

## Step 7 

Import code in react via 
```
import init, { add, concat_names_from_rust, concat_names_from_js } from 'rust-wasm';
```

And use them within React code.

## Step 8

Run the application
```
npm run start
```