# engram-cap-random

Official engram capability `random`. Provides the guest with randomness from the host.
Published through the [engram-capabilities](https://github.com/immanent-ly/engram-capabilities) registry.

## Interface

- Provider export: `engram:cap-random/provider`.
- Host imports: the allowlisted WASI interfaces declared in `wit/world.wit`.

## Build

```sh
cargo component build --release --target wasm32-unknown-unknown
```

Artifact: `target/wasm32-unknown-unknown/release/cap_random.wasm`.

## License

FSL-1.1-ALv2. See LICENSE.md, CONTRIBUTING.md, and CLA.md.
