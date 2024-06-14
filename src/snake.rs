use std::collections::VecDeque;
use crate::error::{Error, Result};
use crate::point::Point;
use crate::utils::generate_random_int;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum FoodStatus {
    Eaten,
    Unoutched,
}

#[derive(PartialEq)]
pub struct Head {
    pub position: Point,
    pub direction: Direction,
}

pub struct Snake {
    body: VecDeque<Point>,
    current_direction: Direction,
}

impl Snake {
    pub fn new(board_dimensions: (u16, u16)) -> Self {
        let body = VecDeque::from([(generate_random_int(1, board_dimensions.0 - 1), generate_random_int(1, board_dimensions.1 - 1)).into()]);
        Self { body, current_direction: Direction::Left }
    }

    pub fn get_current_head_position(&self) -> Result<&Point> {
        self.body.front().ok_or(Error::EmptySnake)
    }

    pub fn get_current_tail_position(&self) -> Result<&Point> {
        self.body.back().ok_or(Error::EmptySnake)
    }

    pub fn get_body(&self) -> &VecDeque<Point> {
        &self.body
    }

    fn get_direction_deltas(&self, direction: Direction) -> (i16, i16) {
        match direction {
            Direction::Up    => ( 0, -1),
            Direction::Down  => ( 0,  1),
            Direction::Left  => (-1,  0),
            Direction::Right => ( 1,  0),
        }
    }

    pub fn calculate_new_head(&self, new_direction: Option<Direction>) -> Result<Head> {
        let direction = new_direction.unwrap_or(self.current_direction);
        let deltas = self.get_direction_deltas(direction);
        let new_head_position = self.get_current_head_position()?.add_deltas(deltas)?;
        
        Ok(Head { position: new_head_position, direction })
    }

    pub fn eat_food(&mut self, food_position: &Point) -> Result<FoodStatus> {
        if food_position == self.get_current_head_position()? {
            self.body.push_back(*self.get_current_tail_position()?);
            return Ok(FoodStatus::Eaten)
        }
        Ok(FoodStatus::Unoutched)
    }

    pub fn does_point_intersect(&self, point: &Point) -> bool {
        for element in &self.body {
            if point == element {
                return true;
            }
        }
        false
    }

    pub fn get_length(&self) -> usize {
        self.body.len()
    }

    pub fn move_forward(&mut self, new_head: Head) {
        self.current_direction = new_head.direction;
        self.body.push_front(new_head.position);
        self.body.pop_back();
    }
}