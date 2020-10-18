
"""
BitcoinQuery: Query Bitcoin Core Data Files As A Database.

1. How to install?
    - `pip install bitcoin-explorer`

2. How to use?
    - Download Bitcoin Core app from bitcoin official website.
    - Sync full Bitcoin data.
    - If you wish to query transaction with txid (i.e. transaction id),
      run Bitcoin Core or `bitcoind` with flag `--txindex=1`,
      or rerun Bitcoin Core with `--reindex --txindex=1`.
    - After having the Bitcoin blockchain data, shutdown Bitcoin Core
      or `bitcoind`. This program cannot run concurrently with
      Bitcoin Core.
    - Open `python`, import the package, and instantiate `BitcoinDB`.