from constants import BORDER, WHITE, BLACK, EMPTY
from random import choice


class RandomHexBot:
    def __init__(self, color, board_size=8):
        self.color = color
        self.init_board()
        self.init_neighbours()

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
        """
        for i, cell in enumerate(self.board):
            if cell == BORDER:
                print("\n" + " " * (i / self.board_size))  # Padding subsequent rows
            else:
                print("{} ".format(cell))

        return

    def make_move(self):
        """Generates the move. For this bot, the move is randomly selected from all empty positions.

        Returns:
            str: The move chosen to play by the bot
        """
        empties = []
        for i, cell in self.board:
            if cell == EMPTY:
                empties.append(i)

        move = self.coord_to_move(choice(empties))
        self.sety(move)
        return move

    def seto(self, move):
        """Tells the bot about a move for the other bot

        Args:
            move (str): A human-readable position on which the opponent has just played

        Returns:
            bool: True if the move successfully been played internally, False otherwise
        """
        return True

    def sety(self, move):
        """Tells the bot to play a move for itself

        Args:
            move (str): A human-readable position on the board
        Returns:
            bool: True if the move is possible (and has been made), False otherwise
        """
        return True

    def unset(self, move):
        """Tells the bot to set a tile as unused

        Args:
            move (str): A human-readable position on the board
        Returns:
            bool: True if the move has been unmade, False otherwise
        """
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

        def dfs(self, i, color):
            """Oopsie poopsie! I made a fucky wucky! This code is too slow! UwU

            Args:
                i (int): The current location of the depth-first search
                color (int): The current color of the dfs.

            Returns:
                bool: Whether or not the current dfs found a winner or not
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
                return 1 if cell == self.color else -1

        return 0  # No winner yet!

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
        move = ""
        return move

    def move_to_coord(self, move):
        """Converts a human-readable move to a coordinate within self.baord

        Args:
            move (str): A human-readable position on the board

        Returns:
            int: The integer coordinate of 'move' on self.board
        """
        coord = 0
        return coord
