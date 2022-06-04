# Polkadot Hackathon Project

We are translating Uniswap v2 to [Ink].

[Ink]: https://github.com/paritytech/ink


## Dependencies

Install [`cargo-contract`] for building Ink contracts:

```
cargo install dylint-link cargo-dylint
cargo install cargo-contract
```

`cargo-contract` requires [Binaryen] for its `wasm-opt` tool.
It's in most system packagers like apt and homebrew,
but versions older than 99 are rejected by the Rust wasm tooling.
The best way to get this tool may be to get it directly
from their [binary releases][br] or build it yourself,
in either case adding its `bin` directory to your `PATH` environment variable.

[isg]: https://ink.substrate.io/getting-started/setup
[`cargo-contract`]: https://github.com/paritytech/cargo-contract
[Binaryen]: https://github.com/WebAssembly/binaryen
[br]: https://github.com/WebAssembly/binaryen/releases

For the web frontend:

```
(cd www && npm install)
```

## Building

At the moment ink crates have to be built individually:

```
cargo contract build --manifest-path=components/swap_traits/Cargo.toml
```

For the web frontend:

```
npm run build
```

or to serve it with live reloading:

```
npm run dev
```

## License

GPL3
