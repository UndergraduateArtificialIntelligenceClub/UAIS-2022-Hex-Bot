mod board;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEvent, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{
    backend::{Backend, CrosstermBackend},
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

const BG_BLACK: Style = Style {
    fg: None,
    bg: Some(Color::Blue),
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};

const BG_WHITE: Style = Style {
    fg: None,
    bg: Some(Color::Green),
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};

fn main() -> Try<()> {
    let mut stdout = io::stdout();
    execute!(stdout, EnableMouseCapture, EnterAlternateScreen)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(Tile::White);
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
    size: usize,
    messages: Vec<String>,
    color: Tile,
}

impl App {
    pub fn new(color: Tile) -> Self {
        let default_size = 6;

        Self {
            board: Board::new(default_size),
            size: default_size as usize,
            messages: vec!["Starting game...".to_string(), "Starting game...".to_string()],
            color,
        }
    }

    fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Try<()> {
        let re_init = Regex::new(r"init_board (\d+)").unwrap();
        let re_make_move = Regex::new(r"make_move").unwrap();
        let re_seto = Regex::new(r"seto ([a-z]\d)").unwrap();
        let re_sety = Regex::new(r"sety ([a-z]\d)").unwrap();
        let re_unset = Regex::new(r"unset ([a-z]\d)").unwrap();
        let re_quit = Regex::new(r"quit").unwrap();

        let stdin = io::stdin();
        let mut lines = stdin.lock().lines().map(|s| s.expect("Failed to read line from stdin"));

        terminal.draw(|f| self.tui(f))?;
        self.get_next_click(terminal).expect("Failed to get crossterm event");

        for ref line in lines {
            if re_init.is_match(line) {
                let caps = re_init.captures(line).unwrap();
                self.size = caps.get(1).unwrap().as_str().parse()?;
                self.board = Board::new(self.size as u8);
                self.messages.push(format!("Created new board of size {s}x{s}", s = self.size));
            } else if re_make_move.is_match(line) {
                let mut stdout = io::stdout().lock();
                //let reply = self.get_next_click().expect("Failed to get crossterm event");
                //writeln!(&mut stdout, "{reply}")?;
            } else if re_seto.is_match(line) {
                todo!();
            } else if re_sety.is_match(line) {
                todo!();
            } else if re_unset.is_match(line) {
                todo!();
            } else if re_quit.is_match(line) {
                return Ok(());
            } else {
                panic!("Unrecognized command from central: `{line}`");
            }

            terminal.draw(|f| self.tui(f))?;
        }

        Ok(())
    }

    fn get_next_click<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Try<String> {
        loop {
            if let Event::Mouse(mouse) = crossterm::event::read()? {
                self.messages.push(format!("({}, {})", mouse.row, mouse.column));
                terminal.draw(|f| self.tui(f))?;

                //let row = ((mouse.row as usize - 3 - 1) / 2) + 1;
                //let col = ((mouse.column as usize - row + 1 - 3 - 1) / 2) + 1;


                //if row <= self.size && col <= self.size {
                //    let c = char::from_u32((row + 97) as u32).unwrap();
                //    return Ok(format!("{}{}", c, col));
                //}
            }
        }
    }

    fn tui<B: Backend>(&mut self, f: &mut Frame<B>) {
        let mut outer = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .horizontal_margin(2)
            .constraints([
                Constraint::Length((self.size * 5 + 2) as u16),
                Constraint::Min(1),
            ].as_ref())
            .split(f.size());

        outer[1].x += 3 * self.size as u16;
        outer[1].width -= 3 * self.size as u16;

        let left_side = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(1)].as_ref())
            .split(outer[0]);

        let horizontal_board = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(2), Constraint::Min(1)].as_ref())
            .split(left_side[1]);

        let board_rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(3); self.size + 1].as_ref())
            .split(horizontal_board[1]);

        let tile_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        for i in 0..self.size {
            let cols = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Length(5); self.size].as_ref())
                .split(board_rows[i]);

            for mut col in cols {
                let bg = Span::styled("   ", BG_WHITE);

                col.x += 3 * i as u16;
                //col.y -= i as u16;

                f.render_widget(Paragraph::new(bg).block(tile_block.clone()), col);
            }
        }

        let mut col_labels: Vec<_> = (1..=(self.size + 1)).map(|n| format!("    {n}")).collect();
        eprintln!("Col length: {}", col_labels.len());
        eprintln!("Should be: {}", self.size);

        let para = Paragraph::new(col_labels.join(""));
        f.render_widget(para, left_side[0]);

        let mut row_labels = (1..=self.size)
            .map(|n| char::from_u32(n as u32 + 96).unwrap())
            .map(|c| ListItem::new(c.to_string()))
            .collect::<Vec<ListItem>>();

        for i in 0..row_labels.len() {
            row_labels.insert(i * 3 + 1, ListItem::new(" ".to_string()));
            row_labels.insert(i * 3 + 1, ListItem::new(" ".to_string()));
        }

        row_labels.insert(0, ListItem::new(" ".to_string()));

        f.render_widget(List::new(row_labels), horizontal_board[0]);

        let list = List::new(self.messages
                .iter()
                .rev()
                .map(|s| ListItem::new(s.clone()))
                .collect::<Vec<ListItem>>())
            .start_corner(Corner::BottomLeft);

        f.render_widget(
            list.block(Block::default().borders(Borders::ALL).title("Play Log")),
            outer[1]);

        //f.render_widget(Block::default().borders(Borders::ALL), outer[0]);
    }
}
