# 📣 MeMeSwap: Uniswap V2 for Meme Coins

![Memeswap logo](https://i.ibb.co/CPck0ZR/DALL-E-2024-01-25-16-12-59-Design-a-logo-for-Meme-Swap-a-Uniswap-style-decentralized-token-exchange.png)

Welcome to MeMeSwap! Built as a version of Uniswap V2 specifically for Meme Coins, MeMeSwap is an automatic protocol for token swaps and liquidity provision on a decentralized network. You can create liquidity pools, provide liquidity, and swap tokens seamlessly!

## 🚀 Features
- **Swap Tokens**: Safely swap any two Meme tokens without the need for a centralized party only with 0.3% fee.
- **Add Liquidity**: Provide liquidity to pools and earn on the pro-rata share of trading fees.
- **Remove Liquidity**: Remove your liquidity anytime, access to your assets when you need them.
- **Mint LP Tokens**: Minting Liquidity Provider tokens as proof of your pro-rata share in the pool.
- **Burn LP Tokens**: Burn your LP tokens to remove liquidity and receive your pro-rata share of the pool's assets.

## 🛠 Technical Features
- Uses the standard `pallet-assets` to manage multi-token swaps
- Follows the standard rewarding mechanism that rewards liquidity providers with the 0.3% of the swaps
- Exposes Traits for fetching token ratio and amount needed to swap to other pallets acting as the price oracle for existing liquidity pools


## 📚 Documentation
- The html documentation can be found under `/docs/index.html`

## 💗 Support
Give a ⭐️ if this project helped you! Your support helps us continuously improve and maintain this project.

## 📬 Contact
If you have any questions or need further clarification, feel free to reach out to us.


**MeMeSwap: A new playground for meme coin enthusiasts. Happy Swapping!**

## Design Considerations
- My pallet assumes that AssetId is an integer generic so that it can be used for ordering. We would later on add a functionality to reward early liquidity provider creators. 


## Running locally
### Setup

Please first check the latest information on getting starting with Substrate dependencies required to build this project [here](https://docs.substrate.io/main-docs/install/).

### See docs
```sh
cargo +nightly doc --open --package pallet-dex --no-deps
```

### Development Testing

To test while developing, without a full build (thus reduce time to results):

```sh
cargo t -p pallet-dex
```

### Build

Build the node without launching it, with `release` optimizations:

```sh
cargo b -r
```

### Run

Build and launch the node, with `release` optimizations:

```sh
cargo r -r -- --dev
```

### CLI Docs

Once the project has been built, the following command can be used to explore all CLI arguments and subcommands:

```sh
./target/release/node-template -h
```
