# UAIS Fall 2022 Hex Bot Tournament
**This document is not finalized and subject to change at any time**

The UAIS is planning to host a hex bot tournament. We're planning a series of
workshops to get your team ready to win. Dates are broadly undecided and will be
announced sometime mid October. The tournament itself will likely take place at
the end of fall semester or right at the beginning of winter 2023

This repository contains a reference implementation for a very basic random
guess bot, the central communicating program, and defines the [communication
protocol](#Communication-protocol). Bots must adhere to this protocol exactly

[Hex](https://en.wikipedia.org/wiki/Hex_(board_game)) is a classic board game
for bots to play. It consists of two players, we'll denote them red and blue,
trying to form and uninterrupted path between their sides of the board. On each
turn, a player must place exactly one piece in any untaken tile. It's like
extended tiktoktoe

# Central program
This program will start up both bots, send and receive communication and
validate moves. Provide a full path to both executables as the argument

The program implements the following commands, which it reads from stdin line by
line. Errors, such as a bot failing to start up, will be output to the stderr.
Everything else goes in stdout. All commands can be used via their first letter
only. For example `next` and `n` map to the same command

| Cmd | Result |
| --- | ------ |
| `run` | Starts up both bots with the `init_board` message |
| `run {}` | Like `run` though also sets the board's state to the given save file |
| `next` | Messages and receives a move from the next bot |
| `print` | Prints out the current board |
| `undo` | Undo the last move. May be useful for debugging |
| `save` | Back up the current game's state. Can be used for manual debugging |

# Communication Protocol
We'll be keeping things simple and use stdin/stdout messaging. Your bot must
implement valid responses to all the messages below. Each message is terminated
by a line break, often notated `\n`. An invalid response may result in an
immediate loss, so don't try to cheat

| Message | Example | Description |
| ------- | ------- | ----------- |
| `init_board` | `init_board` | Tells the bot to reset the game |
| `make_move` | `make_move` | Asks the bot to give their move, based on the current board |
| `seto {}` | `seto a1` | Tells the bot about a move the other bot made |
| `sety {}` | `sety a1` | Tells the bot about a move it made |
| `unset {}` | `unset a1` | Tells the bot to set a tile as unused |
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

Bots must send back a message, through the stdout in response to certain
messages. Otherwise a bot will only be required to update its internal state

| Message from central program | Response required from bot | Example | Internal state |
| ---------------------------- | -------------------------- | ------- | -------------- |
| `init_board` | None | None | Reset the board and move counters |
| `make_move` | Respond with the coordinate you set your tile at | `a1` | Set the tile on your board too |
| `seto {}` | None | None | Set the coordinate as the opponent's tile on your board |
| `sety {}` | None | None | Set the coordinate as the your tile on your board |
| `unset {}` | None | None | Set the coordinate as an unmarked tile on your board |
| `quit` | Exit code 0 | None | Run destructors and shutdown program, preferably with exit code 0 |
| Anything | The bot sees a message as invalid | `error` | Don't change anything |

`sety` and `unset` provide a way to save the game. They won't be used in the
normal course of play otherwise

A bot may respond with `error` when the requested operation conflicts with its
internal logic. For example, if a1 is set to your tile and the bot receives
`seto a1`. It is not necessary to implement this, you could just overwrite a1,
though it may help catch errors faster if the bots become desynchronized
