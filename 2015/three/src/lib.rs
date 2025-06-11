use std::collections::HashMap;

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Santa {
    pub position: Position,
    pub houses: HashMap<Position, u32>,
}

pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

impl Santa {
    pub fn move_in_direction(&mut self, direction: MoveDirection) {
        match direction {
            MoveDirection::Up => self.position.y += 1,
            MoveDirection::Down => self.position.y -= 1,
            MoveDirection::Left => self.position.x -= 1,
            MoveDirection::Right => self.position.x += 1,
        };

        self.add_visit_to_house(self.position);
    }

    pub fn add_visit_to_house(&mut self, position: Position) {
        match self.houses.get_mut(&position) {
            Some(visits) => *visits += 1,
            None => {
                self.houses.insert(position, 1);
            }
        };
    }

    /// Generate santa from directions with ^v><
    pub fn new_from_directions(directions: String) -> Self {
        let mut directions = directions.clone();
        let directions_parsed = directions.drain(..).map(|character| match character {
            '^' => MoveDirection::Up,
            'v' => MoveDirection::Down,
            '>' => MoveDirection::Left,
            '<' => MoveDirection::Right,
            _ => panic!("invalid move direction: {character}"),
        });

        let mut santa = Santa::default();

        for direction in directions_parsed {
            santa.move_in_direction(direction);
        }

        santa
    }
}

impl Default for Santa {
    fn default() -> Self {
        let mut santa = Santa {
            position: Position::default(),
            houses: HashMap::new(),
        };

        santa.houses.insert(santa.position, 1);

        santa
    }
}
