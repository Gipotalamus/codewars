fn main() {
    println!("{:?}", spiralize(8));
}

// [
//     [1, 1, 1, 1, 1, 1, 1, 1],
//     [0, 0, 0, 0, 0, 0, 0, 1],
//     [1, 1, 1, 1, 1, 1, 0, 1],
//     [1, 0, 0, 0, 0, 1, 0, 1],
//     [1, 0, 1, 0, 0, 1, 0, 1],
//     [1, 0, 1, 1, 1, 1, 0, 1],
//     [1, 0, 0, 0, 0, 0, 0, 1],
//     [1, 1, 1, 1, 1, 1, 1, 1],
// ]

fn spiralize(size: usize) -> Vec<Vec<i8>> {
    let mut spiralizer = Spiralizer::new(size);
    while !spiralizer.finished {
        spiralizer.make_move();
    }
    spiralizer.board
}

struct Spiralizer {
    position: (usize, usize),
    direction: (isize, isize),
    board: Vec<Vec<i8>>,
    steps_between_turn: usize,
    finished: bool,
}

impl Spiralizer {
    fn new(size: usize) -> Self {
        let mut spiralizer = Self {
            board: vec![vec![0; size]; size],
            position: (0, 0),
            direction: (0, 1),
            steps_between_turn: 0,
            finished: false,
        };
        spiralizer.board[0][0] = 1;
        spiralizer
    }

    fn make_move(&mut self) {
        let next_position = self.get_next_position();

        if next_position.is_none() {
            self.turn();
            return ();
        }
        self.position = next_position.unwrap();
        self.board[self.position.0][self.position.1] = 1;
        self.steps_between_turn += 1;
    }

    fn get_next_position(&self) -> Option<(usize, usize)> {
        let (x, y) = self.position;
        let (dx, dy) = self.direction;
        let next_x = x as isize + dx;
        let next_y = y as isize + dy;
        let next_next_x = next_x + dx;
        let next_next_y = next_y + dy;
        if next_x < 0
            || next_y < 0
            || next_x as usize >= self.board.len()
            || next_y as usize >= self.board.len()
            || (next_next_x >= 0
                && next_next_x < self.board.len() as isize
                && next_next_y >= 0
                && next_next_y < self.board.len() as isize
                && self.board[next_next_x as usize][next_next_y as usize] == 1)
        {
            None
        } else {
            Some((next_x as usize, next_y as usize))
        }
    }

    fn turn(&mut self) {
        let (dx, dy) = self.direction;
        self.direction = match (dx, dy) {
            (0, 1) => (1, 0),
            (1, 0) => (0, -1),
            (0, -1) => (-1, 0),
            _ => (0, 1),
        };
        if self.steps_between_turn <= 1 {
            self.finished = true;
        }
        self.steps_between_turn = 0;
    }
}
