# Setup

You will need rust installed to run this program. If you're unable to install
rust, we can send you the binaries directly during an [UAIS
meeting](https://uais.dev/)

Get rust from [here](https://www.rust-lang.org/tools/install)

Alternatively run the following in your terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

To uninstall rust later, simply use `rustup self uninstall`

To check if you have rust installed run:

```bash
rustc --version
cargo --version
```

# Examples

```bash
# Get help
cargo run --release -- help
cargo run --release -- help test
cargo run --release -- help matchup

# Testing if your bot works
cargo run --release -- test ../random_bot/main.py white
cargo run --release -- test ~/Documents/code/hex_bot/my_binary_executable black

# You can also use the binary directly
./target/release/sentience_validator test ../random_bot/main.py white

# Making a bot play itself on a 4x4 board
cargo run --release -- matchup 4 ../random_bot/main.py ../random_bot/main.py

# Pitting two bots against each other on an 11x11 board
cargo run --release -- matchup 11 ../random_bot/main.py ~/Documents/rust/hex_box/target/release/hex_box
```
