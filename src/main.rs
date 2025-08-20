fn main() {
    let knight = Knight {
        position: Position(0, 0),
        target: Position(7, 7),
        moves: Vec::new(),
    };

    let mut result = knight.move_in_all_directions().unwrap_or(vec![]);
    result.sort_by(|a, b| a.moves.len().cmp(&b.moves.len()));
    println!("Result: {:?}", result.first().unwrap());
}

#[derive(Debug, Clone)]
struct Knight {
    position: Position,
    target: Position,
    moves: Vec<Position>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Position(i8, i8);

impl Knight {
    fn make_move(&mut self, position: Position) -> Option<()> {
        if self.moves.contains(&position)
            || position.0 < 0
            || position.0 > 7
            || position.1 < 0
            || position.1 > 7
        {
            return None;
        }
        self.position = position;
        self.moves.push(position);
        Some(())
    }

    fn check_target(&self) -> bool {
        self.position == self.target
    }

    fn move_in_all_directions(&self) -> Option<Vec<Knight>> {
        let mut result = Vec::new();

        if self.moves.len() > 8 {
            return None;
        }

        let increments = [
            (-2, 1),
            (-2, -1),
            (2, 1),
            (2, -1),
            (-1, 2),
            (-1, -2),
            (1, 2),
            (1, -2),
        ];

        for (x, y) in increments {
            let mut knight = self.clone();

            if knight
                .make_move(Position(self.position.0 + x, self.position.1 + y))
                .is_some()
            {
                if knight.check_target() {
                    result.push(knight);
                } else {
                    if let Some(sub_result) = knight.move_in_all_directions() {
                        result.extend(sub_result);
                    }
                }
            }
        }
        // println!("subbb {:?}", &result);

        if result.len() == 0 {
            return None;
        }

        Some(result.into_iter().map(|el| el.clone()).collect())
    }
}
