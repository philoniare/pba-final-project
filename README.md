[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-24ddc0f5d75046c5622901739e7c5dd533143b0c8e959d652212380cedb1ea36.svg)](https://classroom.github.com/a/6Wn1Dsn4)
# PBA Assignment - FRAME

## !! See [ASSIGNMENT.md](./ASSIGNMENT.md) for instructions to complete this assignment !!

**_TODO: Update this README for the Grading Team about your project!_**

---

## [Substrate Node Template](https://github.com/substrate-developer-hub/substrate-node-template)

A fresh FRAME-based [Substrate](https://www.substrate.io/) node, ready for hacking :rocket:

### Setup

Please first check the latest information on getting starting with Substrate dependencies required to build this project [here](https://docs.substrate.io/main-docs/install/).

### Development Testing

To test while developing, without a full build (thus reduce time to results):

```sh
cargo t -p pallet-dex
cargo t -p pallet-dpos
cargo t -p pallet-voting
cargo t -p <other crates>
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
