# UAIS Fall 2022 Hex Bot Tournament
**This document is not finalized and subject to change at any time**

The UAIS is planning to host a hex bot tournament. We're planning a series of
workshops to get your team ready to win. Dates are broadly undecided and will be
announced sometime mid October. The tournament itself will likely take place at
the end of fall semester or right at the beginning of winter 2023

This repository contains a reference implementation for a very basic random
guess bot, the central communicating program, and defines the [communication
protocol](#Communication-protocol). Bots must adhere to this protocol
**exactly**, lest an inconsistent state between opponents arise. To ensure that
adherence is ensured, a test script in "Test/" has been provided which takes the
name of your program as an argument.

The Bot infrastructure is 🚀<u>**blazing fast**</u>⚡, since it's been written
in fearlessly concurrent and zero-cost abstracted ✨Rust✨.

# Hex the board game
[Hex](https://en.wikipedia.org/wiki/Hex_(board_game)) is a classic board game
for bots to play. It consists of two players, we'll denote them red and blue,
trying to form and uninterrupted path between their sides of the board. On each
turn, a player must place exactly one piece in any untaken tile. It's like
extended tiktoktoe

Our board uses coordinates of the form `[a-z][0-9]+`. Hexagons are adjacent to
2-6 other hexagons, depending on their position. An example of our coordinate
system on a 4x4 grid looks like the one below. Bots must be able to implement up
to 26x26 sized boards. All boards will be square

<p align="center">
    <img width="400" alt="Alacritty Logo" src="https://raw.githubusercontent.com/UndergraduateArtificialIntelligenceClub/UAIS-2022-Hex-Bot/main/hex_grid_example.jpg">
</p>

# Central program
This program will start up both bots, send and receive communication and
validate moves. Provide a full path to both executables as the argument

The program implements the following commands, which it reads from stdin line by
line. Errors, such as a bot failing to start up, will be output to the stderr.
Everything else goes in stdout. All commands can be used via their first letter
only. For example `next` and `n` map to the same command

| Cmd | Effect |
| --- | ------ |
| `run` | Starts up both bots with the `init_board` message |
| `run {file}` | Performs identical function to `run`, though also parses commands in `file`. Essentially loads a state.|
| `next` | Messages and receives a move from the next bot |
| `print` | Prints out the current board |
| `undo` | Undo the last move. May be useful for debugging |
| `save` | Back up the current game's state. Can be used for manual debugging |

# Communication Protocol
We'll be keeping things simple and use stdin/stdout messaging. Your bot must
implement valid responses to all the messages below. Each message is terminated
by a line break, notated `\n` on unix. An invalid response may result in an
immediate loss, so don't try to cheat

| Message | Example | Description |
| ------- | ------- | ----------- |
| `init_board {digit}` | `init_board 8` | Tells the bot to reset the game to an empty board w/ side length "digit" |
| `show_board` | `show_board` | Prints the board to stdout. Used for internal testing |
| `make_move` | `make_move` | Asks the bot to give their move, based on the current board |
| `seto {}` | `seto a1` | Tells the bot about a move for the other bot |
| `sety {}` | `sety a1` | Tells the bot to play a move for itself |
| `swap` | `swap` | Uses the opening "swap" move in Hex |
| `unset {}` | `unset a1` | Tells the bot to set a tile as unused |
| `check_win` | `check_win` | Tells the bot to check if the game is over. Returns `1` if itself has won, `-1` if the opponent has won, `0` if the game has not terminated. Note that draws are mathematically impossible in Hex. |
| `quit` | `quit` | The game is over |

Example message sequence:
```
init_board
make_move
seto a1
make_move
seto f4
make_move
quit
```

| Message from central program | Response required from bot | Example | Internal state |
| ---------------------------- | -------------------------- | ------- | -------------- |
| `init_board` | None | None | Reset the board and move counters |
| `make_move` | Respond with the coordinate you set your tile at | `a1` | Set the tile on your board too |
| `seto {}` | None | None | Set the coordinate as the opponent's tile on your board |
| `sety {}` | None | None | Set the coordinate as the your tile on your board |
| `unset {}` | None | None | Set the coordinate as an unmarked tile on your board |
| `quit` | Exit code 0 | None | Run destructors and shutdown program, preferably with exit code 0 |

[Database](http://hex.kosmanor.com/hex-bin/board/10/en_US:0/)

# Taking sides
Your bot must take in exactly 1 positional argument, either "black" or "white".
These will tell your bot which side it's playing on. See the random_bot for a
python3 example

The black player wins when they have an uninterrupted path from top to bottom.
For example, they could have filled 1 column with their tiles. The white player
wins when they have an uninterrupted path from left to right, like a row filled
with their tiles
