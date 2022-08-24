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

## Test.sh script

A helpful wrapper is provided for testing your bot

```bash
./test.sh --help   # Prints the help menu

./test.sh white ../random_bot/main.py -c white  # Tests the example bot using white
./test.sh black ../random_bot/main.py -c black  # Tests the example bot using black

# Your arguments may be different. For example, this bot takes in just one
# uppercase letter to determine its color and has a different path
./test.sh black ~/Documents/rust/hex_bot/target/release/hex_bot B
```

## Using cargo directly

```bash
# Testing if your bot works
cargo run --release -- test white ../random_bot/main.py -- -c white
cargo run --release -- test black ~/Documents/code/hex_bot/my_binary_executable -- black

# Running against another bot TODO
#cargo run --release -- matchup ../random_bot/main.py ~/Documents/code/hex_bot/my_binary_executable

# You can also use the binary directly
./target/release/sentience_validator test white ../random_bot/main.py -- -c white
```
