// This program is written with unix in mind. No clue what'll happen on windows...
use std::process::{self, Command};
use std::io::{self, BufRead, Write};

fn main()
{
    println!("UAIS Hex Bot Controller v{}\n\
              Type \"help\" for options", env!("CARGO_PKG_VERSION"));

    let stdin = io::stdin();

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        if "help" == line || "h" == line {
            let _ = print_help();
        } else if "exit" == line || "e" == line {
            println!("Shutdown NOW!");
            process::exit(0);
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
