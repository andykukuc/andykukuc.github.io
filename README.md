# andykukuc.github.io

Personal portfolio and resume site. Built with Rust compiled to WebAssembly (WASM) and served as a static site via GitHub Pages.

## Tech Stack

- **Rust → WASM** — portfolio UI compiled with `wasm-bindgen`
- **Vanilla JS** — bootstraps and drives the WASM module
- **CSS** — custom styling
- **GitHub Pages** — hosting

## Live Site

[andykukuc.github.io](https://andykukuc.github.io)

## Structure

```
index.html                  # Entry point
style.css                   # Styles
wasm-portfolio/             # Rust WASM source
wasm/                       # Compiled WASM output
portfolio-*.js              # Generated JS bindings
portfolio-*_bg.wasm         # Compiled WASM binaries
andykukuc_resume.pdf        # Resume download
```

## Building the WASM

```bash
cd wasm-portfolio
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/portfolio.wasm --out-dir ../wasm --web
```

Requires `wasm-bindgen-cli`:
```bash
cargo install wasm-bindgen-cli
```

## License

MIT
