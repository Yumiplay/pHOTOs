# bitcoin-explorer
![CI](https://github.com/Congyuwang/Py-Bitcoin-Explorer/actions/workflows/CI.yml/badge.svg)
![PyPI](https://img.shields.io/pypi/dm/bitcoin-explorer)
![PyPI](https://img.shields.io/pypi/wheel/bitcoin-explorer)

`bitcoin_explorer` is an efficient library for reading
bitcoin-core binary blockchain file as a database (utilising multi-threading).

This package is ported using pyO3 from rust library `bitcoin-explorer`

### Installation

MacOS, Windows, and Linux wheels are provided.

Use `pip install bitcoin-explorer` to install.

## Documentation

This library has a Rust version, go to [Rust Documentation](https://docs.rs/bitcoin-explorer/)

For python documentation, go to [Documentation](https://congyuwang.github.io/Py-Bitcoin-Explorer/bitcoin_explorer.html).

## Compatibility Note

This package deals with the binary file of another software `Bitcoin Core`.
It might not be compatible with older Bitcoin Core versions.

Currently, it is compatible with Bitcoin Core 