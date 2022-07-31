use std::io::{self, BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio, ChildStdout};

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
    bot.wait()?;
    Ok(())
}

fn get_command_output(stdout: &mut ChildStdout, cmds_to_skip: i32) -> String {
    let mut reader = BufReader::new(stdout);
    let mut output = String::new();

    for _ in 0..=cmds_to_skip {
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
    let mut stdout = bot.stdout.take().unwrap();
    let output = get_command_output(&mut stdout, 0);

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
fn test_sety_a1() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "sety a1\n")?;
    write!(bot_in, "show_board\n")?;
    let mut stdout = bot.stdout.take().unwrap();
    let output = get_command_output(&mut stdout, 1);

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
fn test_sety_a8() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "sety a8\n")?;
    write!(bot_in, "show_board\n")?;
    let mut stdout = bot.stdout.take().unwrap();
    let output = get_command_output(&mut stdout, 1);

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
fn test_sety_h1() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "sety h1\n")?;
    write!(bot_in, "show_board\n")?;
    let mut stdout = bot.stdout.take().unwrap();
    let output = get_command_output(&mut stdout, 1);

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
fn test_sety_h8() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "sety h8\n")?;
    write!(bot_in, "show_board\n")?;
    let mut stdout = bot.stdout.take().unwrap();
    let output = get_command_output(&mut stdout, 1);

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
fn test_sety_mid() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "sety d3\n")?;
    write!(bot_in, "show_board\n")?;
    let mut stdout = bot.stdout.take().unwrap();
    let output = get_command_output(&mut stdout, 1);

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

// test play on own spot
#[test]
fn test_sety_on_own_spot() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "sety a1\n")?;
    write!(bot_in, "sety a1\n")?;
    write!(bot_in, "show_board\n")?;
    let mut stdout = bot.stdout.take().unwrap();
    let output = get_command_output(&mut stdout, 2);

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

// test play on opponents spot
#[test]
fn test_sety_on_opponents_spot() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "seto a1\n")?;
    write!(bot_in, "sety a1\n")?;
    write!(bot_in, "show_board\n")?;
    let mut stdout = bot.stdout.take().unwrap();
    let output = get_command_output(&mut stdout, 2);

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

// test seto
#[test]
fn test_seto_a1() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "seto a1\n")?;
    write!(bot_in, "show_board\n")?;
    let mut stdout = bot.stdout.take().unwrap();
    let output = get_command_output(&mut stdout, 1);

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

#[test]
fn test_seto_a8() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "seto a8\n")?;
    write!(bot_in, "show_board\n")?;
    let mut stdout = bot.stdout.take().unwrap();
    let output = get_command_output(&mut stdout, 1);

    assert_eq!(
        output.as_str().trim_start(),
        ". . . . . . . B 
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
fn test_seto_h1() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "seto h1\n")?;
    write!(bot_in, "show_board\n")?;
    let mut stdout = bot.stdout.take().unwrap();
    let output = get_command_output(&mut stdout, 1);

    assert_eq!(
        output.as_str().trim_start(),
        ". . . . . . . . 
 . . . . . . . . 
  . . . . . . . . 
   . . . . . . . . 
    . . . . . . . . 
     . . . . . . . . 
      . . . . . . . . 
       B . . . . . . . 
= 
"
    );

    shutdown_bot(&mut bot)?;
    Ok(())
}

#[test]
fn test_seto_h8() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "seto h8\n")?;
    write!(bot_in, "show_board\n")?;
    let mut stdout = bot.stdout.take().unwrap();
    let output = get_command_output(&mut stdout, 1);

    assert_eq!(
        output.as_str().trim_start(),
        ". . . . . . . . 
 . . . . . . . . 
  . . . . . . . . 
   . . . . . . . . 
    . . . . . . . . 
     . . . . . . . . 
      . . . . . . . . 
       . . . . . . . B 
= 
"
    );

    shutdown_bot(&mut bot)?;
    Ok(())
}

#[test]
fn test_seto_mid() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "seto d3\n")?;
    write!(bot_in, "show_board\n")?;
    let mut stdout = bot.stdout.take().unwrap();
    let output = get_command_output(&mut stdout, 1);

    assert_eq!(
        output.as_str().trim_start(),
        ". . . . . . . . 
 . . . . . . . . 
  . . . . . . . . 
   . . B . . . . . 
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

// test setoing on own spot
#[test]
fn test_seto_on_own_spot() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "seto a1\n")?;
    write!(bot_in, "seto a1\n")?;
    write!(bot_in, "show_board\n")?;
    let mut stdout = bot.stdout.take().unwrap();
    let output = get_command_output(&mut stdout, 2);

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

// test playing on own spot
#[test]
fn test_seto_on_opponents_spot() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "sety a1\n")?;
    write!(bot_in, "seto a1\n")?;
    write!(bot_in, "show_board\n")?;
    let mut stdout = bot.stdout.take().unwrap();
    let output = get_command_output(&mut stdout, 2);

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

// test 'swap'
#[test]
fn test_own_swap() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "seto a1\n")?;
    write!(bot_in, "sety swap\n")?;
    write!(bot_in, "seto a2\n")?;
    write!(bot_in, "show_board\n")?;
    let mut stdout = bot.stdout.take().unwrap();
    let output = get_command_output(&mut stdout, 3);

    assert_eq!(
        output.as_str().trim_start(),
        "B W . . . . . . 
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
fn test_opponent_swap() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "sety a1\n")?;
    write!(bot_in, "seto swap\n")?;
    write!(bot_in, "sety a2\n")?;
    write!(bot_in, "show_board\n")?;
    let mut stdout = bot.stdout.take().unwrap();
    let output = get_command_output(&mut stdout, 3);

    assert_eq!(
        output.as_str().trim_start(),
        "W B . . . . . . 
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
fn test_own_swap_at_wrong_time() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "sety a1\n")?;
    write!(bot_in, "seto a2\n")?;
    write!(bot_in, "sety swap\n")?;
    write!(bot_in, "seto a3\n")?;
    write!(bot_in, "show_board\n")?;
    let mut stdout = bot.stdout.take().unwrap();
    let output = get_command_output(&mut stdout, 4);

    assert_eq!(
        output.as_str().trim_start(),
        "W B B . . . . . 
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
fn test_opponent_swap_at_wrong_time() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "seto a1\n")?;
    write!(bot_in, "sety a2\n")?;
    write!(bot_in, "seto swap\n")?;
    write!(bot_in, "sety a3\n")?;
    write!(bot_in, "show_board\n")?;
    let mut stdout = bot.stdout.take().unwrap();
    let output = get_command_output(&mut stdout, 4);

    assert_eq!(
        output.as_str().trim_start(),
        "B W W . . . . . 
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

// test basic win conditions
#[test]
fn test_basic_wins_with_white() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    // top-level win
    write!(bot_in, "sety a1\n")?;
    write!(bot_in, "sety a2\n")?;
    write!(bot_in, "sety a3\n")?;
    write!(bot_in, "sety a4\n")?;
    write!(bot_in, "sety a5\n")?;
    write!(bot_in, "sety a6\n")?;
    write!(bot_in, "sety a7\n")?;
    write!(bot_in, "sety a8\n")?;
    write!(bot_in, "check_win\n")?;

    // bottom-row win
    write!(bot_in, "init_board 8\n")?;
    write!(bot_in, "sety h1\n")?;
    write!(bot_in, "sety h2\n")?;
    write!(bot_in, "sety h3\n")?;
    write!(bot_in, "sety h4\n")?;
    write!(bot_in, "sety h5\n")?;
    write!(bot_in, "sety h6\n")?;
    write!(bot_in, "sety h7\n")?;
    write!(bot_in, "sety h8\n")?;
    write!(bot_in, "check_win\n")?;

    // top-left -> bottom-right diag
    write!(bot_in, "init_board 8\n")?;
    write!(bot_in, "sety a1\n")?;
    write!(bot_in, "sety b1\n")?;
    write!(bot_in, "sety b2\n")?;
    write!(bot_in, "sety c2\n")?;
    write!(bot_in, "sety c3\n")?;
    write!(bot_in, "sety d3\n")?;
    write!(bot_in, "sety d4\n")?;
    write!(bot_in, "sety e4\n")?;
    write!(bot_in, "sety e5\n")?;
    write!(bot_in, "sety f5\n")?;
    write!(bot_in, "sety f6\n")?;
    write!(bot_in, "sety g6\n")?;
    write!(bot_in, "sety g7\n")?;
    write!(bot_in, "sety h7\n")?;
    write!(bot_in, "sety h8\n")?;
    write!(bot_in, "check_win\n")?;

    // bottom-left -> top-right diag
    write!(bot_in, "init_board 8\n")?;
    write!(bot_in, "sety h1\n")?;
    write!(bot_in, "sety g2\n")?;
    write!(bot_in, "sety f3\n")?;
    write!(bot_in, "sety e4\n")?;
    write!(bot_in, "sety d5\n")?;
    write!(bot_in, "sety c6\n")?;
    write!(bot_in, "sety b7\n")?;
    write!(bot_in, "sety a8\n")?;
    write!(bot_in, "check_win\n")?;

    let mut stdout = bot.stdout.take().unwrap();
    let mut output = get_command_output(&mut stdout, 8);
    assert_eq!(
        output.as_str().trim_start(),
        "1
= 
"
    );

    output = get_command_output(&mut stdout, 9);
    assert_eq!(
        output.as_str().trim_start(),
        "1
= 
"
    );

    output = get_command_output(&mut stdout, 16);
    assert_eq!(
        output.as_str().trim_start(),
        "1
= 
"
    );

    output = get_command_output(&mut stdout, 9);
    assert_eq!(
        output.as_str().trim_start(),
        "1
= 
"
    );

    shutdown_bot(&mut bot)?;
    Ok(())
}

// test different non-terminal conditions
// test different win conditions
// test different loss conditions

/**
 * LET THE TESTING OF 'UNSET' BEGIN
 * NOTE: These tests assume that 'seto' and 'sety' work as intended
 */
// test clearing
#[test]
fn test_unset_clears_correctly() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "sety a1\n")?;
    write!(bot_in, "unset a1\n")?;
    write!(bot_in, "show_board\n")?;
    let mut stdout = bot.stdout.take().unwrap();
    let output = get_command_output(&mut stdout, 2);

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

// test that other cells are unaffected
#[test]
fn test_unset_leaves_other_cells() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    let letters = ["a", "b", "c", "d", "e", "f", "g", "h"];
    for letter in letters {
        for digit in 1..=8 {
            write!(bot_in, "{} {}{}\n", "sety", letter, digit.to_string().as_str())?;
        }
    }
    write!(bot_in, "unset a1\n")?;
    write!(bot_in, "show_board\n")?;
    let mut stdout = bot.stdout.take().unwrap();
    let output = get_command_output(&mut stdout, (letters.len() as i32) * 8 + 1);

    assert_eq!(
        output.as_str().trim_start(),
        ". W W W W W W W 
 W W W W W W W W 
  W W W W W W W W 
   W W W W W W W W 
    W W W W W W W W 
     W W W W W W W W 
      W W W W W W W W 
       W W W W W W W W 
= 
"
    );

    shutdown_bot(&mut bot)?;
    Ok(())
}

// test clearing empty cell
#[test]
fn test_unset_on_empty_cell() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "unset a1\n")?;
    write!(bot_in, "show_board\n")?;
    let mut stdout = bot.stdout.take().unwrap();
    let output = get_command_output(&mut stdout, 1);

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

/*
 *LET THE TESTING OF INIT_BOARD COMMENCE
 */
#[test]
fn test_init_board_3() -> io::Result<()> {
    let mut bot = get_bot()?;

    let bot_in = bot.stdin.as_mut().unwrap();
    write!(bot_in, "init_board 3\n")?;
    write!(bot_in, "show_board\n")?;
    let mut stdout = bot.stdout.take().unwrap();
    let output = get_command_output(&mut stdout, 1);

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
    let mut stdout = bot.stdout.take().unwrap();
    let output = get_command_output(&mut stdout, 1);

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
    let mut stdout = bot.stdout.take().unwrap();
    let output = get_command_output(&mut stdout, 9);

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
