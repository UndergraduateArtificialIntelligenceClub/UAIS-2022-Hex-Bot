// This program is written with unix in mind. No clue what'll happen on windows...
mod board;

use std::process::{self, Command, Stdio};
use std::io::{self, BufRead, Read, Write};
use board::{Board, Tile};

fn main()
{
    // Read cli args
    let mut args = std::env::args();

    args.next();
    let bot_1 = args.next().expect("Please provide two bots");
    let bot_2 = args.next().expect("Please provide two bots");

    println!("UAIS Hex Bot Controller v{}\n\
              Type \"help\" for options", env!("CARGO_PKG_VERSION"));

    // Repl
    let stdin = io::stdin();

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        if "help" == line || "h" == line {
            let _ = print_help();
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

fn print_help() -> io::Result<()>
{
    let mut wb = io::BufWriter::new(io::stdout());

    wb.write_all(b">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>\n")?;
    write!(&mut wb, "UAIS Hex Bot Controller v{}\n", env!("CARGO_PKG_VERSION"))?;

    wb.write_all(b"Supported commands:\n\t\
        help    Print this help message\n\t\
        run     TODO: Start the program. Optionally provide a save file argument\n\t\
        next    TODO: Ask the bot for its next turn\n\t\
        print   TODO: Print the board to stdout\n\t\
        undo    TODO: As the bot to undo its last move\n\t\
        save    TODO: Save game state into /tmp/uais_hex/<timestamp>.saved\n\t\
        exit    TODO: Save game state and exit the program\n")?;

    wb.write_all(b">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>\n")?;
    wb.flush()
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
