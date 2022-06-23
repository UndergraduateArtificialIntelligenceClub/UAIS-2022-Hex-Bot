// Red is at the top/bottom. Blue is at the left/right
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Red,
    Blue,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum DFS {
    Visited,
    Visiting,
    Unvisited,
}

#[derive(Debug)]
pub struct Board {
    size: (isize, isize),
    board: Vec<Tile>,
}

impl Board {
    pub fn new(height: isize, width: isize) -> Self {
        Self {
            // Row major storage
            size: (height, width),
            board: vec![Tile::Empty; (height * width) as usize],
        }
    }

    pub fn get(&self, row: isize, col: isize) -> Option<Tile> {
        let index = self.coord_to_index(row, col)?;
        Some(self.board[index])
    }

    // Sets a tile to the given color
    pub fn set(&mut self, row: isize, col: isize, color: Tile) -> Option<()> {
        let index = self.coord_to_index(row, col)?;
        self.board[index] = color;
        Some(())
    }

    // Returns an array of all indicies adjacent to a given hex. That's 2-5 indicies
    fn get_adj(&self, r: isize, c: isize) -> Vec<usize> {
        let a = [(r, c-1), (r+1, c-1), (r-1, c), (r+1, c), (r-1, c+1), (r, c+1)];

        a.into_iter()
            .filter_map(|(r, c)| self.coord_to_index(r, c))
            .collect()
    }

    fn coord_to_index(&self, r: isize, c: isize) -> Option<usize> {
        if r < 0 || c < 0 || r >= self.size.0 || c >= self.size.1 {
            None
        } else {
            Some((r * self.size.1 + c) as usize)
        }
    }

    fn index_to_coord(&self, i: usize) -> Option<(isize, isize)> {
        let i = i as isize;

        if 0 <= i && i < self.board.len() as isize {
            Some((i / self.size.1 as isize, i % self.size.1 as isize))
        } else {
            None
        }
    }

    // Returns the color of the player who won, empty otherwise
    pub fn has_win(&self) -> Tile {
        if self.is_red_win() {
            Tile::Red
        } else if self.is_blue_win() {
            Tile::Blue
        } else {
            Tile::Empty
        }
    }

    fn is_red_win(&self) -> bool {
        for start_col in 0..self.size.1 {
            for adj in self.get_adj(start_col, 0) {
                if self.board[adj] == Tile::Red {
                    let mut dfs_tree = vec![DFS::Unvisited; (self.size.0 * self.size.1) as usize];

                    if self.has_path(adj, Tile::Red, &mut dfs_tree) {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn is_blue_win(&self) -> bool {
        for start_row in 0..self.size.0 {
            for adj in self.get_adj(start_row, 0) {
                if let Tile::Blue = self.board[adj] {
                    let mut dfs_tree = vec![DFS::Unvisited; (self.size.0 * self.size.1) as usize];

                    if self.has_path(adj, Tile::Blue, &mut dfs_tree) {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn has_path(&self, from: usize, color: Tile, dfs_tree: &mut Vec<DFS>) -> bool {
        let (r, c) = self.index_to_coord(from).unwrap();

        if color == Tile::Blue && c == self.size.1 || color == Tile::Red && r == self.size.0 {
            return true;
        }

        for adj in self.get_adj(r, c) {
            if dfs_tree[adj] == DFS::Unvisited && self.board[adj] == color {
                dfs_tree[adj] = DFS::Visiting;
                if self.has_path(adj, color, dfs_tree) {
                    return true;
                }
                dfs_tree[adj] = DFS::Visited;
            }
        }
        false
    }
}

impl std::fmt::Display for Board {
    // Example output:
    // X . . .
    //  . X O .
    //   . . X .
    //    O . O X
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.size.0 {
            write!(f, "{}", " ".repeat(r as usize))?;

            for c in 0..self.size.1 {
                match self.board[self.coord_to_index(r, c).unwrap()] {
                    Tile::Red => write!(f, "X ")?,
                    Tile::Blue => write!(f, "O ")?,
                    Tile::Empty => write!(f, ". ")?,
                }
            }
            write!(f, "\n")?;
        }

        write!(f, "{}\nRed: X, Blue: O", "-".repeat(15))
    }
}
