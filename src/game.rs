#![allow(dead_code, unused_variables)]
use std::collections::{HashMap, HashSet};

use crate::extra::Coord;
use crate::render::fill_rounded_rect;
use crate::random::Random;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Wait,
    ClearLineX(i16, i16),
    ClearLineY(i16, i16),
}

#[derive(Debug)]
pub struct Field {
    field_size: Coord,
    tile_size: Coord,
    tile_sep: Coord,
    pos: Coord,
    field: HashSet<Coord>,
    pub colors: HashMap<Coord, Color>,
    cur_state: State,
    all_state: Vec<State>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Figure {
    blocks: HashSet<Coord>,
    color: Color,
}

#[derive(Debug)]
pub struct Basket {
    field_size: Coord,
    tile_size: Coord,
    tile_sep: Coord,
    figure: Option<Figure>,
    pos: Coord
}

#[derive(Debug)]
pub struct BasketSystem {
    basket: Vec<Basket>,
    current: Option<usize>,
    rnd: Random
}

impl Field {
    pub fn init_square(pole_size: u8, tile_size: u8, tile_sep: u8, pos: Coord) -> Field {
        Field {
            field_size: coord!(pole_size as i16, pole_size as i16),
            tile_size: coord!(tile_size as i16, tile_size as i16),
            tile_sep: coord!(tile_sep as i16, tile_sep as i16),
            pos,
            field: HashSet::new(),
            colors: HashMap::new(),
            cur_state: State::Wait,
            all_state: Vec::new(),
        }
    }

    pub fn set_figure(&mut self, x: i32, y: i32, figure: &Figure) -> bool {
        if self.cur_state != State::Wait {
            return false;
        }
        let nx = x as f32 - self.pos.x as f32;
        let ny = y as f32 - self.pos.y as f32;
        let xi = (nx / (self.tile_size.x as f32 + self.tile_sep.x as f32)).floor() as i16;
        let yi = (ny / (self.tile_size.y as f32 + self.tile_sep.y as f32)).floor() as i16;
        let new_figure = figure.shift(coord!(xi, yi));
        for Coord { x, y } in &new_figure.blocks {
            if *x >= self.field_size.x || *x < 0 || *y >= self.field_size.y || *y < 0 {
                return false;
            }
        }
        let intersect = self.field.intersection(&new_figure.blocks).count() == 0;
        if intersect {
            for p in new_figure.blocks {
                self.field.insert(p);
                self.colors.insert(p, figure.color);
            }
        }
        intersect
    }

    fn check_line_h(&self, index: u8) -> Option<bool> {
        if index > self.field_size.x as u8 {
            return None;
        }
        let mut counter = 0;
        for i in 0..self.field_size.x {
            if self.field.contains(&coord!(i, index as i16)) {
                counter += 1;
            }
        }
        if counter == self.field_size.x {
            Some(true)
        } else {
            Some(false)
        }
    }

    fn check_line_v(&self, index: u8) -> Option<bool> {
        if index > self.field_size.y as u8 {
            return None;
        }
        let mut counter = 0;
        for i in 0..self.field_size.y {
            if self.field.contains(&coord!(index as i16, i)) {
                counter += 1;
            }
        }
        if counter == self.field_size.y {
            Some(true)
        } else {
            Some(false)
        }
    }

    fn pop_state(&mut self) -> State {
        if self.all_state.len() > 0 {
            self.all_state.pop().unwrap()
        } else {
            State::Wait
        }
    }

    fn remove(&mut self, p: Coord) {
        self.field.remove(&p);
        self.colors.remove(&p);
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, empty_field_color: Color) -> Result<(), String> {
        let (x_sep, y_sep) = (self.tile_sep.x, self.tile_sep.y);
        let (w, h) = (self.tile_size.x + x_sep, self.tile_size.y + x_sep);
        let (x_shift, y_shift) = (self.pos.x, self.pos.y);
        for y in 0..self.field_size.y {
            for x in 0..self.field_size.x {
                let pos = coord!(x, y);
                let color = if self.field.contains(&pos) {
                    *self.colors.get(&pos).ok_or("Can't get color")?
                } else {
                    empty_field_color
                };
                let (xp, yp) = (x * w + x_shift, y * h + y_shift);
                fill_rounded_rect(canvas, coord!(xp, yp), coord!(xp + w - x_sep, yp + h - y_sep), 4, color)?;
            }
        }
        Ok(())
    }

    pub fn next_state(&mut self) {
        let new_state = match self.cur_state {
            State::Wait => {
                for x in 0..self.field_size.x {
                    if let Some(true) = self.check_line_v(x as u8) {
                        self.all_state.push(State::ClearLineX(x, 0));
                    }
                }
                for y in 0..self.field_size.y {
                    if let Some(true) = self.check_line_h(y as u8) {
                        self.all_state.push(State::ClearLineY(0, y));
                    }
                }
                self.pop_state()
            }
            State::ClearLineX(x, y) => {
                if y == self.field_size.y {
                    self.pop_state()
                } else {
                    self.remove(coord!(x, y));
                    State::ClearLineX(x, y + 1)
                }
            }
            State::ClearLineY(x, y) => {
                if x == self.field_size.x {
                    self.pop_state()
                } else {
                    self.remove(coord!(x, y));
                    State::ClearLineY(x + 1, y)
                }
            }
        };
        self.cur_state = new_state;
    }
}

impl Figure {
    pub fn from_slice(coords: &[Coord], color: Color) -> Figure {
        let mut blocks = HashSet::new();
        for p in coords {
            blocks.insert(*p);
        }
        Figure { blocks, color }
    }

    pub fn shift(&self, pos: Coord) -> Figure {
        let mut blocks = HashSet::new();
        for block in self.blocks.clone() {
            blocks.insert(pos + block);
        }
        Figure { blocks, color: self.color }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, pos: Coord, size: Coord, sep: Coord) -> Result<(), String> {
        let (x_sep, y_sep) = (sep.x, sep.y);
        let (w, h) = (size.x + x_sep, size.y + x_sep);
        let (x_shift, y_shift) = (pos.x, pos.y);
        for c in &self.blocks {
            let (xp, yp) = (c.x * w + x_shift, c.y * h + y_shift);
            fill_rounded_rect(canvas, coord!(xp, yp), coord!(xp + w - x_sep, yp + h - y_sep), 4, self.color)?;
        }
        Ok(())
    }
}


impl Basket {
    pub fn init_square(field_size: u8, tile_size: u8, tile_sep: u8, pos: Coord) -> Basket {
        Basket {
            field_size: coord!(field_size as i16, field_size as i16),
            tile_size: coord!(tile_size as i16, tile_size as i16),
            tile_sep: coord!(tile_sep as i16, tile_sep as i16),
            figure: None,
            pos
        }
    }

    pub fn point_in(&self, pos: Coord) -> bool {
        let p1 = self.pos;
        let p2 = coord!(
            self.field_size.x * (self.tile_size.x + self.tile_sep.x) + p1.x,
            self.field_size.y * (self.tile_size.y + self.tile_sep.y) + p1.y
        );
        pos.x >= p1.x && pos.x <= p2.x && pos.y >= p1.y && pos.y <= p2.y
    }

    pub fn push(&mut self, figure: Figure) {
        self.figure = Some(figure);
    }

    pub fn pop(&mut self) -> Option<Figure> {
        let figure = self.figure.clone();
        self.figure = None;
        figure
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, empty_field_color: Color) -> Result<(), String> {
        let (x_sep, y_sep) = (self.tile_sep.x, self.tile_sep.y);
        let (w, h) = (self.tile_size.x + x_sep, self.tile_size.y + x_sep);
        let (x_shift, y_shift) = (self.pos.x, self.pos.y);
        for y in 0..self.field_size.y {
            for x in 0..self.field_size.x {
                let color = empty_field_color;
                let (xp, yp) = (x * w + x_shift, y * h + y_shift);
                fill_rounded_rect(canvas, coord!(xp, yp), coord!(xp + w - x_sep, yp + h - y_sep), 4, color)?;
            }
        }
        if let Some(figure) = self.figure.clone() {
            let color = figure.color;
            for Coord { x, y } in figure.blocks {
                let (xp, yp) = (x * w + x_shift, y * h + y_shift);
                fill_rounded_rect(canvas, coord!(xp, yp), coord!(xp + w - x_sep, yp + h - y_sep), 4, color)?;
            }
        }
        Ok(())
    }
}

impl BasketSystem {
    pub fn new(count: u8, field_size: u8, tile_size: u8, tile_sep: u8, pos: Coord, shift: Coord) -> BasketSystem {
        let mut basket = Vec::new();
        // TODO: use normal seed value
        let rnd = Random::new(1234);
        for i in 0..count {
            let bpos = pos + shift * (i as i16);
            basket.push(Basket::init_square(field_size, tile_size, tile_sep, bpos));
        }
        BasketSystem {
            basket,
            current: None,
            rnd
        }
    }

    pub fn get(&mut self, pos: Coord) -> Option<Figure> {
        for (index, item) in self.basket.iter_mut().enumerate() {
            if item.point_in(pos) {
                self.current = Some(index);
                return item.pop();
            }
        }
        None
    }

    pub fn ret(&mut self, figure: Figure) {
        if let Some(index) = self.current {
            self.basket[index].push(figure);
            self.current = None;
        }
    }

    pub fn fill(&mut self, figures: &Vec<Figure>) {
        let size = figures.len();
        let idx = [self.rnd.rand() as usize % size, self.rnd.rand() as usize % size, self.rnd.rand() as usize % size];
        for (index, item) in idx.iter().enumerate() {
            self.basket[index].push(figures[*item].clone());
        }
    }

    pub fn check_and_refill(&mut self, figures: &Vec<Figure>) {
        for item in &self.basket {
            if item.figure != None {
                return;
            }
        }
        self.fill(figures);
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, empty_field_color: Color) -> Result<(), String> {
        for item in &self.basket {
            item.render(canvas, empty_field_color)?;
        }
        Ok(())
    }
}