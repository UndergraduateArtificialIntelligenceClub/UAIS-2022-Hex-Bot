#!/usr/bin/env python3
"""
This is an example of a script used to interact with the Hex bot.
"""
import sys
from bot import RandomHexBot

def main():
    bot = RandomHexBot()

    cmd = get_cmd()
    while cmd[0] != "quit":
        if cmd[0] == "init_board":
            bot.init_board(cmd)
        elif cmd[0] == "show_board":
            bot.show_board()
        elif cmd[0] == "make_move":
            bot.make_move()
        elif cmd[0] == "seto":
            bot.seto(cmd)
        elif cmd[0] == "sety":
            bot.sety(cmd)
        elif cmd[0] == "unset":
            bot.unset(cmd)
        elif cmd[0] == "check_win":
            bot.check_win()
        else:
            print("Cmd not recognized. Please refer to known commands.")
        respond()


def respond(message=""):
    print("= " + message)


def get_cmd():
    line = input()
    while not line:
        line = input()
    cmds = line.split(" ")
    if len(cmds) == 1:
        return cmds[0]

    return cmds


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
