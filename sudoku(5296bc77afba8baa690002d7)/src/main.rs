use std::collections::HashSet;

fn main() {
    let mut puzzle = [
        [0, 0, 8, 0, 3, 0, 5, 4, 0],
                [3, 0, 0, 4, 0, 7, 9, 0, 0],
                [4, 1, 0, 0, 0, 8, 0, 0, 2],
                [0, 4, 3, 5, 0, 2, 0, 6, 0],
                [5, 0, 0, 0, 0, 0, 0, 0, 8],
                [0, 6, 0, 3, 0, 9, 4, 1, 0],
                [1, 0, 0, 8, 0, 0, 0, 2, 7],
                [0, 0, 5, 6, 0, 3, 0, 0, 4],
                [0, 2, 9, 0, 7, 0, 8, 0, 0],
    ];
    sudoku(&mut puzzle);
}

fn sudoku(puzzle: &mut [[u8; 9]; 9]) {
    let mut unknown_cells = get_unknown_cells(puzzle);
    let mut was_changed = true;
    while was_changed {
        was_changed = false;
        unknown_cells = get_unknown_cells(puzzle);
        for cell in &mut unknown_cells {
            if cell.is_determined() {
                let value = *cell.possible_values.iter().next().unwrap();
                puzzle[cell.position.0][cell.position.1] = value;
                was_changed = true;
            }
        }
    }
}

fn get_unknown_cells(puzzle: &[[u8; 9]; 9]) -> Vec<UnknownCell> {
    let mut unknown_cells = Vec::new();
    for row in 0..9 {
        for col in 0..9 {
            if puzzle[row][col] == 0 {
                let possible_values = get_possible_values(puzzle, (row, col));
                unknown_cells.push(UnknownCell::new((row, col), possible_values));
            }
        }
    }
    unknown_cells
}

fn get_possible_values(puzzle: &[[u8; 9]; 9], position: (usize, usize)) -> HashSet<u8> {
    let mut possible_values: HashSet<u8> = (1..=9).collect();
    let (row, col) = position;
    for v in 0..9 {
        if puzzle[row][v] != 0 {
            possible_values.remove(&puzzle[row][v]);
        }
        if puzzle[v][col] != 0 {
            possible_values.remove(&puzzle[v][col]);
        }
    }
    let box_start_row = (row / 3) * 3;
    let box_start_col = (col / 3) * 3;
    for r in box_start_row..box_start_row + 3 {
        for c in box_start_col..box_start_col + 3 {
            if puzzle[r][c] != 0 {
                possible_values.remove(&puzzle[r][c]);
            }
        }
    }
    possible_values
}

struct UnknownCell {
    position: (usize, usize),
    possible_values: HashSet<u8>,
}

impl UnknownCell {
    fn new(position: (usize, usize), possible_values: HashSet<u8>) -> Self {
        Self { position, possible_values }
    }

    fn is_determined(&self) -> bool {
        self.possible_values.len() == 1
    }
}
