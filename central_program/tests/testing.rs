use std::io::{self, BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};

fn get_bot() -> io::Result<Child> {
    let bot_1 = Command::new("python")
        .arg("D:\\Over9000\\Documents\\Dev\\UAIS-2022-Hex-Bot\\random_bot\\main.py")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

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
            reader
                .read_line(&mut output)
                .expect("Could not read line from stdout");
        }
    }

    output.replace("\r\n", "\n")
}

#[test]
fn test_empty_show_board() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "show_board\n")?;
    let output = get_command_output(&mut bot, 0);

    assert_eq!(
        output.as_str().trim_start(),
        ". . . . . . . . 
 . . . . . . . . 
  . . . . . . . . 
   . . . . . . . . 
    . . . . . . . . 
     . . . . . . . . 
      . . . . . . . . 
       . . . . . . . . 
= 
"
    );

    shutdown_bot(&mut bot)?;
    Ok(())
}

// test sety
// test different locations
#[test]
fn test_play_a1() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "sety a1\n")?;
    write!(bot_in, "show_board\n")?;
    let output = get_command_output(&mut bot, 1);

    assert_eq!(
        output.as_str().trim_start(),
        "W . . . . . . . 
 . . . . . . . . 
  . . . . . . . . 
   . . . . . . . . 
    . . . . . . . . 
     . . . . . . . . 
      . . . . . . . . 
       . . . . . . . . 
= 
"
    );

    shutdown_bot(&mut bot)?;
    Ok(())
}

#[test]
fn test_play_a8() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "sety a8\n")?;
    write!(bot_in, "show_board\n")?;
    let output = get_command_output(&mut bot, 1);

    assert_eq!(
        output.as_str().trim_start(),
        ". . . . . . . W 
 . . . . . . . . 
  . . . . . . . . 
   . . . . . . . . 
    . . . . . . . . 
     . . . . . . . . 
      . . . . . . . . 
       . . . . . . . . 
= 
"
    );

    shutdown_bot(&mut bot)?;
    Ok(())
}

#[test]
fn test_play_h1() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "sety h1\n")?;
    write!(bot_in, "show_board\n")?;
    let output = get_command_output(&mut bot, 1);

    assert_eq!(
        output.as_str().trim_start(),
        ". . . . . . . . 
 . . . . . . . . 
  . . . . . . . . 
   . . . . . . . . 
    . . . . . . . . 
     . . . . . . . . 
      . . . . . . . . 
       W . . . . . . . 
= 
"
    );

    shutdown_bot(&mut bot)?;
    Ok(())
}

#[test]
fn test_play_h8() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "sety h8\n")?;
    write!(bot_in, "show_board\n")?;
    let output = get_command_output(&mut bot, 1);

    assert_eq!(
        output.as_str().trim_start(),
        ". . . . . . . . 
 . . . . . . . . 
  . . . . . . . . 
   . . . . . . . . 
    . . . . . . . . 
     . . . . . . . . 
      . . . . . . . . 
       . . . . . . . W 
= 
"
    );

    shutdown_bot(&mut bot)?;
    Ok(())
}

#[test]
fn test_play_mid() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "sety d3\n")?;
    write!(bot_in, "show_board\n")?;
    let output = get_command_output(&mut bot, 1);

    assert_eq!(
        output.as_str().trim_start(),
        ". . . . . . . . 
 . . . . . . . . 
  . . . . . . . . 
   . . W . . . . . 
    . . . . . . . . 
     . . . . . . . . 
      . . . . . . . . 
       . . . . . . . . 
= 
"
    );

    shutdown_bot(&mut bot)?;
    Ok(())
}

// test playing on own spot
#[test]
fn test_play_on_own_spot() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "sety a1\n")?;
    write!(bot_in, "sety a1\n")?;
    write!(bot_in, "show_board\n")?;
    let output = get_command_output(&mut bot, 2);

    assert_eq!(
        output.as_str().trim_start(),
        "W . . . . . . . 
 . . . . . . . . 
  . . . . . . . . 
   . . . . . . . . 
    . . . . . . . . 
     . . . . . . . . 
      . . . . . . . . 
       . . . . . . . . 
= 
"
    );

    shutdown_bot(&mut bot)?;
    Ok(())
}

// test playing on opponents spot
#[test]
fn test_play_on_opponents_spot() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "seto a1\n")?;
    write!(bot_in, "sety a1\n")?;
    write!(bot_in, "show_board\n")?;
    let output = get_command_output(&mut bot, 2);

    assert_eq!(
        output.as_str().trim_start(),
        "B . . . . . . . 
 . . . . . . . . 
  . . . . . . . . 
   . . . . . . . . 
    . . . . . . . . 
     . . . . . . . . 
      . . . . . . . . 
       . . . . . . . . 
= 
"
    );

    shutdown_bot(&mut bot)?;
    Ok(())
}
// test 'swap'

// test seto
// test different locations
// test playing on own spot
// test playing on opponents spot
// test 'swap'

// test basic win conditions
// test different non-terminal conditions
// test different win conditions
// test different loss conditions

// test unset
// test clearing
// test that other cells are unaffected
// test clearing empty cell

/*
    LET THE TESTING OF INIT_BOARD COMMENCE
*/
#[test]
fn test_init_board_3() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "init_board 3\n")?;
    write!(bot_in, "show_board\n")?;
    let output = get_command_output(&mut bot, 1);

    assert_eq!(
        output.as_str().trim_start(),
        ". . . 
 . . . 
  . . . 
= 
"
    );

    shutdown_bot(&mut bot)?;
    Ok(())
}

#[test]
fn test_init_board_11() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "init_board 11\n")?;
    write!(bot_in, "show_board\n")?;
    let output = get_command_output(&mut bot, 1);

    assert_eq!(
        output.as_str().trim_start(),
        ". . . . . . . . . . . 
 . . . . . . . . . . . 
  . . . . . . . . . . . 
   . . . . . . . . . . . 
    . . . . . . . . . . . 
     . . . . . . . . . . . 
      . . . . . . . . . . . 
       . . . . . . . . . . . 
        . . . . . . . . . . . 
         . . . . . . . . . . . 
          . . . . . . . . . . . 
= 
"
    );

    shutdown_bot(&mut bot)?;
    Ok(())
}

// test board is cleared
// NOTE: This test assumes that 'sety' works properly.
#[test]
fn test_init_board_clears_correctly() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "sety a1\n")?;
    write!(bot_in, "seto b1\n")?;
    write!(bot_in, "sety a2\n")?;
    write!(bot_in, "seto b2\n")?;
    write!(bot_in, "sety a3\n")?;
    write!(bot_in, "seto b3\n")?;
    write!(bot_in, "sety a4\n")?;
    write!(bot_in, "seto b4\n")?;
    write!(bot_in, "init_board 8\n")?;
    write!(bot_in, "show_board\n")?;
    let output = get_command_output(&mut bot, 9);

    assert_eq!(
        output.as_str().trim_start(),
        ". . . . . . . . 
 . . . . . . . . 
  . . . . . . . . 
   . . . . . . . . 
    . . . . . . . . 
     . . . . . . . . 
      . . . . . . . . 
       . . . . . . . . 
= 
"
    );

    shutdown_bot(&mut bot)?;
    Ok(())
}
