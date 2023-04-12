# decompress

**1. With only source file**

```sh
> cargo run *.zip
   Compiling decompress v0.1.0 (/Users/minwook/code/pust/decompress)
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/decompress blockchains.zip`
File 0 extracted to "blockchains/ethereum/"
File 1 extracted to "blockchains/ethereum/ethereum-eth-logo.png" (160579 bytes)
File 2 extracted to "blockchains/__MACOSX/ethereum/._ethereum-eth-logo.png" (524 bytes)
File 3 extracted to "blockchains/ethereum/Ethereum_Whitepaper_-_Buterin_2014.pdf" (941366 bytes)
File 4 extracted to "blockchains/__MACOSX/ethereum/._Ethereum_Whitepaper_-_Buterin_2014.pdf" (596 bytes)
File 5 extracted to "blockchains/icp/"
File 6 extracted to "blockchains/icp/internet-computer-icp-logo.png" (98375 bytes)
File 7 extracted to "blockchains/__MACOSX/icp/._internet-computer-icp-logo.png" (542 bytes)
File 8 extracted to "blockchains/icp/icp_whitepaper.pdf" (371 bytes)
File 9 extracted to "blockchains/__MACOSX/icp/._icp_whitepaper.pdf" (498 bytes)
File 10 extracted to "blockchains/bitcoin_logo.svg" (6259 bytes)
File 11 extracted to "blockchains/__MACOSX/._bitcoin_logo.svg" (487 bytes)
File 12 extracted to "blockchains/bitcoin.pdf" (184292 bytes)
File 13 extracted to "blockchains/__MACOSX/._bitcoin.pdf" (474 bytes)
```

**2. With source and destination directories**

```sh
> cargo run *.zip <OUT_DIR>
```
