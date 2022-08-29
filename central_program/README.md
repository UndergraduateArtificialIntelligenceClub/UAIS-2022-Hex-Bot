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

# Testing if your bot works
cargo run --release -- test ../random_bot/main.py white
cargo run --release -- test ~/Documents/code/hex_bot/my_binary_executable black

# Running against another bot TODO
#cargo run --release -- matchup ../random_bot/main.py ~/Documents/code/hex_bot/my_binary_executable

# You can also use the binary directly
./target/release/sentience_validator test ../random_bot/main.py white
```
