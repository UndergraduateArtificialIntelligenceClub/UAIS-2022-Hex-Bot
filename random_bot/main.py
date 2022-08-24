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

    cmd = get_cmd()
    while cmd[0] != "quit":
        try:
            getattr(bot, cmd[0])
            bot.run_command(cmd)
        except AttributeError:
            print("Cmd not recognized. Please refer to known commands.")

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
