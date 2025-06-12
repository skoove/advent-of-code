use std::collections::HashMap;

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Santa {
    pub santa_position: Position,
    pub robo_santa_position: Position,
    pub houses: HashMap<Position, u32>,
}

pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

impl Santa {
    pub fn move_normal_santa_in_direction(&mut self, direction: MoveDirection) {
        match direction {
            MoveDirection::Up => self.santa_position.y += 1,
            MoveDirection::Down => self.santa_position.y -= 1,
            MoveDirection::Left => self.santa_position.x -= 1,
            MoveDirection::Right => self.santa_position.x += 1,
        };

        self.add_visit_to_house(self.santa_position);
    }

    pub fn move_robo_santa_in_direction(&mut self, direction: MoveDirection) {
        match direction {
            MoveDirection::Up => self.robo_santa_position.y += 1,
            MoveDirection::Down => self.robo_santa_position.y -= 1,
            MoveDirection::Left => self.robo_santa_position.x -= 1,
            MoveDirection::Right => self.robo_santa_position.x += 1,
        };

        self.add_visit_to_house(self.robo_santa_position);
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

        let mut santa_moved = false;
        for direction in directions_parsed {
            if santa_moved == false {
                santa.move_normal_santa_in_direction(direction);
                santa_moved = true;
            } else {
                santa.move_robo_santa_in_direction(direction);
                santa_moved = false;
            }
        }

        santa
    }
}

impl Default for Santa {
    fn default() -> Self {
        let mut santa = Santa {
            santa_position: Position::default(),
            robo_santa_position: Position::default(),
            houses: HashMap::new(),
        };

        santa.houses.insert(santa.santa_position, 1);

        santa
    }
}

#[cfg(test)]
mod tests {
    use crate::Santa;

    #[test]
    fn single_move() {
        let santa = Santa::new_from_directions(">".into());

        assert_eq!(santa.houses.len(), 2);

        for (_, visits) in santa.houses {
            assert_eq!(visits, 1)
        }
    }
}
