POSITRON LOGIC – RICH FULL STACK MVP

This version removes placeholders and implements:
- Real Rust-side control math
- WebGPU compute dispatch
- Lorentz-style particle rotation kernel
- WASM <-> JS control loop

Build:
cd rust
wasm-pack build --target web --out-dir ../web/pkg

Serve:
cd web
python -m http.server
