#!/usr/bin/env python3
"""
This is an example of a script used to interact with the Hex bot.
"""
import sys
from bot import RandomHexBot

def main():
    bot = RandomHexBot()

    msg = get_msg()
    while msg != "quit":
        respond()


def respond(message=""):
    print("= " + message)


def getcmd():
    return "quit"


if __name__ == "__main__":
    main()
