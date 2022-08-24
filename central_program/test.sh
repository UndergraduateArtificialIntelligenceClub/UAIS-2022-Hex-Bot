#!/usr/bin/env bash
print_help() {
  cat <<HELP
A shell wrapper to make testing your bot easier

USAGE: $(basename "$0") [FLAGS] <COLOR> <PATH> [BOT_ARG [BOT_ARG...]]

FLAGS:
  -h, --help    Print this help message

ARGS:
  COLOR    Your bot's color with given args. Remember to test both white & black
  PATH     Path to the main executable of your bot
  BOT_ARG  An argument to pass to your bot

EXAMPLES:
  $(basename "$0") white ../random_bot/main.py -c white
  $(basename "$0") black ../random_bot/main.py -c black
  $(basename "$0") black ~/Documents/rust/hex_bot/target/release/hex_bot BLACK
  $(basename "$0") black ~/Documents/rust/hex_bot/target/release/hex_bot -d horizontal
HELP
}

if [[ -z "$1" || "$1" == -h || "$1" == --help || ! ( "$1" == white || "$1" == black ) ]]; then
  print_help
  exit 1
else
  color="$1"; shift
  path="$1"; shift
  cargo run --release -- test "$color" "$path" -- "$@"
fi
