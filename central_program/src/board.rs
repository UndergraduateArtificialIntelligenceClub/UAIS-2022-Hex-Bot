// Black goes top -> bottom. White goes left -> right
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Black,
    Empty,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum DFS {
    Visited,
    Visiting,
    Unvisited,
}

#[derive(Debug)]
pub struct Board {
    size: usize,
    board: Vec<Tile>,
}

impl Board {
    pub fn new(size: u8) -> Self {
        Self {
            size: size as usize,
            board: vec![Tile::Empty; size.pow(2) as usize],
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<Tile> {
        let index = self.coord_to_index(row, col);
        Some(self.board[index])
    }

    // Sets a tile to the given color
    pub fn set(&mut self, row: usize, col: usize, color: Tile) -> Option<()> {
        let index = self.coord_to_index(row, col);
        self.board[index] = color;
        Some(())
    }

    // Returns an array of all indicies adjacent to a given hex. That's 2-5 indicies
    fn get_adj(&self, row: usize, column: usize) -> Vec<usize> {
        let r = row as isize;
        let c = column as isize;
        let s = self.size as isize;

        let a = [(r, c-1), (r+1, c-1), (r-1, c), (r+1, c), (r-1, c+1), (r, c+1)];

        a.into_iter()
            .filter(|(r, c)| 0 <= *r && *r < s && 0 <= *c && *c < s)
            .map(|(r, c)| self.coord_to_index(r as usize, c as usize))
            .collect()
    }

    fn coord_to_index(&self, r: usize, c: usize) -> usize {
        r * self.size + c
    }

    fn index_to_coord(&self, i: usize) -> Option<(usize, usize)> {
        if i < self.board.len() {
            Some((i / self.size, i % self.size))
        } else {
            None
        }
    }

    // Returns the color of the player who won, empty otherwise
    pub fn has_win(&self) -> Tile {
        if self.is_black_win() {
            Tile::Black
        } else if self.is_white_win() {
            Tile::White
        } else {
            Tile::Empty
        }
    }

    fn is_black_win(&self) -> bool {
        let mut dfs_tree = vec![DFS::Unvisited; self.size.pow(2)];

        for start_col in 0..self.size {
            let start = self.coord_to_index(0, start_col);

            if self.board[start] == Tile::Black && dfs_tree[start] == DFS::Unvisited {
                for adj in self.get_adj(0, start_col).into_iter() {
                    if self.board[adj] == Tile::Black {
                        dfs_tree[start] = DFS::Visited;

                        if self.has_path(adj, Tile::Black, &mut dfs_tree) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    fn is_white_win(&self) -> bool {
        let mut dfs_tree = vec![DFS::Unvisited; self.size.pow(2)];

        for start_row in 0..self.size {
            let start = self.coord_to_index(start_row, 0);

            if self.board[start] == Tile::White && dfs_tree[start] == DFS::Unvisited {
                for adj in self.get_adj(start_row, 0).into_iter() {
                    if self.board[adj] == Tile::White {
                        dfs_tree[start] = DFS::Visited;

                        if self.has_path(adj, Tile::White, &mut dfs_tree) {
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

        if color == Tile::White && c == self.size - 1 || color == Tile::Black && r == self.size - 1 {
            return true;
        }

        dfs_tree[from] = DFS::Visiting;
        for adj in self.get_adj(r, c).into_iter() {
            if dfs_tree[adj] == DFS::Unvisited && self.board[adj] == color {
                if self.has_path(adj, color, dfs_tree) {
                    return true;
                }
            }
        }
        false
    }
}

impl std::fmt::Display for Board {
    // Example output:
    // B . . .
    //  . B W .
    //   . . B .
    //    W . W B
    // ---------------
    // Black: B, White: W
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.size {
            write!(f, "{}", " ".repeat(r))?;

            for c in 0..self.size {
                match self.board[self.coord_to_index(r, c)] {
                    Tile::Black => write!(f, "B ")?,
                    Tile::White => write!(f, "W ")?,
                    Tile::Empty => write!(f, ". ")?,
                }
            }
            write!(f, "\n")?;
        }

        write!(f, "{}\nBlack: B, White: W", "-".repeat(15))
    }
}

#[cfg(test)]
mod board_testing {
    use super::*;

    #[test]
    fn setters_getters() {
        let mut board = Board::new(4);

        for r in 0..4 {
            for c in 0..4 {
                assert_eq!(board.get(r,c), Some(Tile::Empty));
            }
        }

        board.set(0,3, Tile::White);
        assert_eq!(board.get(0,3), Some(Tile::White));

        board.set(3,2, Tile::Black);
        assert_eq!(board.get(3,2), Some(Tile::Black));

        board.set(3,2, Tile::Empty);
        assert_eq!(board.get(3,2), Some(Tile::Empty));
    }

    #[test]
    fn adjacent_tiles() {
        let mut board = Board::new(5);
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
            let r = 4; let c = 4;

            let mut adjs = board.get_adj(r, c);
            let mut expected = vec![19, 23];

            assert_eq!(adjs.iter().sum::<usize>(), expected.iter().sum::<usize>());
        }
    }

    #[test]
    fn drawing() {
        let mut board = Board::new(4);

        board.set(0,0, Tile::Black);
        board.set(1,1, Tile::Black);
        board.set(2,2, Tile::Black);
        board.set(3,3, Tile::Black);

        board.set(3,0, Tile::White);
        board.set(3,2, Tile::White);
        board.set(1,2, Tile::White);

        let expected = "B . . . \n . B W . \n  . . B . \n   W . W B \n\
            ---------------\n\
            Black: B, White: W";

        assert_eq!(format!("{}", board), expected);

        board.set(0,1, Tile::Black);
        board.set(0,2, Tile::Black);
        board.set(0,3, Tile::Black);

        let expected2 = "B B B B \n . B W . \n  . . B . \n   W . W B \n\
            ---------------\n\
            Black: B, White: W";

        assert_eq!(format!("{}", board), expected2);
    }

    #[test]
    fn check_win() {
        {
            let mut board = Board::new(4);
            board.set(0,0, Tile::White);
            board.set(0,1, Tile::White);
            board.set(0,2, Tile::White);
            board.set(0,3, Tile::White);

            assert_eq!(board.has_win(), Tile::White);
        }
        {
            let mut board = Board::new(4);
            board.set(0,0, Tile::Black);
            board.set(1,0, Tile::Black);
            board.set(2,0, Tile::Black);
            board.set(3,0, Tile::Black);

            assert_eq!(board.has_win(), Tile::Black);
        }
        {
            let mut board = Board::new(4);
            board.set(0,0, Tile::Black);
            board.set(0,1, Tile::Black);
            board.set(0,2, Tile::Black);
            board.set(0,3, Tile::Black);

            board.set(1,0, Tile::White);
            board.set(2,0, Tile::White);
            board.set(3,0, Tile::White);

            assert_eq!(board.has_win(), Tile::Empty);
        }
        {
            let mut board = Board::new(4);
            board.set(2,0, Tile::White);
            board.set(2,1, Tile::White);
            board.set(1,2, Tile::White);
            board.set(0,3, Tile::White);

            board.set(1,1, Tile::Black);
            board.set(2,2, Tile::Black);
            board.set(1,3, Tile::Black);
            board.set(3,2, Tile::Black);

            assert_eq!(board.has_win(), Tile::White);
        }
        {
            let mut board = Board::new(4);
            board.set(3,0, Tile::White);
            board.set(2,1, Tile::White);
            board.set(1,2, Tile::White);
            board.set(0,3, Tile::White);
            board.set(1,3, Tile::White);
            board.set(2,3, Tile::White);
            board.set(3,3, Tile::White);

            board.set(1,1, Tile::Black);
            board.set(3,2, Tile::Black);
            board.set(0,0, Tile::Black);
            board.set(0,1, Tile::Black);
            board.set(1,1, Tile::Black);
            board.set(2,0, Tile::Black);
            board.set(3,1, Tile::Black);

            assert_eq!(board.has_win(), Tile::White);

            board.set(2,1, Tile::Black);
            assert_eq!(board.has_win(), Tile::Black);

            board.set(2,1, Tile::Empty);
            assert_eq!(board.has_win(), Tile::Empty);

        }
    }
}
