#!/usr/bin/env python3

"""
This is an exampel of a script used to interact with the Hex bot.
"""

from bot import RandomHexBot


def main():
    bot = RandomHexBot()

    cmd = getcmd()
    while cmd != "quit":
        respond()


def respond(message=""):
    print("= " + message)


def getcmd():
    return "quit"


if __name__ == "__main__":
    main()
