use std::io::{stdout, Stdout, Write};

use crossterm::{cursor::MoveTo, style::Print, terminal, ExecutableCommand, QueueableCommand, cursor};
use crate::error::Result;
use crate::snake::Snake;
use crate::point::Point;
use crate::utils;

pub struct Board {
    dimensions: (u16, u16), // (x, y)
    stdout: Stdout,
}

impl Board {
    pub fn new(dimensions: (u16, u16)) -> Self {
        Self { dimensions, stdout: stdout() }
    }

    pub fn terminate(&mut self) {
        self.stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
        self.stdout.execute(cursor::Show).unwrap();
        terminal::disable_raw_mode().unwrap();
    }

    pub fn get_dimensions(&self) -> (u16, u16) {
        self.dimensions
    }

    pub fn init(&mut self) -> Result<()> {
        self.stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        self.stdout.execute(cursor::Hide)?;

        for x in 0..self.dimensions.0 {
            for y in 0..self.dimensions.1 {
                if (x == 0 || x == self.dimensions.0 - 1) || (y == 0 || y == self.dimensions.1 - 1) {
                    self.stdout
                        .queue(MoveTo(x as u16, y as u16))?
                        .queue(Print("#"))?;
                }
            }
        }

        self.stdout.flush()?;
        Ok(())
    }

    pub fn clear_board(&mut self) -> Result<()> {
        for x in 1..self.dimensions.0 - 1 {
            for y in 1..self.dimensions.1 - 1 {
                self.print(' ', x, y)?;
            }
        }
        Ok(())
    }

    pub fn draw_snake(&mut self, snake: &Snake) -> Result<()> {
        for e in snake.get_body() {
            self.print('x', e.x as u16, e.y as u16)?;
        }
        Ok(())
    }

    pub fn draw_food(&mut self, food_position: &Point) -> Result<()> {
        self.print('O', food_position.x as u16, food_position.y as u16)
    }

    pub fn random_point(&self) -> Point {
        Point::new(utils::generate_random_int(1, self.dimensions.0 - 1), utils::generate_random_int(1, self.dimensions.1 - 1))
    }

    pub fn flush(&mut self) -> Result<()> {
        self.stdout.flush()?;
        Ok(())
    }

    fn print<T: std::fmt::Display>(&mut self, value: T, x: u16, y: u16) -> Result<()> {
        self.stdout
            .queue(MoveTo(x, y))?
            .queue(Print(value))?;
        Ok(())
    }
}