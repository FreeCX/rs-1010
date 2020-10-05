use crate::extra::Coord;
use crate::random::Random;
use crate::render::fill_rounded_rect;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::collections::{HashMap, HashSet};
use std::time::SystemTime;

#[derive(Clone, Copy)]
pub struct Lines {
    pub x: u32,
    pub y: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Wait,
    ClearLineX(i16, i16),
    ClearLineY(i16, i16),
}

pub struct Field {
    field_size: Coord,
    tile_size: Coord,
    tile_sep: Coord,
    pos: Coord,
    field: HashSet<Coord>,
    colors: HashMap<Coord, Color>,
    cur_state: State,
    all_state: Vec<State>,
    lines: Lines,
}

#[derive(Clone, Eq, PartialEq)]
pub struct Figure {
    blocks: HashSet<Coord>,
    color: Color,
}

pub struct Basket {
    field_size: Coord,
    tile_size: Coord,
    tile_sep: Coord,
    figure: Option<Figure>,
    pos: Coord,
}

pub struct BasketSystem {
    basket: Vec<Basket>,
    current: Option<usize>,
    rnd: Random,
}

impl Lines {
    pub fn empty() -> Lines {
        Lines { x: 0, y: 0 }
    }

    pub fn not_empty(self) -> bool {
        self.x != 0 || self.y != 0
    }
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
            lines: Lines::empty(),
        }
    }

    pub fn get_cell_index(&self, pos: &Coord) -> Coord {
        (*pos - self.pos).floor_frac(self.tile_size + self.tile_sep).normalize(Coord::zero(), self.field_size)
    }

    pub fn get_point_in(&self, pos: &Coord, figure: &Figure) -> Coord {
        let mut norm = self.get_cell_index(pos).normalize(Coord::zero(), self.field_size - figure.max() - 1_i16);
        norm = norm * (self.tile_size + self.tile_sep) + self.pos;
        norm
    }

    pub fn is_point_in(&self, pos: &Coord) -> bool {
        let v = (*pos - self.pos).floor_frac(self.tile_size + self.tile_sep);
        v.x >= 0 && v.x < self.field_size.x && v.y >= 0 && v.y < self.field_size.y
    }

    pub fn set_figure(&mut self, pos: &Coord, figure: &Figure) -> bool {
        if self.cur_state != State::Wait {
            return false;
        }
        let new_figure = figure.shift(self.get_cell_index(pos));
        for Coord { x, y } in &new_figure.blocks {
            if *x >= self.field_size.x || *x < 0 || *y >= self.field_size.y || *y < 0 {
                return false;
            }
        }
        let not_intersect = self.field.intersection(&new_figure.blocks).count() == 0;
        if not_intersect {
            for p in new_figure.blocks {
                self.field.insert(p);
                self.colors.insert(p, figure.color);
            }
        }
        not_intersect
    }

    #[cfg(debug_assertion)]
    pub fn set_vec(&mut self, v: Vec<Vec<u8>>, c: Color) {
        for (y, row) in v.into_iter().enumerate() {
            for (x, item) in row.into_iter().enumerate() {
                if item == 1 {
                    let pos = coord!(x as i16, y as i16);
                    self.field.insert(pos);
                    self.colors.insert(pos, c);
                }
            }
        }
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

    pub fn can_set(&self, figures: Vec<Figure>) -> bool {
        if figures.len() == 0 || self.cur_state != State::Wait {
            return true;
        }
        for figure in figures {
            let x_max = figure.blocks.iter().fold(0, |m, p| m.max(p.x));
            let y_max = figure.blocks.iter().fold(0, |m, p| m.max(p.y));
            for y in 0..self.field_size.y - y_max {
                for x in 0..self.field_size.x - x_max {
                    let new_figure = figure.shift(coord!(x, y));
                    // can set a figure?
                    if self.field.intersection(&new_figure.blocks).count() == 0 {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn clear(&mut self) {
        self.field.clear();
        self.colors.clear();
        self.cur_state = State::Wait;
        self.all_state.clear();
        self.lines = Lines::empty();
    }

    pub fn next_state(&mut self) -> Option<Lines> {
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
                    self.lines.y += 1;
                    self.pop_state()
                } else {
                    self.remove(coord!(x, y));
                    State::ClearLineX(x, y + 1)
                }
            }
            State::ClearLineY(x, y) => {
                if x == self.field_size.x {
                    self.lines.x += 1;
                    self.pop_state()
                } else {
                    self.remove(coord!(x, y));
                    State::ClearLineY(x + 1, y)
                }
            }
        };
        self.cur_state = new_state;
        if self.cur_state == State::Wait {
            if self.lines.not_empty() {
                let lines = self.lines;
                self.lines = Lines::empty();
                Some(lines)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, empty_field_color: Color, radius: i16) -> Result<(), String> {
        for y in 0..self.field_size.y {
            for x in 0..self.field_size.x {
                let pos = coord!(x, y);
                let color = if self.field.contains(&pos) {
                    *self.colors.get(&pos).ok_or("Can't get color")?
                } else {
                    empty_field_color
                };
                let p1 = pos * (self.tile_size + self.tile_sep) + self.pos;
                let p2 = p1 + self.tile_size;
                fill_rounded_rect(canvas, p1, p2, radius, color)?;
            }
        }
        Ok(())
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

    pub fn blocks(&self) -> u32 {
        self.blocks.len() as u32
    }

    pub fn max(&self) -> Coord {
        let (mut max_x, mut max_y) = (0, 0);
        for Coord { x, y } in &self.blocks {
            max_x = max_x.max(*x);
            max_y = max_y.max(*y);
        }
        coord!(max_x, max_y)
    }

    pub fn render(
        &self,
        canvas: &mut Canvas<Window>,
        pos: Coord,
        size: Coord,
        sep: Coord,
        alpha: u8,
        radius: i16,
    ) -> Result<(), String> {
        let color = Color::RGBA(self.color.r, self.color.g, self.color.b, alpha);
        for c in &self.blocks {
            let p1 = *c * (size + sep) + pos;
            let p2 = p1 + size;
            fill_rounded_rect(canvas, p1, p2, radius, color)?;
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
            pos,
        }
    }

    pub fn point_in(&self, pos: Coord) -> bool {
        let p1 = self.pos;
        let p2 = self.field_size * (self.tile_size + self.tile_sep) + p1;
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

    pub fn figure(&self) -> Option<Figure> {
        self.figure.clone()
    }

    pub fn centering(&self, figure: &Figure) -> Coord {
        (self.field_size - figure.max()) >> 1_i16
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, empty_field_color: Color, radius: i16) -> Result<(), String> {
        let wsize = self.tile_size + self.tile_sep;
        let color = empty_field_color;
        for y in 0..self.field_size.y {
            for x in 0..self.field_size.x {
                let p1 = coord!(x, y) * wsize + self.pos;
                let p2 = p1 + wsize - self.tile_sep;
                fill_rounded_rect(canvas, p1, p2, radius, color)?;
            }
        }
        if let Some(figure) = self.figure.clone() {
            let color = figure.color;
            let cen = self.centering(&figure);
            for pos in figure.blocks {
                let p1 = (pos + cen) * wsize + self.pos;
                let p2 = p1 + wsize - self.tile_sep;
                fill_rounded_rect(canvas, p1, p2, radius, color)?;
            }
        }
        Ok(())
    }
}

impl BasketSystem {
    pub fn new(count: u8, field_size: u8, tile_size: u8, tile_sep: u8, pos: Coord, shift: Coord) -> BasketSystem {
        let mut basket = Vec::new();
        let seed = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            // https://xkcd.com/221/
            Err(_) => 4,
        };
        let rnd = Random::new(seed as u32);
        for i in 0..count {
            let bpos = pos + shift * (i as i16);
            basket.push(Basket::init_square(field_size, tile_size, tile_sep, bpos));
        }
        BasketSystem { basket, current: None, rnd }
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
        for index in 0..self.basket.len() {
            let item = self.rnd.rand() as usize % size;
            self.basket[index].push(figures[item].clone());
        }
    }

    #[cfg(debug_assertion)]
    pub fn set_vec(&mut self, figures: Vec<Figure>) {
        for (index, f) in figures.into_iter().enumerate() {
            self.basket[index].push(f);
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

    pub fn figures(&self) -> Vec<Figure> {
        let mut figures = Vec::new();
        for basket in &self.basket {
            if let Some(figure) = basket.figure() {
                figures.push(figure);
            }
        }
        figures
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, empty_field_color: Color, radius: i16) -> Result<(), String> {
        for item in &self.basket {
            item.render(canvas, empty_field_color, radius)?;
        }
        Ok(())
    }
}
