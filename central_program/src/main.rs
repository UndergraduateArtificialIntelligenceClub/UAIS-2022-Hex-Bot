// This program is written with unix in mind. No clue what'll happen on windows...
mod board;
mod testing;

use testing::BotTest;

use std::process::{self, Command, Stdio, Child};
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
        Commands::Test { bot_path, color } => {
            let mut bot_test = BotTest::new(color, bot_path);
            bot_test.test();
        }
        Commands::Matchup { size, white_bot, black_bot } => {
            let white = spawn_bot(white_bot, "white");
            let black = spawn_bot(black_bot, "black");
            run_match(size, white, black);
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

fn run_match(size: u8, mut white: Child, mut black: Child) {
    let mut board = Board::new(size);

    let stdin = io::stdin();

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        if "help" == line || "h" == line {
            print_repl_help();
        } else if "exit" == line {
            println!("Shutting down");
            process::exit(0);
        } else if "run" == line || "r" == line {
            todo!();
        } else {
            println!("Command `{}` not found. See \"help\" for a list of commands", line);
        }
    }
}

fn print_repl_help() {
    println!("{}", [
        "==== Sentience Validator: Interactive REPL ====",
        "Command     Description",
        "h | help    Prints this help menu",
        "n | next    Prompts the bot for its next move",
        "exit        Shuts down both bots and exits",
        "==========================================",
    ].join("\n"));
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
