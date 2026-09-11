use std::collections::{HashSet, VecDeque};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum PieceType {
    Red2x2,
    Yellow1x2,
    Yellow2x1,
    Blue1x1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Piece {
    piece_type: PieceType,
    x: u8,
    y: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Board {
    pieces: Vec<Piece>,
}

impl Board {
    fn new() -> Self {
        let mut pieces = Vec::new();
        // Red 2x2
        pieces.push(Piece { piece_type: PieceType::Red2x2, x: 1, y: 0 });
        // Yellow 1x2 vertical
        pieces.push(Piece { piece_type: PieceType::Yellow1x2, x: 0, y: 0 });
        pieces.push(Piece { piece_type: PieceType::Yellow1x2, x: 3, y: 0 });
        pieces.push(Piece { piece_type: PieceType::Yellow1x2, x: 0, y: 2 });
        pieces.push(Piece { piece_type: PieceType::Yellow1x2, x: 3, y: 2 });
        // Yellow 2x1 horizontal
        pieces.push(Piece { piece_type: PieceType::Yellow2x1, x: 1, y: 2 });
        // Blue 1x1
        pieces.push(Piece { piece_type: PieceType::Blue1x1, x: 1, y: 3 });
        pieces.push(Piece { piece_type: PieceType::Blue1x1, x: 2, y: 3 });
        pieces.push(Piece { piece_type: PieceType::Blue1x1, x: 1, y: 4 });
        pieces.push(Piece { piece_type: PieceType::Blue1x1, x: 2, y: 4 });

        Board { pieces }
    }

    fn to_grid(&self) -> [[i8; 4]; 5] {
        let mut grid = [[-1; 4]; 5];
        for (idx, p) in self.pieces.iter().enumerate() {
            let (w, h) = match p.piece_type {
                PieceType::Red2x2 => (2, 2),
                PieceType::Yellow1x2 => (1, 2),
                PieceType::Yellow2x1 => (2, 1),
                PieceType::Blue1x1 => (1, 1),
            };
            for dx in 0..w {
                for dy in 0..h {
                    grid[(p.y + dy) as usize][(p.x + dx) as usize] = idx as i8;
                }
            }
        }
        grid
    }

    fn canonicalize(&mut self) {
        self.pieces.sort_by(|a, b| {
            if a.piece_type != b.piece_type {
                return a.piece_type.cmp(&b.piece_type);
            }
            (a.y, a.x).cmp(&(b.y, b.x))
        });
    }

    fn is_goal(&self) -> bool {
        for p in &self.pieces {
            if p.piece_type == PieceType::Red2x2 {
                return p.x == 1 && p.y == 3;
            }
        }
        false
    }

    fn get_moves(&self) -> Vec<Board> {
        let grid = self.to_grid();
        let mut moves = Vec::new();

        for (idx, p) in self.pieces.iter().enumerate() {
            let (w, h) = match p.piece_type {
                PieceType::Red2x2 => (2, 2),
                PieceType::Yellow1x2 => (1, 2),
                PieceType::Yellow2x1 => (2, 1),
                PieceType::Blue1x1 => (1, 1),
            };

            // Try moving in 4 directions
            let dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)];
            for (dx, dy) in dirs {
                let nx = p.x as i8 + dx;
                let ny = p.y as i8 + dy;

                if nx >= 0 && ny >= 0 && nx + w as i8 <= 4 && ny + h as i8 <= 5 {
                    let mut possible = true;
                    for ox in 0..w {
                        for oy in 0..h {
                            let tx = nx + ox as i8;
                            let ty = ny + oy as i8;
                            let occupant = grid[ty as usize][tx as usize];
                            if occupant != -1 && occupant != idx as i8 {
                                possible = false;
                                break;
                            }
                        }
                        if !possible { break; }
                    }

                    if possible {
                        let mut next_board = self.clone();
                        next_board.pieces[idx].x = nx as u8;
                        next_board.pieces[idx].y = ny as u8;
                        next_board.canonicalize();
                        moves.push(next_board);
                    }
                }
            }
        }
        moves
    }
}

fn solve() {
    let mut start = Board::new();
    start.canonicalize();

    if start.is_goal() {
        println!("Already at goal");
        return;
    }

    let mut queue = VecDeque::new();
    queue.push_back((start.clone(), vec![start.clone()]));

    let mut visited = HashSet::new();
    visited.insert(start);

    while let Some((current, path)) = queue.pop_front() {
        for next in current.get_moves() {
            if !visited.contains(&next) {
                if next.is_goal() {
                    println!("Found solution in {} moves", path.len());
                    for (i, board) in path.iter().enumerate() {
                        println!("Step {}:", i);
                        print_board(board);
                    }
                    println!("Final Step:");
                    print_board(&next);
                    return;
                }
                visited.insert(next.clone());
                let mut next_path = path.clone();
                next_path.push(next.clone());
                queue.push_back((next, next_path));
            }
        }
    }
    println!("No solution found");
}

fn print_board(board: &Board) {
    let grid = board.to_grid();
    for row in grid.iter() {
        for &cell in row.iter() {
            if cell == -1 {
                print!(". ");
            } else {
                let p = &board.pieces[cell as usize];
                match p.piece_type {
                    PieceType::Red2x2 => print!("R "),
                    PieceType::Yellow1x2 => print!("Y "),
                    PieceType::Yellow2x1 => print!("y "),
                    PieceType::Blue1x1 => print!("B "),
                }
            }
        }
        println!();
    }
    println!();
}

fn main() {
    solve();
}
