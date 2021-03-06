use std::cmp::Ordering;

use rand::Rng;

use bracket_lib::prelude::*;

use crate::types::Mode;

#[derive(Clone, Debug)]
pub struct GameCell {
    point: PointF,
    symbol: char,
    color: RGB,
    selected: bool,
    destination: Option<Point>,
    mode: Mode,
    tic: f32,
    harmed: bool,
}

impl GameCell {
    pub fn new(x: i32, y: i32, symbol: char, color: RGB) -> Self {
        Self {
            point: PointF::new(x as f32, y as f32),
            symbol,
            color,
            selected: false,
            destination: None,
            mode: Mode::Select,
            tic: 0.0,
            harmed: false,
        }
    }

    pub fn move_pos(&mut self, point: Point, mode: Mode) {
        self.mode = mode;
        self.destination = Some(point);
    }
    pub fn move_towards(&mut self, other: Point) {
        if self.destination.is_none() {
            let a = match self.x().cmp(&other.x) {
                Ordering::Less => 1,
                Ordering::Greater => -1,
                _ => 0,
            };
            let b = match self.y().cmp(&other.y) {
                Ordering::Less => 1,
                Ordering::Greater => -1,
                _ => 0,
            };
            self.destination = Some(Point::new(self.x() + a, self.y() + b));
        }
    }
    pub fn move_to_attacker(&mut self, other: Point) {
        if !self.is_holding() && !self.is_moving() {
            self.move_pos(other, Mode::Attack);
        }
    }
    pub fn stop_moving(&mut self) {
        if !self.is_holding() {
            self.mode = Mode::Select
        };
        self.destination = None;
    }
    pub fn is_moving(&self) -> bool {
        self.mode == Mode::Move
    }

    pub fn hold(&mut self) {
        self.mode = Mode::Hold;
        self.destination = None;
    }
    pub fn is_holding(&self) -> bool {
        self.mode == Mode::Hold
    }

    pub fn update(&mut self, dt: f32, speed: f32) {
        if self.tic > 0.6 {
            self.harmed = false;
        }

        if self.tic > 1.0 {
            self.tic = 0.0;
            self.harmed = false;
        } else {
            self.tic += dt;
        }

        if let Some(dest) = self.destination {
            let distx = dest.x as f32 - self.point.x;
            let disty = dest.y as f32 - self.point.y;
            let dist = (distx * distx + disty * disty).sqrt();

            if dist != 0.0 {
                self.point.x += distx / dist * speed * dt;
                self.point.y += disty / dist * speed * dt;
            }

            if Rect::with_exact(dest.x - 1, dest.y - 1, dest.x + 1, dest.y + 1)
                .point_in_rect(self.point())
            {
                self.stop_moving();
            }
        } else {
            self.point.x = self.point.x.round();
            self.point.y = self.point.y.round();
        }
    }

    /// Randomly move the cell in one of 8 directions
    pub fn bump(&mut self) {
        if self.tic >= 0.1 {
            let (a, b) = match rand::thread_rng().gen_range(0, 7) {
                0 => (0.0, -1.0),
                1 => (1.0, -1.0),
                2 => (1.0, 0.0),
                3 => (1.0, 1.0),
                4 => (0.0, 1.0),
                5 => (-1.0, 1.0),
                6 => (-1.0, 0.0),
                _ => (-1.0, -1.0),
            };
            self.point.x += a;
            self.point.y += b;
            self.tic = 0.0;
        }
    }

    /// Given a positive range value, return the range of the cell as a Rect
    pub fn range_rect(&self, r: u32) -> Rect {
        let r = r as i32 + 1;
        Rect::with_exact(
            self.x() - r,
            self.y() - r,
            self.x() + r + 1,
            self.y() + r + 1,
        )
    }

    /// Select the cell
    pub fn select(&mut self) {
        self.selected = true;
    }
    /// Deselect the cell
    pub fn deselect(&mut self) {
        self.selected = false
    }

    /// Set the harmed status of the cell to true, causing it to appear red
    pub fn set_harmed(&mut self) {
        self.harmed = true;
    }

    pub fn point(&self) -> Point {
        self.point.into()
    }
    pub fn x(&self) -> i32 {
        self.point.x.round() as i32
    }
    pub fn y(&self) -> i32 {
        self.point.y.round() as i32
    }
    pub fn symbol(&self) -> char {
        self.symbol
    }
    /// Return the RGB color of the cell; if harmed return red
    pub fn color(&self) -> RGB {
        if self.harmed {
            RGB::named((255, 0, 0))
        } else {
            self.color
        }
    }
    /// Return a brightened version of the cell's color
    pub fn color_bright(&self) -> RGB {
        RGB::from_f32(self.color.r * 1.5, self.color.g * 1.5, self.color.b * 1.5)
    }
    /// Return a black background for the cell, but black if selected
    pub fn bg_color(&self) -> RGB {
        if self.selected {
            RGB::from_u8(255, 255, 255)
        } else {
            RGB::new()
        }
    }
    pub fn selected(&self) -> bool {
        self.selected
    }
}
