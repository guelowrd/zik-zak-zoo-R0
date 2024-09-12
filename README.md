# Tic-tac-toe game but it's actually called Zik-zak-zoo (& you can zk-verify you won thanks to RISC Zero)

Title is pretty self-explanatory, but let's dive into the details!

## What is Zik-zak-zoo?

Zik-zak-zoo is a fun twist on the classic tic-tac-toe game, where:
- You play as 'Z' against the computer's 'K'
- The game logic is implemented in Rust
- You can prove your win using zero-knowledge proofs powered by RISC Zero

## Quick Start

First, make sure `rustup` is installed. The `rust-toolchain.toml` file will be used by `cargo` to automatically install the correct version.

To build all methods and execute the method within the zkVM, run the following command:

```bash
RISC0_DEV_MODE=0 cargo run --release
```

## How it works

1. The game uses a simple RNG for the computer's moves ([LCR](https://en.wikipedia.org/wiki/Linear_congruential_generator) with [MMIX](https://en.wikipedia.org/wiki/MMIX) parameters)
2. Your moves are recorded along with the initial seed
3. After the round is played, the game data is used to generate a zero-knowledge proof...
5. ... which get you bragging rights with cryptographic proof of your Zik-zak-zoo skills (well, if you won)!

## Project Structure

- `core/`: Contains the core game logic and data structures
- `host/`: Implements the game flow and user interaction
- `methods/`: Handles the zero-knowledge proof generation

## License

This project is licensed under the Apache License, Version 2.0. See the LICENSE file for details.

## Questions, Feedback, and Collaborations (keeping that part cause RISC Zero ppl are cool!)

RISC Zero would love to hear from you on Discord or Twitter.

- [bonsai access](https://bonsai.xyz/apply)
- [cargo-risczero](https://docs.rs/cargo-risczero)
- [crates](https://github.com/risc0/risc0/blob/main/README.md#rust-binaries)
- [dev-docs](https://dev.risczero.com)
- [dev-mode](https://dev.risczero.com/api/generating-proofs/dev-mode)
- [discord](https://discord.gg/risczero)
- [docs.rs](https://docs.rs/releases/search?query=risc0)
- [examples](https://github.com/risc0/risc0/tree/main/examples)
- [risc0-build](https://docs.rs/risc0-build)
- [risc0-repo](https://www.github.com/risc0/risc0)
- [risc0-zkvm](https://docs.rs/risc0-zkvm)
- [rustup](https://rustup.rs)
- [rust-toolchain](rust-toolchain.toml)
- [twitter](https://twitter.com/risczero)
- [zkvm-overview](https://dev.risczero.com/zkvm)
- [zkhack-iii](https://www.youtube.com/watch?v=Yg_BGqj_6lg&list=PLcPzhUaCxlCgig7ofeARMPwQ8vbuD6hC5&index=5)