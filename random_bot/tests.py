import unittest
from bot import RandomHexBot
from constants import EMPTY, WHITE, BLACK

"""These are some basic tests FOR THE RANDOM BOT SPECIFICALLY.
If you copy-paste these tests to a bot you wrote, there is a high likelihood that they will not work.
TL;DR Write your own tests!
"""

class BotUnitTest(unittest.TestCase):
    def test_coord_to_move(self):
        board_size = 8
        bot = RandomHexBot(WHITE, board_size=board_size)
        self.assertEqual(bot.coord_to_move(1), "a1")
        self.assertEqual(bot.coord_to_move(board_size), "a8")

    def test_move_too_coord_success(self):
        board_size = 8
        bot = RandomHexBot(WHITE, board_size=board_size)
        self.assertEqual(bot.move_to_coord("a1"), 1)
        self.assertEqual(bot.move_to_coord("a8"), 8)
        self.assertEqual(bot.move_to_coord("b1"), 10)
        self.assertEqual(bot.move_to_coord("h8"), 71)

        board_size = 26
        bot = RandomHexBot(WHITE, board_size=board_size)
        self.assertEqual(bot.move_to_coord("a1"), 1)
        self.assertEqual(bot.move_to_coord("f18"), 153)
        self.assertEqual(bot.move_to_coord("z26"), 701)

    def test_move_too_coord_failure(self):
        bot = RandomHexBot(WHITE, board_size=8)
        with self.assertRaises(AssertionError):
            bot.move_to_coord("")
        with self.assertRaises(AssertionError):
            bot.move_to_coord("a")
        with self.assertRaises(AssertionError):
            bot.move_to_coord("1")
        with self.assertRaises(AssertionError):
            bot.move_to_coord("1a")
        with self.assertRaises(AssertionError):
            bot.move_to_coord("a9")
        with self.assertRaises(AssertionError):
            bot.move_to_coord("i1")        

# class BotIntegrationTest(unittest.TestCase):

if __name__ == "__main__":
    unittest.main()