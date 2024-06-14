use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::terminal;

use crate::snake::{Direction, FoodStatus, Head, Snake};
use crate::point::Point;
use crate::board::Board;
use crate::error::Result;

use std::{thread, time::Duration};

pub struct Game {
    board: Board,
    snake: Snake,
    food_position: Point,
}

#[derive(PartialEq)]
enum ActionDecision {
    Proceed(Head),
    GameOver,
}

enum ActionKey {
    Move(Direction),
    Exit,
    None,
}

impl Game {
    pub fn new() -> Self {
        let mut dimensions = terminal::size().unwrap();
        dimensions.0 = dimensions.0 / 4;
        let board = Board::new(dimensions);
        let snake = Snake::new(dimensions.into());
        let food_position = board.random_point();
        Self { board , snake, food_position }
    }

    pub fn terminate(&mut self) {
        self.board.terminate();
    }

    fn generate_new_food(&mut self) {
        loop {
            self.food_position = self.board.random_point();
            if !self.snake.does_point_intersect(&self.food_position) {
                break;
            }
        }
    }

    fn is_within_coordinates(&self, point: Point) -> bool {
        let board_dimensions = self.board.get_dimensions();
        (point.x > 0 && point.x < board_dimensions.0 - 1) && (point.y > 0 && point.y < board_dimensions.1 - 1)
    }

    fn get_last_event() -> Result<Event> {
        let mut event = event::read()?;
        while event::poll(Duration::from_millis(0))? {
            event = event::read()?;
        }
        Ok(event)
    }

    fn get_action_key(&self, key_event: KeyEvent) -> ActionKey {
        match key_event.code {
            KeyCode::Char('w') => ActionKey::Move(Direction::Up),
            KeyCode::Char('s') => ActionKey::Move(Direction::Down),
            KeyCode::Char('a') => ActionKey::Move(Direction::Left),
            KeyCode::Char('d') => ActionKey::Move(Direction::Right),
            KeyCode::Char('q') => ActionKey::Exit,
            _ => ActionKey::None
        }
    }

    fn process_input(&self) -> Result<ActionKey> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = Self::get_last_event()? {
                return Ok(self.get_action_key(key_event));
            };
        }
        Ok(ActionKey::None)
    }

    fn check_movement(&self, direction: Option<Direction>) -> Result<ActionDecision> {
        let new_head = self.snake.calculate_new_head(direction)?;
        if self.is_within_coordinates(new_head.position) && !self.snake.does_point_intersect(&new_head.position) {
            return Ok(ActionDecision::Proceed(new_head));
        }
        return Ok(ActionDecision::GameOver);
    }

    fn update(&self) -> Result<ActionDecision> {
        match self.process_input()? {
            ActionKey::Move(direction) => self.check_movement(Some(direction)),
            ActionKey::Exit => return Ok(ActionDecision::GameOver),
            _ => self.check_movement(None)
        }
    }

    fn process_food(&mut self) -> Result<()> {
        if let FoodStatus::Eaten = self.snake.eat_food(&self.food_position)? {
            self.generate_new_food();
        }
        Ok(())
    }

    fn render(&mut self) -> Result<()> {
        self.board.clear_board()?;
        self.board.draw_food(&self.food_position)?;
        self.board.draw_snake(&self.snake)?;
        self.board.flush()?;
        Ok(())
    }

    fn game_loop(&mut self) -> Result<usize> {
        loop {
            self.process_food()?;
            match self.update()? {
                ActionDecision::Proceed(new_head) => self.snake.move_forward(new_head),
                ActionDecision::GameOver => return Ok(self.snake.get_length())
            }
            self.render()?;
            thread::sleep(Duration::from_millis(200))
        }
    }

    pub fn run(&mut self) {
        self.board.init().unwrap();
        let exit_message: String;
        match self.game_loop() {
            Ok(score) => exit_message = format!("Score: {}", score),
            Err(e)    => exit_message = format!("Error: {}", e),
        };

        self.terminate();
        println!("{}", exit_message);
    } 
}