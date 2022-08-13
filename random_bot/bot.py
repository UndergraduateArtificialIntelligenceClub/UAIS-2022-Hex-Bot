from multiprocessing.sharedctypes import Value
from constants import BORDER, WHITE, BLACK, EMPTY
from random import choice, seed

seed(42)  # Get same results temporarily

# Note: WHITE goes left->right, BLACK goes top->bottom


class RandomHexBot:
    def __init__(self, color, board_size=26):
        self.color = color
        self.move_count = 0
        self.opp = BLACK if color == WHITE else WHITE
        self.init_board(board_size)
        self.init_neighbours()

        self.pub = {
            "init_board": self.init_board,
            "show_board": self.show_board,
            "make_move": self.make_move,
            "seto": self.seto,
            "sety": self.sety,
            "unset": self.unset,
            "check_win": self.check_win,
        }

        self.argnums = {
            "init_board": 1,
            "show_board": 0,
            "make_move": 0,
            "seto": 1,
            "sety": 1,
            "unset": 1,
            "check_win": 0,
        }

    def is_runnable(self, cmd):
        """Checks to see whether the command in 'cmd' conforms to the expected format

        Args:
            cmd (List[str]): A space-separated list of the commands given on the command line

        Returns:
            bool: True if the command exists and has the correct # of arguments, False otherwise
        """
        assert len(cmd)
        if cmd[0] not in self.pub:
            return False
        if len(cmd) - 1 != self.argnums[cmd[0]]:
            return False

        return True

    def run_command(self, cmd):
        """Executes the command contained within 'cmd' if it is applicable

        Args:
            cmd (List[str]): A space-separated list of the commands given on the command line
        """
        if len(cmd) > 1:
            self.pub[cmd[0]](cmd[1])
        else:
            self.pub[cmd[0]]()

    def init_board(self, board_size):
        """Tells the bot to reset the game to an empty board with a specified side length

        Args:
            board_size (int): The width & height of the hex game board to create
        """
        board_size = int(board_size)
        self.board_size = board_size
        self.board = [EMPTY for i in range(board_size**2)]
        self.move_count = 0

        self.offsets = [
            -1,
            1,
            -self.board_size,
            -self.board_size - 1,
            self.board_size,
            self.board_size + 1,
        ]

        #self.board.append(BORDER)
        #for _ in range(self.board_size):
        #    for __ in range(self.board_size):
        #        self.board.append(EMPTY)
        #    self.board.append(BORDER)

        self.init_neighbours()

    def show_board(self):
        """Prints the board to stdout. This is primarily used for
        testing purposes & when playing against a human opponent

        Returns:
            bool: True if the command exists and ran successfully, False otherwise
        """
        tile_chars = {
            EMPTY: ".",
            WHITE: "X",
            BLACK: "O",
        }

        chars = list(map(lambda x: tile_chars[x], self.board))

        for i in reversed(range(1, self.board_size+1)):  # Reverse to avoid shifting indicies
            chars.insert(i * self.board_size, "|")

        print("".join(chars))
        return

    def make_move(self):
        """Generates the move. For this bot, the move is randomly selected from all empty positions."""
        empties = []
        for i, cell in enumerate(self.board):
            if cell == EMPTY:
                empties.append(i)

        move = self.coord_to_move(choice(empties))
        self.sety(move)
        print(move)
        return

    def seto(self, move):
        """Tells the bot about a move for the other bot

        Args:
            move (str): A human-readable position on which the opponent has just played
        """
        # TODO: Handle swap move. Logic moved to move_to_coord()
        coord = self.move_to_coord(move)
        if self.board[coord] == EMPTY:
            # TODO: Warn or not?
            #print("Trying to play on a non-empty square!")
            self.board[coord] = self.opp
            self.move_count += 1
        return

    def sety(self, move):
        """Set Your [tile]. Tells the bot to play a move for itself

        Args:
            move (str): A human-readable position on the board
        """
        coord = self.move_to_coord(move)
        if self.board[coord] != EMPTY:
            #print("Trying to play on a non-empty square!")
            return
        self.board[coord] = self.color
        self.move_count += 1
        return

    def unset(self, move):
        """Tells the bot to set a tile as unused

        Args:
            move (str): A human-readable position on the board
        Returns:
            bool: True if the move has been unmade, False otherwise
        """

        coord = self.move_to_coord(move)
        self.board[coord] = EMPTY
        return True

    def check_win(self):
        """Checks whether or not the game has come to a close.

        Returns:
            int: 1 if this bot has won, -1 if the opponent has won, and 0 otherwise. Note that draws
            are mathematically impossible in Hex.
        """
        seen = set()

        def dfs(i, color, level=0):
            """Oopsie poopsie! I made a fucky wucky! This code is super-duper slow! UwU7

            Args:
                i (int): The current location of the depth-first search
                color (int): The current color of the dfs.
            """
            is_right_column = (i + 1) % self.board_size == 0
            is_bottom_row = i >= self.board_size * (self.board_size - 1)

            if color == WHITE and is_right_column:
                return True
            elif color == BLACK and is_bottom_row:
                return True

            # Label hexagon as 'visited' so we don't get infinite recusion
            seen.add(i)
            for offset in self.offsets:
                new_coord = i + offset
                if (
                    new_coord not in seen
                    and 0 <= new_coord < len(self.board)
                    and self.board[new_coord] == color
                    and dfs(new_coord, color, level=level + 1)
                ):
                    return True

            # Remove hexagon so we can examine it again next time (hint:is this needed?)
            seen.remove(i)
            return False

        # Iterate over all starting spaces for black & white, performing dfs on empty
        # spaces (hint: this leads to repeated computation!)
        for i in range(0, self.board_size):
            if self.board[i] == BLACK and dfs(i, BLACK):
                print(1 if self.color == BLACK else -1)
                return

        for i in range(0, len(self.board), self.board_size):
            if self.board[i] == WHITE and dfs(i, WHITE):
                print(1 if self.color == WHITE else -1)
                return

        print(0)
        return

    def init_neighbours(self):
        """Precalculates all neighbours for each cell"""
        self.neighbours = []
        for i, cell in enumerate(self.board):
            if cell == EMPTY:
                self.neighbours.append([])
                for offset in self.offsets:
                    new_coord = offset + i
                    if 0 <= new_coord < len(self.board):
                        self.neighbours[-1].append(offset)

    def coord_to_move(self, coord):
        """Converts an integer coordinate to a human-readable move

        Args:
            coord (int): A coordinate within self.board

        Returns:
            str: A human-readable version of coord
        Example:
            >>> assert coord_to_move(0) == "a1"
            >>> assert coord_to_move(self.board_size + 2) == "b3"
            >>> assert coord_to_move(22 * self.board_size + 11) == "w12"
        """
        letter = chr(coord // self.board_size + ord("a"))
        number = coord % self.board_size + 1

        return f'{letter}{number}'

    def move_to_coord(self, move):
        """Converts a human-readable move to a coordinate within self.board

        Args:
            move (str): A human-readable position on the board

        Returns:
            int: The integer coordinate of 'move', used to interact with the board

        Example:
            >>> assert move_to_coord("a1") == 0
            >>> assert move_to_coord("b3") == self.board_size + 2
            >>> assert move_to_coord("w12") == 22 * self.board_size + 11
        """
        # TODO: Handle swap move
        if move == "swap":
            self.swap_move()
            return

        assert len(move) >= 2, "Move must be a character-digit pair. Ex: a12"
        assert move[0].isalpha(), "First character must be a letter. Ex: a12"
        assert move[1:].isdigit(), "Digits must follow the first character. Ex: a12"
        assert (
            ord(move[0]) - ord("a") < self.board_size
        ), "The letter in 'move' must have value less than board size!"
        assert (
            0 < int(move[1:]) <= self.board_size
        ), "Integer part of move must be within range (0, board_size]!"

        column = int(move[1:]) - 1
        row = ord(move[0]) - ord("a")
        return row * self.board_size + column
