// This program is written with unix in mind. No clue what'll happen on windows...
mod board;
mod testing;

use testing::BotTest;

use std::process::{self, Command, Stdio};
use std::io::{self, BufRead, Read, Write};
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
        /// Color of bot on this test
        #[clap(action)]
        color: Color,
        /// Path to the main executable for the bot
        #[clap(action)]
        bot_path: PathBuf,
        /// String of arguments to pass to the bot
        #[clap(action)]
        bot_args: Vec<String>,
    },
    /// Let two bots face off in hex
    Matchup {
        /// The size of the board squared. Ex: 11
        #[clap(action)]
        size: u8,
        /// Path to the main executable for the white bot
        #[clap(action)]
        white_bot: PathBuf,
        /// Path to the main executable for the black bot
        #[clap(action)]
        black_bot: PathBuf,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, clap::ValueEnum)]
pub enum Color {
    Black,
    White,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Test { bot_path, bot_args, color } => {
            let mut bot_test = BotTest::new(color, bot_path, bot_args);
            bot_test.test();
        }
        Commands::Matchup { size, white_bot, black_bot } => {
            println!("Board size: {}\nWhite: {:?}\nBlack:{:?}", size, white_bot, black_bot);
            todo!();
        }
    }
}

fn run_match() {
    todo!();
    // Repl
    let stdin = io::stdin();

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        if "help" == line || "h" == line {
            ()
        } else if "exit" == line || "e" == line {
            println!("Shutdown NOW!");
            process::exit(0);
        } else if "run" == line || "r" == line {
            let mut board = Board::new(4, 4);

            // TODO: Remove. This is to demo the display capabilities
            board.set(0,0, Tile::Red);
            board.set(1,1, Tile::Red);
            board.set(2,2, Tile::Red);
            board.set(3,3, Tile::Red);

            board.set(3,0, Tile::Blue);
            board.set(3,2, Tile::Blue);
            board.set(1,2, Tile::Blue);

            println!("{}", board);

            //play_round(&bot_1, &bot_2)
            //    .expect("Something went wrong");
        } else {
            println!("Unsupported command: `{}`", line);
        }
    }
}

fn play_round(bot_1_name: &str, bot_2_name: &str) -> io::Result<()>
{
    println!("Pretending to run {} vs {}", bot_1_name, bot_2_name);

    let mut bot_1 = Command::new(bot_1_name)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut bot_2 = Command::new(bot_2_name)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    // TODO: This is just a sample. Keep running until there's a win state
    let mut output = String::new();

    for round in 0..3 {
        let bot_1_in = bot_1.stdin.as_mut().unwrap();
        //bot_1_in.write_all()?;  // TODO: Use this one, we don't need formatting
        write!(bot_1_in, "Round: {}", round)?;

        output.clear();
        bot_1.stdout.as_mut().unwrap().read_to_string(&mut output)?;

        // TODO: Actually process the output
        println!("Bot 1 says: {}", output);

        let bot_2_in = bot_2.stdin.as_mut().unwrap();
        write!(bot_2_in, "Round: {}", round)?;

        output.clear();
        bot_2.stdout.as_mut().unwrap().read_to_string(&mut output)?;

        println!("Bot 2 says: {}", output);
    }

    Ok(())
}
