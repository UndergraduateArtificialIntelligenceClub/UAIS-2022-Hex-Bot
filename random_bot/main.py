#!/usr/bin/env python3
"""
This is an example of a script used to interact with the Hex bot.
"""
import sys
from bot import RandomHexBot
from constants import WHITE, BLACK
import argparse

def main():
    parser = argparse.ArgumentParser(description="Example hex bot that makes random valid placements")
    parser.add_argument("color", metavar="<COLOR>", choices=["white", "black"],
                        help="This bot's color. White is left->right")
    args = parser.parse_args()

    color = WHITE if args.color == "white" else BLACK
    bot = RandomHexBot(color)

    help_items = [
        ["Command", "Example", "Description"],
        ["init_board {digit}", "init_board 8", "Tells the bot to reset the game to an empty board w/ side length digit"],
        ["show_board", "show_board", "Prints the board to stdout. Used for internal testing"],
        ["make_move", "make_move", "Asks the bot to give their move, based on the current board"],
        ["seto {}", "seto a1", "Tells the bot about a move for the other bot"],
        ["sety {}", "sety a1", "Tells the bot to play a move for itself"],
        ["swap", "swap", "Uses the opening \"swap\" move in Hex"],
        ["unset {}", "unset a1", "Tells the bot to set a tile as unused"],
        ["check_win", "check_win", "Tells the bot to check if the game is over. Returns 1 if itself has won, -1 if the opponent has won, 0 if the game has not terminated"],
        ["quit", "quit", "The game is over"]
    ]

    help_response = "Cmd not recognized. Please refer to known commands below:"

    cmd = get_cmd()
    while cmd[0] != "quit":
        try:
            getattr(bot, cmd[0])
            bot.run_command(cmd)
        except AttributeError:
            print(help_response)
            for command, ex, desc in help_items:
                print("{:30}{:30}{}".format(command, ex, desc))

            print("\nNote that draws are impossible in hex, so no response for a draw is required")

        cmd = get_cmd()
    return


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
