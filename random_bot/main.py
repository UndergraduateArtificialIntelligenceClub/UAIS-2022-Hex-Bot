#!/usr/bin/env python3
"""
This is an example of a script used to interact with the Hex bot.
"""
import sys
from bot import RandomHexBot
from constants import WHITE


def main():
    bot = RandomHexBot(WHITE)

    cmd = get_cmd()
    while cmd[0] != "quit":
        if not bot.is_runnable(cmd):
            print("Cmd not recognized. Please refer to known commands.")
        else:
            bot.run_command(cmd)
        cmd = get_cmd()


def get_cmd():
    line = ""
    while not line:
        try:
            line = input()
        except EOFError:
            return ["quit"]

    return line.strip().split(" ")


if __name__ == "__main__":
    main()

# | `init_board {digit}` | `init_board 8` | Tells the bot to reset the game to an empty board w/ side length "digit" |
# | `show_board` | `show_board` | Prints the board to stdout. This is primarily used for testing purposes & when playing against a human opponent |
# | `make_move` | `make_move` | Asks the bot to give their move, based on the current board |
# | `seto {}` | `seto a1` | Tells the bot about a move the other bot made |
# | `sety {}` | `sety a1` | Tells the bot about a move it made |
# | `unset {}` | `unset a1` | Tells the bot to set a tile as unused |
# | `check_win` | `check_win` | Tells the bot to check if the game is over. Returns `1` if itself has won, `-1` if the opponent has won,
#     `0` if the game has not terminated. Note that draws are mathematically impossible in Hex. |
# | `quit` | `quit` | The game is over |
