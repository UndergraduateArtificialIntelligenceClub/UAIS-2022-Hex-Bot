
use std::process::{Command, Stdio, Child};
use std::io::{self, BufRead, Write, BufReader};

fn get_bot(_board_size: i32) -> io::Result<Child> {
    let bot_1 = Command::new("python")
        .arg("D:\\Over9000\\Documents\\Dev\\UAIS-2022-Hex-Bot\\random_bot\\main.py")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    println!("here 1");
    Ok(bot_1)
}

fn shutdown_bot(bot: &mut Child) -> io::Result<()> {
    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "quit\n")?;

    Ok(())
}

fn get_command_output(bot: &mut Child, cmds_to_skip: i32) -> String {
    let mut reader = BufReader::new(bot.stdout.take().unwrap());
    let mut output = String::new();

    for _ in -1..cmds_to_skip {
        output.clear();
        let bytes_read = reader.read_line(&mut output).unwrap();

        while bytes_read > 0 && !output.contains("=") {
            reader.read_line(&mut output)
                .expect("Could not read line from stdout");
        }
    }

    output.replace("\r\n", "\n")
}

#[test]
fn test_empty_show_board() -> io::Result<()> {
    let mut bot = get_bot(8)?;
    
    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "show_board\n")?;
    let output = get_command_output(&mut bot, 0);

    assert_eq!(output, "
. . . . . . . . 
 . . . . . . . . 
  . . . . . . . . 
   . . . . . . . . 
    . . . . . . . . 
     . . . . . . . . 
      . . . . . . . . 
       . . . . . . . . 
= 
");

    shutdown_bot(&mut bot)?;
    Ok(())
}