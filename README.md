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

Currently, it is compatible with Bitcoin Core version
`Bitcoin Core version v0.21.1.0-g194b9b8792d9b0798fdb570b79fa51f1d1f5ebaf
Copyright (C) 2009-2020 The Bitcoin Core developers`.

## Examples

It contains one class `BitcoinDB`.

```python
import bitcoin_explorer as bex

# parse the same path as `--datadir` argument for `bitcoind`.
db = bex.BitcoinDB("~/Bitcoin")

# get the length of the longest chain currently on disk.
db.get_block_count()

# get block of a certain height
db.get_block(1000)

# to retrieve the connected outputs of each inputs as well.
# note that this is inefficient.
# Use `get_block_iter_range(end, connected=True)` for better performance.
db.get_block(1000, connected=True)

# get block hash of a certain height.
db.get_hash_from_height(1000)

# a fast method for getting just the header.
# in memory query, no disk access
db.get_block_header(1