// This program is written with unix in mind. No clue what'll happen on windows...
mod board;


use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEvent, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{
    backend::{Backend, TermionBackend},
    layout::{Alignment, Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, BorderType, List, ListItem, Paragraph},
    Frame, Terminal,
};
use board::{Board, Tile};
use regex::Regex;
use std::process::{self, Command, Stdio, Child};
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

type Try<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> Try<()> {
    let mut stdout = io::stdout();
    execute!(stdout, EnableMouseCapture, EnterAlternateScreen)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    app.run(&mut terminal)?;

    // Cooking ====
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}

#[derive(Debug)]
struct App {
    board: Board,
    status_msg: String,
    color: Tile,
}

impl App {
    pub fn new(color: Tile) -> Self {
        Self {
            board: Board::new(6),
            status_msg: "Starting game...".to_string(),
            color,
        }
    }

    async fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Try<()> {
        let re_init = Regex::new(r"init_board (\d+)").unwrap();
        let re_make_move = Regex::new(r"make_move").unwrap();
        let re_seto = Regex::new(r"seto ([a-z]\d)").unwrap();
        let re_sety = Regex::new(r"sety ([a-z]\d)").unwrap();
        let re_unset = Regex::new(r"unset ([a-z]\d)").unwrap();
        let re_quit = Regex::new(r"quit").unwrap();
        let stdin = io::stdin();

        for line in stdin.lock().lines() {
            if re_init.is_match(line) {
                let caps = re_init.captures(line).unwrap();
                let size: u8 = caps.get(1).unwrap().parse()?;
                self.board = Board::new(size);
                self.status_msg = format!("Created new board of size {size}x{size}");
            } else if re_make_move.is_match(line) {
            } else if re_seto.is_match(line) {
            } else if re_sety.is_match(line) {
            } else if re_unset.is_match(line) {
            } else if re_quit.is_match(line) {
            } else {
                panic!("Unrecognized command from central: `{line}`")
            }
        }

        Ok(())
    }
}

