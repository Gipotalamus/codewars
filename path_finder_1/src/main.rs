fn main() {
    println!("{}", path_finder("\
            ......\n\
            ......\n\
            ......\n\
            ......\n\
            ......\n\
            ......\
            "));
}

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Maze {
    map: Vec<Vec<Option<()>>>,
    position: Point,
    passed_paths: Vec<Point>,
}

impl Maze {
    fn new(maze: &str) -> Self {
        let map = maze.lines()
            .map(|line| line.chars()
                .map(|c| if c == 'W' { None } else { Some(()) })
                .collect())
            .collect();
        Maze { map, position: Point { x: 0, y: 0 }, passed_paths: vec![] }
    }

    fn is_finished(&self) -> bool {
        self.position == Point { x: self.map.len() - 1, y: self.map[0].len() - 1 }
    }

    fn can_move_to_point(&self, point: &Point) -> bool {
        if self.passed_paths.contains(point) {
            return false;
        }
        if let Some(row) = self.map.get(point.y) {
            if let Some(cell) = row.get(point.x) {
                cell.is_some()
            } else {
                false
            }
        } else {
            false
        }
    }

    fn make_move(&mut self) -> bool {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for (dx, dy) in directions.iter() {
            let new_point = Point {
                x: (self.position.x as isize + dx) as usize,
                y: (self.position.y as isize + dy) as usize,
            };
            if self.can_move_to_point(&new_point) {
                self.position = new_point.clone();
                self.passed_paths.push(new_point);
                return true;
            }
        }

        for prev_point in self.passed_paths.iter().rev() {
            self.position = prev_point.clone();
            for (dx, dy) in directions.iter() {
                let new_point = Point {
                    x: (self.position.x as isize + dx) as usize,
                    y: (self.position.y as isize + dy) as usize,
                };
                if self.can_move_to_point(&new_point) {
                    self.position = new_point.clone();
                    self.passed_paths.push(new_point);
                    return true;
                }
            }            
        }
        false
    }
        
}

fn path_finder(maze: &str) -> bool {
    let mut maze = Maze::new(maze);

    while !maze.is_finished() {
        if !maze.make_move() {
            return false;
        }
        
    }
    true
}
