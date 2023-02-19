// This program is written with unix in mind. No clue what'll happen on windows...
mod board;
mod testing;

use testing::BotTest;

use std::process::{self, Command, Stdio, Child};
use std::io::{self, BufRead, BufReader, Write};
use board::{Board, Tile};

use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "UAIS Sentience Validator")]
#[clap(author = "Undergraduate Artifical Intelligence Society <https://uais.dev>")]
#[clap(version = "0.1.1")]
#[clap(about = "Test and match up hex bots", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run all tests on a bot
    Test {
        /// Path to the main executable for the bot
        #[clap(action)]
        bot_path: PathBuf,
        /// Color of bot on this test
        #[clap(action)]
        color: Color,
    },
    /// Let two bots face off in hex
    Matchup {
        /// The size of the board squared. Ex: 11
        #[clap(action)]
        size: u8,
        /// Path to the main executable for the black bot (top->bottom)
        #[clap(action)]
        black_bot: PathBuf,
        /// Path to the main executable for the white bot (left->right)
        #[clap(action)]
        white_bot: PathBuf,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, clap::ValueEnum)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Black => "B",
            Self::White => "W",
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Test { bot_path, color } => {
            let mut bot_test = BotTest::new(color, bot_path);
            bot_test.test();
        }
        Commands::Matchup { size, black_bot, white_bot } => {
            let black = spawn_bot(black_bot, "black");
            let white = spawn_bot(white_bot, "white");
            print_repl_help();
            run_match(size, black, white);
        }
    }
}

fn print_repl_help() {
    println!("{}", [
        "==== Sentience Validator: Interactive REPL ====",
        "Command      Description",
        "h | help     Prints this help menu",
        "n | next     Prompts the bot for its next move",
        "run {}       Plays {} turns sequentially",
        "s | show     Shows the central board",
        "S | showall  Shows both the bots' boards and the central one",
        "c | check    Checks if a bot has won",
        "exit | quit  Shuts down both bots and exits",
        "===============================================",
    ].join("\n"));
}

fn run_match(size: u8, mut black: Child, mut white: Child) {
    let mut board = Board::new(size);
    let mut is_black_turn = true;

    init_board(size, &mut black, &mut white);

    let stdin = io::stdin();

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        if "help" == line || "h" == line {
            print_repl_help();
        } else if "check" == line || "c" == line {
            match board.has_win() {
                Tile::Empty => println!("Nobody's won... yet"),
                color => println!("{} has won!", color),
            }
        } else if "show" == line || "s" == line {
            println!("{}", board);
        } else if "showall" == line || "S" == line {
            println!("{}\nCentral board ----------------\n{}", "=".repeat(20), board);
            print_bot_board(&mut black, Tile::Black);
            print_bot_board(&mut white, Tile::White);
            println!("{}", "=".repeat(20));
        } else if "exit" == line || "quit" == line {
            println!("Shutting down");
            process::exit(0);
        } else if "next" == line || "n" == line {
            let mv = play_turn(is_black_turn, &mut board, &mut black, &mut white);

            if board.has_win() == Tile::Empty {
                if mv != String::from("swap") {
                    is_black_turn = !is_black_turn;
                } else {
                    let tmp = black;
                    black = white;
                    white = tmp;
                }
            } else if board.has_win() == Tile::Black {
                println!("Black has won");
                break;
            } else {
                println!("White has won");
                break;
            }
        } else if line.len() >= 5 && "run " == &line[..4] && line[4..].parse::<usize>().is_ok() {
            for _ in 0..line[4..].parse::<usize>().unwrap() {
                let mv = play_turn(is_black_turn, &mut board, &mut black, &mut white);

                if board.has_win() == Tile::Empty {
                    if mv != String::from("swap") {
                        is_black_turn = !is_black_turn;
                    } else {
                        let tmp = black;
                        black = white;
                        white = tmp;
                    }
                } else if board.has_win() == Tile::Black {
                    println!("Black has won");
                    break;
                } else {
                    println!("White has won");
                    break;
                }
            }
        } else {
            println!("Command `{}` not found. See \"help\" for a list of commands", line);
        }
    }
}



fn spawn_bot(bot_path: PathBuf, color: &str) -> Child {
        Command::new(bot_path)
        .arg(color)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to startup bot")
}

fn send_message(bot: &mut Child, message: &str) {
    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "{}\n", message).unwrap();
}

fn get_response(bot: &mut Child, message: &str) -> String {
    send_message(bot, message);

    let bot_out = bot.stdout.as_mut().unwrap();
    let mut reader = BufReader::new(bot_out);
    let mut response = String::new();

    reader.read_line(&mut response).unwrap();
    response
}

fn init_board(size: u8, black: &mut Child, white: &mut Child) {
    send_message(black, &format!("init_board {}\n", size));
    send_message(white, &format!("init_board {}\n", size));
}

fn print_bot_board(bot: &mut Child, color: Tile) {
    let response = get_response(bot, "show_board");
    println!("{} board ------------------\n{}", color, Board::from(&response));
}

fn play_turn(is_black_turn: bool, board: &mut Board, black: &mut Child, white: &mut Child) -> String {
    let (this_turn_bot, next_turn_bot, this_turn_color) = if is_black_turn {
        (black, white, Tile::Black)
    } else {
        (white, black, Tile::White)
    };

    let response = get_response(this_turn_bot, "make_move");
    let mv = response.trim();

    if board.is_valid_move(&mv) {
        println!("{}'s move: {}", this_turn_color, mv);
        board.set_move(&mv, this_turn_color);
        if mv.eq("swap") {
            send_message(next_turn_bot, "swap");
        } else {
            send_message(next_turn_bot, &format!("seto {}\n", mv));
        }
    } else {
        panic!("Black bot returned invalid move `{}`!", mv);
    }

    String::from(mv)
}
