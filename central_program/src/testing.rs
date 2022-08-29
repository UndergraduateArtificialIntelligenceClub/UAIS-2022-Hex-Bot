use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
use std::path::PathBuf;
use termion::{color, style};

use super::Color;

pub struct BotTest {
    color: Color,
    bot_path: PathBuf,
}

impl BotTest {
    pub fn new(color: Color, bot_path: PathBuf) -> Self {
        Self { color, bot_path }
    }

    pub fn test(&mut self) {
        let (own_color, other_color) =
            if self.color == Color::Black { ("B", "W") } else { ("W", "B") };

        println!("Init board ==================================================");
        println!("`init_board {{n}}` must create an nxn blank board");
        test_init_board(self.get_bot(), 1);
        test_init_board(self.get_bot(), 2);
        test_init_board(self.get_bot(), 3);
        test_init_board(self.get_bot(), 8);
        test_init_board(self.get_bot(), 11);
        test_init_board(self.get_bot(), 26);

        println!("Set your tile ===============================================");
        println!("`sety {{coord}}` sets your tile, X, at the given coordinate");
        test_set_yours_a1(self.get_bot(), own_color);
        test_set_yours_c8(self.get_bot(), own_color);
        test_set_yours_all_rows(self.get_bot(), own_color);
        test_set_yours_fill(self.get_bot(), own_color);
        test_set_yours_twice_on_same_spot(self.get_bot(), own_color);
        test_set_yours_diagonal(self.get_bot(), own_color);
        test_set_yours_big_diagonal(self.get_bot(), own_color);
        test_set_yours_big_fill(self.get_bot(), own_color);

        println!("Set other player's tile =====================================");
        println!("`seto {{coord}}` sets the other player's tile, O, at the given coordinate");
        test_set_others_a1(self.get_bot(), other_color);
        test_set_others_c8(self.get_bot(), other_color);
        test_set_others_all_rows(self.get_bot(), other_color);
        test_set_others_fill(self.get_bot(), other_color);
        test_set_others_diagonal(self.get_bot(), other_color);
        test_set_others_twice_on_same_spot(self.get_bot(), other_color);
        test_set_others_big_diagonal(self.get_bot(), other_color);
        test_set_others_big_fill(self.get_bot(), other_color);

        println!("Unsetting tiles =============================================");
        println!("`unset {{coord}}` clears any tile at that coordinate");
        self.test_unset_tiles(own_color, other_color);

        println!("Checking for a win ==========================================");
        println!("`check_win` prints 1 if you've won, -1 if the opponent won, 0 otherwise");
        test_no_win_blank(self.get_bot(), 1);
        test_no_win_blank(self.get_bot(), 3);
        test_no_win_blank(self.get_bot(), 8);
        test_no_win_blank(self.get_bot(), 20);
        test_no_win_blank(self.get_bot(), 26);
        test_white_win_small(self.get_bot(), self.is_white());
        test_black_win_small(self.get_bot(), self.is_white());
        test_black_win_big(self.get_bot(), self.is_white());
    }

    // Starts up a new bot
    fn get_bot(&self) -> Child {
        Command::new(&self.bot_path)
            .arg(self.color_str())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn child process")
    }

    fn is_white(&self) -> bool {
        self.color == Color::White
    }

    fn color_str(&self) -> &str {
        if self.is_white() {
            "white"
        } else {
            "black"
        }
    }

    // `unset` has to be tested very manually...
    fn test_unset_tiles(&self, x: &str, o: &str) {
        let mut bot = self.get_bot();
        let bot_out = bot.stdout.as_mut().unwrap();
        let bot_in = bot.stdin.as_mut().unwrap();

        let mut reader = BufReader::new(bot_out);
        let mut real_out = String::new();

        write!(bot_in, "init_board 4\n")
            .expect("Unset tiles: Failed to init board");

        for coord in ["a1", "a2", "d3", "d4"] {
            write!(bot_in, "sety {}\n", coord)
                .expect("Unset tiles: Failed to sety");
        }

        for coord in ["d1", "d2", "a3", "a4"] {
            write!(bot_in, "seto {}\n", coord)
                .expect("Unset tiles: Failed to seto");
        }

        write!(bot_in, "unset a2\nshow_board\n")
            .expect("Unset tiles: Failed to show_board");

        reader.read_line(&mut real_out).expect("Unset tiles: Failed to output");

        pretty_print_result("Unset your a1",
            &"X.OO|....|....|OOXX|\n".replace("X", x).replace("O", o),
            &real_out);
        real_out.clear();

        write!(bot_in, "unset a3\nunset d2\nunset d3\nshow_board\n")
            .expect("Unset tiles: Failed to unset");

        reader.read_line(&mut real_out).expect("Unset tiles: Failed to output");

        pretty_print_result("Unset yours and opponent's tiles",
            &"X..O|....|....|O..X|\n".replace("X", x).replace("O", o)
            , &real_out);
        real_out.clear();

        write!(bot_in, "unset a1\nunset a4\nunset d1\nunset d4\nshow_board\n")
            .expect("Unset tiles: Failed to unset");

        reader.read_line(&mut real_out).expect("Unset tiles: Failed to output");

        pretty_print_result("Unset all tiles", "....|....|....|....|\n", &real_out);
        real_out.clear();

        write!(bot_in, "unset a1\nunset a4\nunset d1\nunset d4\nshow_board\n")
            .expect("Unset tiles: Failed to unset");

        reader.read_line(&mut real_out).expect("Unset tiles: Failed to output");

        pretty_print_result("Unset on empty tiles", "....|....|....|....|\n", &real_out);
        real_out.clear();

        write!(bot_in, "sety a1\nsety a4\nsety d1\nsety d4\ninit_board 4\nshow_board\n")
            .expect("Unset tiles: Failed init board");

        reader.read_line(&mut real_out).expect("Unset tiles: Failed to output");

        pretty_print_result("init_board unsets all tiles", "....|....|....|....|\n", &real_out);

    }
}

#[derive(Debug)]
struct Test<T, G>
where
    T: AsRef<str> + std::fmt::Display + std::fmt::Debug,
    G: AsRef<str> + std::fmt::Display + std::fmt::Debug
{
    pub name: G,
    pub bot: Child,
    pub board_size: u8,
    pub sety: Vec<T>,
    pub seto: Vec<T>,
    pub expected_out: String,
    pub real_out: String,
}

impl<T, G> Test<T, G>
where
    T: AsRef<str> + std::fmt::Display + std::fmt::Debug,
    G: AsRef<str> + std::fmt::Display + std::fmt::Debug
{
    pub fn run(mut self, cmd: &str) {
        self.setup_board();
        self.real_out = self.get_out(cmd);
        self.pretty_print();
    }

    // Pretty output for each testcase
    fn pretty_print(&self) {
        pretty_print_result(self.name.as_ref(), &self.expected_out, &self.real_out)
    }

    // Init board with tiles
    fn setup_board(&mut self) {
        let bot_in = self.bot.stdin.as_mut().unwrap();

        write!(bot_in, "init_board {}\n", self.board_size).expect("Failed to write init_board");

        for coord in self.sety.iter() { write!(bot_in, "sety {}\n", coord).expect("Failed to write sety") }
        for coord in self.seto.iter() { write!(bot_in, "seto {}\n", coord).expect("Failed to write seto") }
    }

    // Check output
    fn get_out(&mut self, cmd: &str) -> String {
        let bot_out = self.bot.stdout.as_mut().unwrap();
        let bot_in = self.bot.stdin.as_mut().unwrap();
        write!(bot_in, "{}\n", cmd).expect(&format!("Failed to write: {}", cmd));

        let mut reader = BufReader::new(bot_out);
        let mut output = String::new();

        reader.read_line(&mut output).unwrap();
        output.replace("\r\n", "\n").to_owned()  // DOS compatibility
    }
}

fn pretty_print_result(name: &str, expected_out: &str, real_out: &str) {
    if expected_out == real_out {
        println!("{}✓ {}... ok{}",
            color::Fg(color::Green),
            name,
            color::Fg(color::Reset),
        );
    } else {
        println!("{}{}✗ {}... FAILED{}{}",
            style::Bold,
            color::Fg(color::Red),
            name,
            color::Fg(color::Reset),
            style::Reset,
        );
        println!("{}EXPECTED ====================\n{}{}",
            color::Fg(color::Red), color::Fg(color::Reset), expected_out);
        println!("{}REAL ========================\n{}{}",
            color::Fg(color::Red), color::Fg(color::Reset), real_out);
        println!("{}============================={}",
            color::Fg(color::Red), color::Fg(color::Reset));
    }
}


// Init board =======================================================
// `init_board {{n}}` must create an nxn blank board

fn test_init_board(bot: Child, size: u8) {
    let mut test = Test::<String, String> {
        name: format!("Creates a {}x{} board", size, size),
        bot: bot,
        board_size: size,
        sety: vec![],
        seto: vec![],
        expected_out: String::new(), //"...|...|...|\n".to_string(),
        real_out: String::new(),
    };

    for _row in 1..=size {
        for _column in 1..=size {
            test.expected_out.push_str(".");
        }
        test.expected_out.push_str("|");
    }
    test.expected_out.push_str("\n");

    test.run("show_board")
}

// Set your tile ====================================================
// `sety {coord}` sets your tile at coordinate {coord}. Use X to represent your tiles Some examples
// of coordinates:
// a1 == (0,0)
// a2 == (0,1)
// c12 == (2,11)
// Only a single letter is allowed, followed by up to 2 digits. No need to implement anything over
// board size 26x26. Note that the coordinates ARE 1 indexed

fn test_set_yours_a1(bot: Child, c: &str) {
    let test = Test {
        name: "Sets own tile on a1",
        bot: bot,
        board_size: 3,
        sety: vec!["a1"],
        seto: vec![],
        expected_out: "X..|...|...|\n".replace("X", c).to_string(),
        real_out: String::new(),
    };

    test.run("show_board")
}

fn test_set_yours_c8(bot: Child, c: &str) {
    let test = Test {
        name: "Sets own tile on c8",
        bot: bot,
        board_size: 10,
        sety: vec!["c8"],
        seto: vec![],
        expected_out: "\
            ..........|\
            ..........|\
            .......X..|\
            ..........|\
            ..........|\
            ..........|\
            ..........|\
            ..........|\
            ..........|\
            ..........|\n".replace("X", c).to_string(),
        real_out: String::new(),
    };

    test.run("show_board")
}

fn test_set_yours_all_rows(bot: Child, c: &str) {
    let test = Test {
        name: "Sets own tiles on first column",
        bot: bot,
        board_size: 3,
        sety: vec!["a1", "b1", "c1"],
        seto: vec![],
        expected_out: "X..|X..|X..|\n".replace("X", c).to_string(),
        real_out: String::new(),
    };

    test.run("show_board")
}

fn test_set_yours_fill(bot: Child, c: &str) {
    let test = Test {
        name: "Sets own tiles on every spot",
        bot: bot,
        board_size: 3,
        sety: vec!["a1", "a2", "a3", "b1", "b2", "b3", "c1", "c2", "c3"],
        seto: vec![],
        expected_out: "XXX|XXX|XXX|\n".replace("X", c).to_string(),
        real_out: String::new(),
    };

    test.run("show_board")
}

fn test_set_yours_diagonal(bot: Child, c: &str) {
    let test = Test {
        name: "Sets own tiles diagonally",
        bot: bot,
        board_size: 3,
        sety: vec!["a1", "b2", "c3"],
        seto: vec![],
        expected_out: "X..|.X.|..X|\n".replace("X", c).to_string(),
        real_out: String::new(),
    };

    test.run("show_board")
}

fn test_set_yours_twice_on_same_spot(bot: Child, c: &str) {
    let test = Test {
        name: "Sets own twice on same spot",
        bot: bot,
        board_size: 3,
        sety: vec!["a1", "a1", "c3"],
        seto: vec![],
        expected_out: "X..|...|..X|\n".replace("X", c).to_string(),
        real_out: String::new(),
    };

    test.run("show_board")
}

fn test_set_yours_big_diagonal(bot: Child, c: &str) {
    let test = Test {
        name: "Sets own tiles diagonally on a large board",
        bot: bot,
        board_size: 12,
        sety: vec!["a1", "b2", "c3", "d4", "e5", "f6", "g7", "h8", "i9", "j10", "k11", "l12"],
        seto: vec![],
        expected_out: "\
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
            ...........X|\n".replace("X", c).to_string(),
        real_out: String::new(),
    };

    test.run("show_board")
}

fn test_set_yours_big_fill(bot: Child, c: &str) {
    let mut test = Test::<String, &str> {
        name: "Sets own tiles on every spot on a large board",
        bot: bot,
        board_size: 12,
        sety: vec![],
        seto: vec![],
        expected_out: "XXXXXXXXXXXX|".replace("X", c).repeat(12) + "\n",
        real_out: String::new(),
    };

    for letter in ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'].iter() {
        for number in 1..=12 {
            test.sety.push(format!("{}{}", letter, number));
        }
    }

    test.run("show_board");
}

// Set other player's tile ==========================================
// `seto {coord}` sets the other player's tile at coordinate {coord}. Use O to represent the other
// player

fn test_set_others_a1(bot: Child, c: &str) {
    let test = Test {
        name: "Sets own tile on a1",
        bot: bot,
        board_size: 3,
        sety: vec![],
        seto: vec!["a1"],
        expected_out: "O..|...|...|\n".replace("O", c).to_string(),
        real_out: String::new(),
    };

    test.run("show_board")
}

fn test_set_others_c8(bot: Child, c: &str) {
    let test = Test {
        name: "Sets own tile on c8",
        bot: bot,
        board_size: 10,
        sety: vec![],
        seto: vec!["c8"],
        expected_out: "\
            ..........|\
            ..........|\
            .......O..|\
            ..........|\
            ..........|\
            ..........|\
            ..........|\
            ..........|\
            ..........|\
            ..........|\n".replace("O", c).to_string(),
        real_out: String::new(),
    };

    test.run("show_board")
}

fn test_set_others_all_rows(bot: Child, c: &str) {
    let test = Test {
        name: "Sets own tiles on first column",
        bot: bot,
        board_size: 3,
        sety: vec![],
        seto: vec!["a1", "b1", "c1"],
        expected_out: "O..|O..|O..|\n".replace("O", c).to_string(),
        real_out: String::new(),
    };

    test.run("show_board")
}

fn test_set_others_fill(bot: Child, c: &str) {
    let test = Test {
        name: "Sets own tiles on every spot",
        bot: bot,
        board_size: 3,
        sety: vec![],
        seto: vec!["a1", "a2", "a3", "b1", "b2", "b3", "c1", "c2", "c3"],
        expected_out: "OOO|OOO|OOO|\n".replace("O", c).to_string(),
        real_out: String::new(),
    };

    test.run("show_board")
}

fn test_set_others_diagonal(bot: Child, c: &str) {
    let test = Test {
        name: "Sets own tiles diagonally",
        bot: bot,
        board_size: 3,
        sety: vec![],
        seto: vec!["a1", "b2", "c3"],
        expected_out: "O..|.O.|..O|\n".replace("O", c).to_string(),
        real_out: String::new(),
    };

    test.run("show_board")
}

fn test_set_others_twice_on_same_spot(bot: Child, c: &str) {
    let test = Test {
        name: "Sets own twice on same spot",
        bot: bot,
        board_size: 3,
        sety: vec![],
        seto: vec!["a1", "a1", "c3"],
        expected_out: "O..|...|..O|\n".replace("O", c).to_string(),
        real_out: String::new(),
    };

    test.run("show_board")
}

fn test_set_others_big_diagonal(bot: Child, c: &str) {
    let test = Test {
        name: "Sets own tiles diagonally on a large board",
        bot: bot,
        board_size: 12,
        sety: vec![],
        seto: vec!["a1", "b2", "c3", "d4", "e5", "f6", "g7", "h8", "i9", "j10", "k11", "l12"],
        expected_out: "\
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
            ...........O|\n".replace("O", c).to_string(),
        real_out: String::new(),
    };

    test.run("show_board")
}

fn test_set_others_big_fill(bot: Child, c: &str) {
    let mut test = Test::<String, &str> {
        name: "Sets own tiles on every spot on a large board",
        bot: bot,
        board_size: 12,
        sety: vec![],
        seto: vec![],
        expected_out: "OOOOOOOOOOOO|".replace("O", c).repeat(12) + "\n",
        real_out: String::new(),
    };

    for letter in ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'].iter() {
        for number in 1..=12 {
            test.seto.push(format!("{}{}", letter, number));
        }
    }

    test.run("show_board");
}

// Check for identifying win ========================================
// `check_win` asks the bot if any player has won. Print one of the following:
//  1 - Your bot has won
// -1 - The other bot has won
//  0 - No bot has won yet
// All of these tests assume `sety` and `seto` work properly

fn test_no_win_blank(bot: Child, size: u8) {
    let test = Test::<String, String> {
        name: format!("No win on blank {}x{} board", size, size),
        bot: bot,
        board_size: size,
        sety: vec![],
        seto: vec![],
        expected_out: "0\n".to_string(),
        real_out: String::new(),
    };

    test.run("check_win");
}

fn test_white_win_small(bot: Child, is_white: bool) {
    let mut test = Test {
        name: "Identifies white win across one (top) row",
        bot: bot,
        board_size: 3,
        sety: vec!["a1", "a2", "a3"],
        seto: vec![],
        expected_out: format!("{}\n", if is_white { 1 } else { -1 }),
        real_out: String::new(),
    };

    if !is_white { std::mem::swap(&mut test.sety, &mut test.seto) }

    test.run("check_win");
}

fn test_black_win_small(bot: Child, is_white: bool) {
    let mut test = Test {
        name: "Identifies white win across one (left) column",
        bot: bot,
        board_size: 3,
        sety: vec!["a1", "b1", "c1"],
        seto: vec![],
        expected_out: format!("{}\n", if !is_white { 1 } else { -1 }),
        real_out: String::new(),
    };

    if is_white { std::mem::swap(&mut test.sety, &mut test.seto) }

    test.run("check_win");
}

fn test_black_win_big(bot: Child, is_white: bool) {
    let mut test = Test {
        name: "Identifies black win on a big board",
        bot: bot,
        board_size: 11,
        sety: vec!["a3", "b3", "c3", "c4", "b5", "b6", "b7", "b8", "b9", "b10", "b11", "c11",
            "d11", "e11", "f10", "g9", "h9", "i8", "j8", "j9", "j10", "j11", "k11"],
        seto: vec![],
        expected_out: format!("{}\n", if !is_white { 1 } else { -1 }),
        real_out: String::new(),
    };

    if is_white { std::mem::swap(&mut test.sety, &mut test.seto) }

    test.run("check_win");
}

// TODO: Test "swap" command
//  - Swap as your move
//  - Swap as opponent's move
//  - Swap as your move at the wrong time
//  - Swap as opponent's move at the wrong time
//
// TODO: Test invalid coordinates
// Behaviour for invalid coodrdinates is currently undefined, though we probably shouldn't be
// crashing the program from an invalid input. Maybe some sort of output to signal the input wasn't
// read?

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
