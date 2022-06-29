from multiprocessing.sharedctypes import Value
from constants import BORDER, WHITE, BLACK, EMPTY
from random import choice


class RandomHexBot:
    def __init__(self, color, board_size=8):
        self.color = color
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
            "init_board": 0,
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
            board_size (int): The width & height of the game board to create
        """
        self.board_size = board_size
        self.board = []

        self.offsets = [
            -1,
            1,
            -self.board_size - 1,
            -self.board_size - 2,
            self.board_size + 1,
            self.board_size + 2,
        ]

        self.board.append(BORDER)
        for _ in range(self.board_size):
            for __ in range(self.board_size):
                self.board.append(EMPTY)
            self.board.append(BORDER)

        self.init_neighbours()

    def show_board(self):
        """Prints the board to stdout. This is primarily used for
        testing purposes & when playing against a human opponent

        Returns:
            bool: True if the command exists and ran successfully, False otherwise
        """
        for i, cell in enumerate(self.board[:-1]):
            if cell == BORDER:
                print("\n" + " " * (i // self.board_size), end="")  # Padding subsequent rows
            elif cell == WHITE:
                print("W ", end="")
            elif cell == BLACK:
                print("B ", end="")
            else:
                print(". ", end="")
        print()
        return True

    def make_move(self):
        """Generates the move. For this bot, the move is randomly selected from all empty positions.
        """
        empties = []
        for i, cell in self.board:
            if cell == EMPTY:
                empties.append(i)

        move = self.coord_to_move(choice(empties))
        print(move)
        self.sety(move)
        return

    def seto(self, move):
        """Tells the bot about a move for the other bot

        Args:
            move (str): A human-readable position on which the opponent has just played

        Raises:
            IndexError: When the move played is for an occupied square
        """
        
        coord = self.move_to_coord(move)
        if self.board[coord] != EMPTY:
            raise IndexError("Trying to play on a non-empty square!")
        self.board[coord] = self.opp
        return

    def sety(self, move):
        """Tells the bot to play a move for itself

        Args:
            move (str): A human-readable position on the board

        Raises:
            IndexError: When the move played is for an occupied square
        """
        coord = self.move_to_coord(move)
        if self.board[coord] != EMPTY:
            raise IndexError("Trying to play on a non-empty square!")
        self.board[coord] = self.color
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
        top_left = False
        bottom_right = False

        def dfs(i, color):
            """Oopsie poopsie! I made a fucky wucky! This code is too slow! UwU

            Args:
                i (int): The current location of the depth-first search
                color (int): The current color of the dfs.
            """
            if color == WHITE:
                if (i - 1) % self.board_size == 0:
                    top_left = True  # Touching left side of board
                if (i + 1) % self.board_size == 0:
                    bottom_right = True  # Touching right of board
            else:
                if i <= self.board_size:
                    top_left = True  # Touching top of board
                if i >= (self.board_size) * (self.board_size + 1):
                    bottom_right = True  # Touching bottom of board

            # 'Base case' of dfs
            if top_left and bottom_right:
                return True

            seen.add(i)
            for offset in self.offsets:
                new_coord = i + offset
                if (
                    i not in seen
                    and 0 <= i < len(self.board)
                    and self.board[i] == color
                    and dfs(new_coord, color)
                ):
                    return True

            seen.remove(i)
            return False

        # Iterate over all spaces, performing dfs on empty
        # spaces (hint: this leads to repeated computation!)
        for i, cell in enumerate(self.board):
            top_left = False
            bottom_right = False
            seen = set()

            if cell in [WHITE, BLACK] and dfs(i, cell):
                print(1 if cell == self.color else -1)
                return

        print(0)  # No winner yet!
        return

    def init_neighbours(self):
        """Precalculates all neighbours for each cell"""
        self.neighbours = []
        for i, cell in enumerate(self.board):
            if cell == EMPTY:
                self.neighbours.append([])
                for offset in self.offsets:
                    new_coord = offset + i
                    if (
                        0 <= new_coord < len(self.board)
                        and self.board[new_coord] != BORDER
                    ):
                        self.neighbours[-1].append(offset)

        return

    def coord_to_move(self, coord):
        """Converts an integer coordinate to a human-readable move

        Args:
            coord (int): A coordinate within self.board

        Returns:
            str: A human-readable version of coord
        """
        assert coord, "Coord must be non-zero"
        coord -= 1  # Ignores border at start of board
        letter = ord('a') + coord // (self.board_size + 1)
        number = coord % (self.board_size + 1) + 1
        
        return chr(letter) + str(number)

    def move_to_coord(self, move):
        """Converts a human-readable move to a coordinate within self.board

        Args:
            move (str): A human-readable position on the board

        Returns:
            int: The integer coordinate of 'move', used to interact with the board
        """
        assert len(move), "Move must be a non-empty string!"
        assert move[0].isalpha(), "The first character of move must be an alphanumeric character!"
        assert ord(move[0]) - ord('a') < self.board_size, "The letter in 'move' must have value less than board size!"
        assert move[1:].isdigit(), "All characters other than the first must be numeric in move!"
        assert 0 < int(move[1:]) <= self.board_size, "Integer part of move must be within range (0, board_size]!"

        coord = 1  # Ignores first border in self.board
        for _ in range(ord('a'), ord(move[0])):
            coord += self.board_size + 1
        
        offset = int(move[1:]) - 1
        coord += offset
        return coord
