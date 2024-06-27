# QR Code Creator

This repository contains a Rust-based project that generates QR codes from text input and deploys the generated code to GitHub Pages using WebAssembly.

## Prerequisites

Before you can build and run this project locally, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Node.js](https://nodejs.org/en/)

## Getting Started

### Clone the Repository

```sh
git clone https://github.com/your-username/qr_creator.git
cd qr_creator
```

## Build the Project

### Install the required Rust target

```sh
rustup target add wasm32-unknown-unknown
```

### Build the project with wasm-pack

```sh
wasm-pack build --target web
```

## Set Up the Web Page

### Navigate to the docs directory

```sh
cd docs
```

### Start a local web server to serve the files. You can use http-server for this

```sh
npx http-server . -c-1
```

Open your web browser and navigate to [http://localhost:8080](http://localhost:8080) to view the QR code creator.
