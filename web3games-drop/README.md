# Web3Games NFT 

near 721 version

## Testing
To test run:
```bash
cargo test -- --nocapture
```
## Build
to run:
```bash
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
```