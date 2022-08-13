// show_board is assumed to work for all these tests... Otherwise there's not way to verify tests.
// If the init_board tests are failing, show_board is likely the culprit
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio, ChildStdout};

fn get_bot() -> io::Result<Child> {
    let bot_1 = Command::new("python3")
        .arg("../random_bot/main.py")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    Ok(bot_1)
}

fn shutdown_bot(bot: &mut Child) -> io::Result<()> {
    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "quit\n")?;
    bot.wait()?;
    Ok(())
}

fn get_command_output(stdout: &mut ChildStdout) -> String {
    let mut reader = BufReader::new(stdout);
    let mut output = String::new();

    reader.read_line(&mut output).unwrap();
    output.replace("\r\n", "\n")  // DOS compatibility
          .trim()
          .to_owned()
}

// Tests a blank board at given size
fn test_init_board(size: usize) -> io::Result<()> {
    let mut bot = create_board::<&str>(size, &[], &[])?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "show_board\n")?;
    let mut stdout = bot.stdout.take().unwrap();
    let output = get_command_output(&mut stdout);

    let mut expected = String::new();
    for _ in 0..size {
        for _ in 0..size {
            expected.push_str(".")
        }
        expected.push_str("|")
    }

    assert_eq!(output, expected);

    shutdown_bot(&mut bot)?;
    Ok(())
}

// Creates bot and sets up board. Requires most commands work
fn create_board<T>(board_size: usize, sety: &[T], seto: &[T]) -> io::Result<Child>
where
    T: AsRef<str> + std::fmt::Display
{
    let mut bot = get_bot()?;
    let bot_in = bot.stdin.as_mut().unwrap();

    write!(bot_in, "init_board {}\n", board_size)?;

    for coord in sety.iter() { write!(bot_in, "sety {}\n", coord)? }
    for coord in seto.iter() { write!(bot_in, "seto {}\n", coord)? }

    Ok(bot)
}

// Sets a mix of your and other player's tiles
fn assert_set<T>(board_size: usize, sety: &[T], seto: &[T], expected_out: &str) -> io::Result<()>
where
    T: AsRef<str> + std::fmt::Display
{
    let mut bot = create_board(board_size, sety, seto)?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "show_board\n")?;

    let output = get_command_output(bot.stdout.as_mut().unwrap());

    assert_eq!(output.as_str(), expected_out);

    shutdown_bot(&mut bot)?;
    Ok(())
}

fn test_sety<T>(board_size: usize, sety: &[T], expected_out: &str) -> io::Result<()>
where
    T: AsRef<str> + std::fmt::Display
{
    assert_set(board_size, sety, &[], expected_out)
}

fn test_seto<T>(board_size: usize, seto: &[T], expected_out: &str) -> io::Result<()>
where
    T: AsRef<str> + std::fmt::Display
{
    assert_set(board_size, &[], seto, expected_out)
}

fn assert_win<T>(board_size: usize, sety: &[T], seto: &[T], expected: &str) -> io::Result<()>
where
    T: AsRef<str> + std::fmt::Display
{
    let mut bot = create_board(board_size, sety, seto)?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "check_win\n")?;

    let output = get_command_output(bot.stdout.as_mut().unwrap());

    assert_eq!(expected, output);
    Ok(())
}

// Init board =======================================================
// `init_board {n}` must create an nxn blank board

#[test]
fn test_init_board_1() -> io::Result<()> { test_init_board(1) }
#[test]
fn test_init_board_2() -> io::Result<()> { test_init_board(2) }
#[test]
fn test_init_board_3() -> io::Result<()> { test_init_board(3) }
#[test]
fn test_init_board_8() -> io::Result<()> { test_init_board(8) }
#[test]
fn test_init_board_11() -> io::Result<()> { test_init_board(11) }
#[test]
fn test_init_board_26() -> io::Result<()> { test_init_board(26) }

// Set your tile ====================================================
// `sety {coord}` sets your tile at coordinate {coord}. Use X to represent your tiles Some examples
// of coordinates:
// a1 == (0,0)
// a2 == (0,1)
// c12 == (2,11)
// Only a single letter is allowed, followed by up to 2 digits. No need to implement anything over
// board size 26x26. Note that the coordinates ARE 1 indexed

#[test]
fn test_set_yours_a1() -> io::Result<()> {
    let board_size = 3;
    let set_coords = ["a1"];
    let expected_out = "X..|...|...|";

    test_sety(board_size, &set_coords, &expected_out)
}

#[test]
fn test_set_yours_c8() -> io::Result<()> {
    let board_size = 10;
    let set_coords = ["c8"];
    let expected_out = "\
        ..........|\
        ..........|\
        .......X..|\
        ..........|\
        ..........|\
        ..........|\
        ..........|\
        ..........|\
        ..........|\
        ..........|\
    ";

    test_sety(board_size, &set_coords, &expected_out)
}

#[test]
fn test_set_yours_rows() -> io::Result<()> {
    let board_size = 3;
    let set_coords = ["a1", "b1", "c1"];
    let expected_out = "X..|X..|X..|";

    test_sety(board_size, &set_coords, &expected_out)
}

#[test]
fn test_set_yours_fill() -> io::Result<()> {
    let board_size = 3;
    let set_coords = ["a1", "a2", "a3", "b1", "b2", "b3", "c1", "c2", "c3"];
    let expected_out = "XXX|XXX|XXX|";

    test_sety(board_size, &set_coords, &expected_out)
}

#[test]
fn test_set_yours_diagonal() -> io::Result<()> {
    let board_size = 3;
    let set_coords = ["a1", "b2", "c3"];
    let expected_out = "X..|.X.|..X|";

    test_sety(board_size, &set_coords, &expected_out)
}

#[test]
fn test_set_yours_twice_on_same_spot() -> io::Result<()> {
    let board_size = 3;
    let set_coords = ["a1", "a1", "c3"];
    let expected_out = "X..|...|..X|";

    test_sety(board_size, &set_coords, &expected_out)
}

#[test]
fn test_set_yours_big_diagonal() -> io::Result<()> {
    let board_size = 12;
    let set_coords = ["a1", "b2", "c3", "d4", "e5", "f6", "g7", "h8", "i9", "j10", "k11", "l12"];
    let expected_out = "\
        X...........|\
        .X..........|\
        ..X.........|\
        ...X........|\
        ....X.......|\
        .....X......|\
        ......X.....|\
        .......X....|\
        ........X...|\
        .........X..|\
        ..........X.|\
        ...........X|\
    ";

    test_sety(board_size, &set_coords, &expected_out)
}

#[test]
fn test_set_yours_big_fill() -> io::Result<()> {
    let board_size = 12;
    let mut set_coords = Vec::new();

    for letter in ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'].iter() {
        for number in 1..=12 {
            set_coords.push(format!("{}{}", letter, number));
        }
    }

    let expected_out = "XXXXXXXXXXXX|".repeat(12);
    test_sety(board_size, &set_coords, &expected_out)
}

// Set other player's tile ==========================================
// `seto {coord}` sets the other player's tile at coordinate {coord}. Use O to represent the other
// player

#[test]
fn test_set_others_a1() -> io::Result<()> {
    let board_size = 3;
    let set_coords = ["a1"];
    let expected_out = "O..|...|...|";

    test_seto(board_size, &set_coords, &expected_out)
}

#[test]
fn test_set_others_c8() -> io::Result<()> {
    let board_size = 10;
    let set_coords = ["c8"];
    let expected_out = "\
        ..........|\
        ..........|\
        .......O..|\
        ..........|\
        ..........|\
        ..........|\
        ..........|\
        ..........|\
        ..........|\
        ..........|\
    ";

    test_seto(board_size, &set_coords, &expected_out)
}

#[test]
fn test_set_others_rows() -> io::Result<()> {
    let board_size = 3;
    let set_coords = ["a1", "b1", "c1"];
    let expected_out = "O..|O..|O..|";

    test_seto(board_size, &set_coords, &expected_out)
}

#[test]
fn test_set_others_fill() -> io::Result<()> {
    let board_size = 3;
    let set_coords = ["a1", "a2", "a3", "b1", "b2", "b3", "c1", "c2", "c3"];
    let expected_out = "OOO|OOO|OOO|";

    test_seto(board_size, &set_coords, &expected_out)
}

#[test]
fn test_set_others_diagonal() -> io::Result<()> {
    let board_size = 3;
    let set_coords = ["a1", "b2", "c3"];
    let expected_out = "O..|.O.|..O|";

    test_seto(board_size, &set_coords, &expected_out)
}

#[test]
fn test_set_others_twice_on_same_spot() -> io::Result<()> {
    let board_size = 3;
    let set_coords = ["a1", "a1", "c3"];
    let expected_out = "O..|...|..O|";

    test_seto(board_size, &set_coords, &expected_out)
}

#[test]
fn test_set_others_big_diagonal() -> io::Result<()> {
    let board_size = 12;
    let set_coords = ["a1", "b2", "c3", "d4", "e5", "f6", "g7", "h8", "i9", "j10", "k11", "l12"];
    let expected_out = "\
        O...........|\
        .O..........|\
        ..O.........|\
        ...O........|\
        ....O.......|\
        .....O......|\
        ......O.....|\
        .......O....|\
        ........O...|\
        .........O..|\
        ..........O.|\
        ...........O|\
    ";

    test_seto(board_size, &set_coords, &expected_out)
}

#[test]
fn test_set_others_big_fill() -> io::Result<()> {
    let board_size = 12;
    let mut set_coords = Vec::new();

    for letter in ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'].iter() {
        for number in 1..=12 {
            set_coords.push(format!("{}{}", letter, number));
        }
    }

    let expected_out = "OOOOOOOOOOOO|".repeat(12);
    test_seto(board_size, &set_coords, &expected_out)
}

// Unsetting tiles ==================================================
// `unset {coord}` clears the tile in a spot. Mostly for debugging, easy to implement
#[test]
fn test_unset_tiles() -> io::Result<()> {
    let board_size = 4;
    let sety = ["a1", "a2", "d3", "d4"];
    let seto = ["d1", "d2", "a3", "a4"];

    let mut bot = create_board(board_size, &sety, &seto)?;
    let bot_in = bot.stdin.as_mut().unwrap();

    write!(bot_in, "show_board\n")?;
    let output = get_command_output(bot.stdout.as_mut().unwrap());
    assert_eq!(output.as_str(), "XXOO|....|....|OOXX|");

    for coord in ["a2", "a3", "d2", "d3"].iter() {
        write!(bot_in, "unset {}\n", coord)?;
    }
    write!(bot_in, "show_board\n")?;
    let output = get_command_output(bot.stdout.as_mut().unwrap());
    assert_eq!(output.as_str(), "X..O|....|....|O..X|");

    for coord in ["a1", "a4", "d1", "d4"].iter() {
        write!(bot_in, "unset {}\n", coord)?;
    }
    write!(bot_in, "show_board\n")?;
    let output = get_command_output(bot.stdout.as_mut().unwrap());
    assert_eq!(output.as_str(), "....|....|....|....|");

    Ok(())
}

#[test]
fn test_unset_leave_others_untouched() -> io::Result<()> {
    let board_size = 3;
    let sety = ["a1", "a2", "c2", "c3"];
    let seto = ["a3", "b1", "b2", "b3", "c1"];

    let mut bot = create_board(board_size, &sety, &seto)?;
    let bot_in = bot.stdin.as_mut().unwrap();

    write!(bot_in, "show_board\n")?;
    let output = get_command_output(bot.stdout.as_mut().unwrap());
    assert_eq!(output.as_str(), "XXO|OOO|OXX|");

    write!(bot_in, "unset b2\nshow_board\n")?;
    let output = get_command_output(bot.stdout.as_mut().unwrap());
    assert_eq!(output.as_str(), "XXO|O.O|OXX|");

    write!(bot_in, "unset a2\nshow_board\n")?;
    let output = get_command_output(bot.stdout.as_mut().unwrap());
    assert_eq!(output.as_str(), "X.O|O.O|OXX|");

    write!(bot_in, "unset c3\nshow_board\n")?;
    let output = get_command_output(bot.stdout.as_mut().unwrap());
    assert_eq!(output.as_str(), "X.O|O.O|OX.|");

    write!(bot_in, "unset b1\nshow_board\n")?;
    let output = get_command_output(bot.stdout.as_mut().unwrap());
    assert_eq!(output.as_str(), "X.O|..O|OX.|");

    Ok(())
}

#[test]
fn test_unset_on_empty_cell() -> io::Result<()> {
    let board_size = 3;
    let sety = ["a1"];
    let seto = ["c3"];

    let mut bot = create_board(board_size, &sety, &seto)?;
    let bot_in = bot.stdin.as_mut().unwrap();

    write!(bot_in, "show_board\n")?;
    let output = get_command_output(bot.stdout.as_mut().unwrap());
    assert_eq!(output.as_str(), "X..|...|..O|");

    write!(bot_in, "unset a1\n")?;
    write!(bot_in, "unset a1\n")?;
    write!(bot_in, "unset c3\n")?;
    write!(bot_in, "unset c3\n")?;
    write!(bot_in, "unset c1\n")?;

    write!(bot_in, "show_board\n")?;
    let output = get_command_output(bot.stdout.as_mut().unwrap());
    assert_eq!(output.as_str(), "...|...|...|");

    Ok(())
}


#[test]
// `init_board` should clear the board of all tiles
fn test_init_clear() -> io::Result<()> {
    let mut bot = create_board(4, &["a1", "a2"], &["b1", "b2"])?;
    let bot_in = bot.stdin.as_mut().unwrap();

    write!(bot_in, "show_board\n")?;
    let output = get_command_output(bot.stdout.as_mut().unwrap());
    assert_eq!(output.as_str(), "XX..|OO..|....|....|");

    write!(bot_in, "init_board 4\nshow_board\n")?;
    let output = get_command_output(bot.stdout.as_mut().unwrap());
    assert_eq!(output.as_str(), "....|....|....|....|");

    write!(bot_in, "sety a3\nshow_board\n")?;
    let output = get_command_output(bot.stdout.as_mut().unwrap());
    assert_eq!(output.as_str(), "..X.|....|....|....|");

    write!(bot_in, "init_board 26\nshow_board\n")?;
    let output = get_command_output(bot.stdout.as_mut().unwrap());
    assert_eq!(output.as_str(), "..........................|".repeat(26));

    Ok(())
}

// Check for identifying win ========================================
// `check_win` asks the bot if any player has won. Print one of the following:
//  1 - Your bot has won
// -1 - The other bot has won
//  0 - No bot has won yet
// All of these tests assume `sety` and `seto` work properly

#[test]
fn test_no_win_blank() -> io::Result<()> {
    assert_win::<&str>(1, &[], &[], "0")?;
    assert_win::<&str>(3, &[], &[], "0")?;
    assert_win::<&str>(8, &[], &[], "0")?;
    assert_win::<&str>(20, &[], &[], "0")?;
    assert_win::<&str>(26, &[], &[], "0")
}

#[test]
fn test_row_win_yours_small() -> io::Result<()> {
    let size = 3;
    let sety = ["a1", "a2", "a3"];
    let expected = "1";

    assert_win(size, &sety, &[], expected)
}

#[test]
fn test_column_win_others_small() -> io::Result<()> {
    let size = 3;
    let seto = ["a1", "b1", "c1"];
    let expected = "-1";

    assert_win(size, &[], &seto, expected)
}

// TODO: This test is failing, despite being manually verified. Random bot is wrong
#[test]
fn test_win_others_big() -> io::Result<()> {
    let size = 11;
    let sety = [];
    let seto = ["a3", "b3", "c3", "c4", "b5", "b6", "b7", "b8", "b9", "b10", "b11", "c11", "d11",
        "e11", "f10", "g9", "h9", "i8", "j8", "j9", "j10", "j11", "k11"];
    let expected = "1";

    assert_win(size, &sety, &seto, expected)
}

// TODO: Test "swap" command
//  - Swap as your move
//  - Swap as opponent's move
//  - Swap as your move at the wrong time
//  - Swap as opponent's move at the wrong time
//
// TODO: Test invalid coordinates
// Behaviour for invalid coodrdinates is currently undefined, though we probably should be crashing
// the program from an invalid input. Maybe some sort of output to signal the input wasn't read?

// TODO: Fix all tests below
//
//#[test]
//fn test_empty_show_board() -> io::Result<()> {
//    let mut bot = get_bot()?;
//
//    let bot_in = bot.stdin.as_mut().unwrap();
//    write!(bot_in, "show_board\n")?;
//    write!(bot_in, "show_board\n")?;
//    let mut stdout = bot.stdout.take().unwrap();
//    let output = get_command_output(&mut stdout, 0);
//
//    assert_eq!(
//        output.as_str().trim_start(),
//        [ ". . . . . . . . \n",
//          " . . . . . . . . \n",
//          "  . . . . . . . . \n",
//          "   . . . . . . . . \n",
//          "    . . . . . . . . \n",
//          "     . . . . . . . . \n",
//          "      . . . . . . . . \n",
//          "       . . . . . . . . \n",
//          "= \n"].concat()
//    );
//
//    shutdown_bot(&mut bot)?;
//    Ok(())
//}
//
//// test play on opponents spot
//#[test]
//fn test_sety_on_opponents_spot() -> io::Result<()> {
//    let mut bot = get_bot()?;
//
//    let bot_in = bot.stdin.as_mut().unwrap();
//    write!(bot_in, "seto a1\n")?;
//    write!(bot_in, "sety a1\n")?;
//    write!(bot_in, "show_board\n")?;
//    let mut stdout = bot.stdout.take().unwrap();
//    let output = get_command_output(&mut stdout, 2);
//
//    assert_eq!(
//        output.as_str().trim_start(),
//        [ "B . . . . . . . \n",
//          " . . . . . . . . \n",
//          "  . . . . . . . . \n",
//          "   . . . . . . . . \n",
//          "    . . . . . . . . \n",
//          "     . . . . . . . . \n",
//          "      . . . . . . . . \n",
//          "       . . . . . . . . \n",
//          "= \n"].concat()
//    );
//
//    shutdown_bot(&mut bot)?;
//    Ok(())
//}
//
//// test seto
//#[test]
//fn test_seto_on_opponents_spot() -> io::Result<()> {
//    let mut bot = get_bot()?;
//
//    let bot_in = bot.stdin.as_mut().unwrap();
//    write!(bot_in, "sety a1\n")?;
//    write!(bot_in, "seto a1\n")?;
//    write!(bot_in, "show_board\n")?;
//    let mut stdout = bot.stdout.take().unwrap();
//    let output = get_command_output(&mut stdout, 2);
//
//    assert_eq!(
//        output.as_str().trim_start(),
//        [ "W . . . . . . . \n",
//          " . . . . . . . . \n",
//          "  . . . . . . . . \n",
//          "   . . . . . . . . \n",
//          "    . . . . . . . . \n",
//          "     . . . . . . . . \n",
//          "      . . . . . . . . \n",
//          "       . . . . . . . . \n",
//          "= \n"].concat()
//    );
//
//    shutdown_bot(&mut bot)?;
//    Ok(())
//}
//
//// test 'swap'
//#[test]
//fn test_own_swap() -> io::Result<()> {
//    let mut bot = get_bot()?;
//
//    let bot_in = bot.stdin.as_mut().unwrap();
//    write!(bot_in, "seto a1\n")?;
//    write!(bot_in, "sety swap\n")?;
//    write!(bot_in, "seto a2\n")?;
//    write!(bot_in, "show_board\n")?;
//    let mut stdout = bot.stdout.take().unwrap();
//    let output = get_command_output(&mut stdout, 3);
//
//    assert_eq!(
//        output.as_str().trim_start(),
//        [ "B W . . . . . . \n",
//          " . . . . . . . . \n",
//          "  . . . . . . . . \n",
//          "   . . . . . . . . \n",
//          "    . . . . . . . . \n",
//          "     . . . . . . . . \n",
//          "      . . . . . . . . \n",
//          "       . . . . . . . . \n",
//          "= \n"].concat()
//    );
//
//    shutdown_bot(&mut bot)?;
//    Ok(())
//}
//
//#[test]
//fn test_opponent_swap() -> io::Result<()> {
//    let mut bot = get_bot()?;
//
//    let bot_in = bot.stdin.as_mut().unwrap();
//    write!(bot_in, "sety a1\n")?;
//    write!(bot_in, "seto swap\n")?;
//    write!(bot_in, "sety a2\n")?;
//    write!(bot_in, "show_board\n")?;
//    let mut stdout = bot.stdout.take().unwrap();
//    let output = get_command_output(&mut stdout, 3);
//
//    assert_eq!(
//        output.as_str().trim_start(),
//        [ "W B . . . . . . \n",
//          " . . . . . . . . \n",
//          "  . . . . . . . . \n",
//          "   . . . . . . . . \n",
//          "    . . . . . . . . \n",
//          "     . . . . . . . . \n",
//          "      . . . . . . . . \n",
//          "       . . . . . . . . \n",
//          "= \n"].concat()
//    );
//
//    shutdown_bot(&mut bot)?;
//    Ok(())
//}
//
//#[test]
//fn test_own_swap_at_wrong_time() -> io::Result<()> {
//    let mut bot = get_bot()?;
//
//    let bot_in = bot.stdin.as_mut().unwrap();
//    write!(bot_in, "sety a1\n")?;
//    write!(bot_in, "seto a2\n")?;
//    write!(bot_in, "sety swap\n")?;
//    write!(bot_in, "seto a3\n")?;
//    write!(bot_in, "show_board\n")?;
//    let mut stdout = bot.stdout.take().unwrap();
//    let output = get_command_output(&mut stdout, 4);
//
//    assert_eq!(
//        output.as_str().trim_start(),
//        [ "W B B . . . . . \n",
//          " . . . . . . . . \n",
//          "  . . . . . . . . \n",
//          "   . . . . . . . . \n",
//          "    . . . . . . . . \n",
//          "     . . . . . . . . \n",
//          "      . . . . . . . . \n",
//          "       . . . . . . . . \n",
//          "= \n"].concat()
//    );
//
//    shutdown_bot(&mut bot)?;
//    Ok(())
//}
//
//#[test]
//fn test_opponent_swap_at_wrong_time() -> io::Result<()> {
//    let mut bot = get_bot()?;
//
//    let bot_in = bot.stdin.as_mut().unwrap();
//    write!(bot_in, "seto a1\n")?;
//    write!(bot_in, "sety a2\n")?;
//    write!(bot_in, "seto swap\n")?;
//    write!(bot_in, "sety a3\n")?;
//    write!(bot_in, "show_board\n")?;
//    let mut stdout = bot.stdout.take().unwrap();
//    let output = get_command_output(&mut stdout, 4);
//
//    assert_eq!(
//        output.as_str().trim_start(),
//        [ "B W W . . . . . \n",
//          " . . . . . . . . \n",
//          "  . . . . . . . . \n",
//          "   . . . . . . . . \n",
//          "    . . . . . . . . \n",
//          "     . . . . . . . . \n",
//          "      . . . . . . . . \n",
//          "       . . . . . . . . \n",
//          "= \n"].concat()
//    );
//
//    shutdown_bot(&mut bot)?;
//    Ok(())
//}
//
//// test basic win conditions
//#[test]
//fn test_basic_wins_with_white() -> io::Result<()> {
//    let mut bot = get_bot()?;
//
//    let bot_in = bot.stdin.as_mut().unwrap();
//    // top-level win
//    write!(bot_in, "sety a1\n")?;
//    write!(bot_in, "sety a2\n")?;
//    write!(bot_in, "sety a3\n")?;
//    write!(bot_in, "sety a4\n")?;
//    write!(bot_in, "sety a5\n")?;
//    write!(bot_in, "sety a6\n")?;
//    write!(bot_in, "sety a7\n")?;
//    write!(bot_in, "sety a8\n")?;
//    write!(bot_in, "check_win\n")?;
//
//    // bottom-row win
//    write!(bot_in, "init_board 8\n")?;
//    write!(bot_in, "sety h1\n")?;
//    write!(bot_in, "sety h2\n")?;
//    write!(bot_in, "sety h3\n")?;
//    write!(bot_in, "sety h4\n")?;
//    write!(bot_in, "sety h5\n")?;
//    write!(bot_in, "sety h6\n")?;
//    write!(bot_in, "sety h7\n")?;
//    write!(bot_in, "sety h8\n")?;
//    write!(bot_in, "check_win\n")?;
//
//    // top-left -> bottom-right diag
//    write!(bot_in, "init_board 8\n")?;
//    write!(bot_in, "sety a1\n")?;
//    write!(bot_in, "sety b1\n")?;
//    write!(bot_in, "sety b2\n")?;
//    write!(bot_in, "sety c2\n")?;
//    write!(bot_in, "sety c3\n")?;
//    write!(bot_in, "sety d3\n")?;
//    write!(bot_in, "sety d4\n")?;
//    write!(bot_in, "sety e4\n")?;
//    write!(bot_in, "sety e5\n")?;
//    write!(bot_in, "sety f5\n")?;
//    write!(bot_in, "sety f6\n")?;
//    write!(bot_in, "sety g6\n")?;
//    write!(bot_in, "sety g7\n")?;
//    write!(bot_in, "sety h7\n")?;
//    write!(bot_in, "sety h8\n")?;
//    write!(bot_in, "check_win\n")?;
//
//    // bottom-left -> top-right diag
//    write!(bot_in, "init_board 8\n")?;
//    write!(bot_in, "sety h1\n")?;
//    write!(bot_in, "sety g2\n")?;
//    write!(bot_in, "sety f3\n")?;
//    write!(bot_in, "sety e4\n")?;
//    write!(bot_in, "sety d5\n")?;
//    write!(bot_in, "sety c6\n")?;
//    write!(bot_in, "sety b7\n")?;
//    write!(bot_in, "sety a8\n")?;
//    write!(bot_in, "check_win\n")?;
//
//    let mut stdout = bot.stdout.take().unwrap();
//    let mut output = get_command_output(&mut stdout, 8);
//    assert_eq!( output.as_str().trim_start(), "1\n= \n");
//
//    output = get_command_output(&mut stdout, 9);
//    assert_eq!( output.as_str().trim_start(), "1\n= \n");
//
//    output = get_command_output(&mut stdout, 16);
//    assert_eq!( output.as_str().trim_start(), "1\n= \n");
//
//    output = get_command_output(&mut stdout, 9);
//    assert_eq!( output.as_str().trim_start(), "1\n= \n");
//
//    shutdown_bot(&mut bot)?;
//    Ok(())
//}
//
//// test different non-terminal conditions
//// test different win conditions
//// test different loss conditions
