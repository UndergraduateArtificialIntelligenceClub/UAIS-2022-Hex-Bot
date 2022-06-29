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
    rows: isize,
    cols: isize,
    board: Vec<Tile>,
}

impl Board {
    pub fn new(height: isize, width: isize) -> Self {
        Self {
            // Row major storage
            rows: height,
            cols: width,
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
        if r < 0 || c < 0 || r >= self.rows || c >= self.cols {
            None
        } else {
            Some((r * self.cols + c) as usize)
        }
    }

    fn index_to_coord(&self, i: usize) -> Option<(isize, isize)> {
        let i = i as isize;

        if 0 <= i && i < self.board.len() as isize {
            Some((i / self.cols as isize, i % self.cols as isize))
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
        let mut dfs_tree = vec![DFS::Unvisited; (self.rows * self.cols) as usize];

        for start_col in 0..self.cols {
            let start = self.coord_to_index(0, start_col).unwrap();

            if self.board[start] == Tile::Red && dfs_tree[start] == DFS::Unvisited {
                for adj in self.get_adj(0, start_col).into_iter() {
                    if self.board[adj] == Tile::Red {
                        dfs_tree[start] = DFS::Visited;

                        if self.has_path(adj, Tile::Red, &mut dfs_tree) {
                            //eprintln!("{} -> {}", start, adj);
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    fn is_blue_win(&self) -> bool {
        let mut dfs_tree = vec![DFS::Unvisited; (self.rows * self.cols) as usize];

        for start_row in 0..self.rows {
            let start = self.coord_to_index(start_row, 0).unwrap();

            if self.board[start] == Tile::Blue && dfs_tree[start] == DFS::Unvisited {
                for adj in self.get_adj(start_row, 0).into_iter() {
                    if self.board[adj] == Tile::Blue {
                        dfs_tree[start] = DFS::Visited;

                        if self.has_path(adj, Tile::Blue, &mut dfs_tree) {
                            //eprintln!("{} -> {}", start, adj);
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    fn has_path(&self, from: usize, color: Tile, dfs_tree: &mut Vec<DFS>) -> bool {
        let (r, c) = self.index_to_coord(from).unwrap();

        if color == Tile::Blue && c == self.cols - 1 || color == Tile::Red && r == self.rows - 1 {
            return true;
        }

        dfs_tree[from] = DFS::Visiting;
        for adj in self.get_adj(r, c).into_iter() {
            if dfs_tree[adj] == DFS::Unvisited && self.board[adj] == color {
                if self.has_path(adj, color, dfs_tree) {
                    //eprintln!("{} -> {}", from, adj);
                    return true;
                }
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
    // ---------------
    // Red: X, Blue: O
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.rows {
            write!(f, "{}", " ".repeat(r as usize))?;

            for c in 0..self.cols {
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

#[cfg(test)]
mod board_testing {
    use super::*;

    #[test]
    fn setters_getters() {
        let mut board = Board::new(4,4);

        for r in 0..4 {
            for c in 0..4 {
                assert_eq!(board.get(r,c), Some(Tile::Empty));
            }
        }

        board.set(0,3, Tile::Blue);
        assert_eq!(board.get(0,3), Some(Tile::Blue));

        board.set(3,2, Tile::Red);
        assert_eq!(board.get(3,2), Some(Tile::Red));

        board.set(3,2, Tile::Empty);
        assert_eq!(board.get(3,2), Some(Tile::Empty));
    }

    #[test]
    fn adjacent_tiles() {
        let mut board = Board::new(4,5);
        {
            let r = 2; let c = 2;

            let adjs = board.get_adj(r, c);
            let expected = vec![7, 8, 11, 13, 16, 17];

            assert_eq!(adjs.iter().sum::<usize>(), expected.iter().sum::<usize>());
        }
        {
            let r = 1; let c = 4;

            let mut adjs = board.get_adj(r, c);
            let mut expected = vec![4, 8, 13, 14];

            assert_eq!(adjs.iter().sum::<usize>(), expected.iter().sum::<usize>());
        }
        {
            let r = 3; let c = 4;

            let mut adjs = board.get_adj(r, c);
            let mut expected = vec![14, 18];

            assert_eq!(adjs.iter().sum::<usize>(), expected.iter().sum::<usize>());
        }
    }

    #[test]
    fn drawing() {
        let mut board = Board::new(4,4);

        board.set(0,0, Tile::Red);
        board.set(1,1, Tile::Red);
        board.set(2,2, Tile::Red);
        board.set(3,3, Tile::Red);

        board.set(3,0, Tile::Blue);
        board.set(3,2, Tile::Blue);
        board.set(1,2, Tile::Blue);

        let expected = "X . . . \n . X O . \n  . . X . \n   O . O X \n\
            ---------------\n\
            Red: X, Blue: O";

        assert_eq!(format!("{}", board), expected);

        board.set(0,1, Tile::Red);
        board.set(0,2, Tile::Red);
        board.set(0,3, Tile::Red);

        let expected2 = "X X X X \n . X O . \n  . . X . \n   O . O X \n\
            ---------------\n\
            Red: X, Blue: O";

        assert_eq!(format!("{}", board), expected2);
    }

    #[test]
    fn check_win() {
        {
            let mut board = Board::new(4,4);
            board.set(0,0, Tile::Blue);
            board.set(0,1, Tile::Blue);
            board.set(0,2, Tile::Blue);
            board.set(0,3, Tile::Blue);

            assert_eq!(board.has_win(), Tile::Blue);
        }
        {
            let mut board = Board::new(4,4);
            board.set(0,0, Tile::Red);
            board.set(1,0, Tile::Red);
            board.set(2,0, Tile::Red);
            board.set(3,0, Tile::Red);

            assert_eq!(board.has_win(), Tile::Red);
        }
        {
            let mut board = Board::new(4,4);
            board.set(0,0, Tile::Red);
            board.set(0,1, Tile::Red);
            board.set(0,2, Tile::Red);
            board.set(0,3, Tile::Red);

            board.set(1,0, Tile::Blue);
            board.set(2,0, Tile::Blue);
            board.set(3,0, Tile::Blue);

            assert_eq!(board.has_win(), Tile::Empty);
        }
        {
            let mut board = Board::new(4,4);
            board.set(2,0, Tile::Blue);
            board.set(2,1, Tile::Blue);
            board.set(1,2, Tile::Blue);
            board.set(0,3, Tile::Blue);

            board.set(1,1, Tile::Red);
            board.set(2,2, Tile::Red);
            board.set(1,3, Tile::Red);
            board.set(3,2, Tile::Red);

            assert_eq!(board.has_win(), Tile::Blue);
        }
        {
            let mut board = Board::new(4,5);
            board.set(3,0, Tile::Blue);
            board.set(2,1, Tile::Blue);
            board.set(1,2, Tile::Blue);
            board.set(0,3, Tile::Blue);
            board.set(1,3, Tile::Blue);
            board.set(2,3, Tile::Blue);
            board.set(3,3, Tile::Blue);
            board.set(3,4, Tile::Blue);

            board.set(1,1, Tile::Red);
            board.set(3,2, Tile::Red);
            board.set(0,0, Tile::Red);
            board.set(0,1, Tile::Red);
            board.set(1,1, Tile::Red);
            board.set(2,0, Tile::Red);
            board.set(3,1, Tile::Red);

            assert_eq!(board.has_win(), Tile::Blue);

            board.set(2,1, Tile::Red);
            assert_eq!(board.has_win(), Tile::Red);

            board.set(2,1, Tile::Empty);
            assert_eq!(board.has_win(), Tile::Empty);

        }
    }
}
