use std::collections::{HashMap, HashSet};
use std::mem;
use std::time::SystemTime;

use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::consts::{FAKE_K, GET_COLOR_ERROR, TILE_CLEAN_ANIMATION_SIZE};
use crate::extra::{fake_contrast, BlendColor, Coord, RectData};
use crate::random::Random;
use crate::render::*;

type Blocks = HashSet<Coord>;

#[derive(Clone, Copy)]
pub struct Lines {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum State {
    Wait,
    Clear(u8),
}

pub struct Field {
    pub textures: HashMap<i16, RectData>,
    pub field_size: Coord,
    tile_size: Coord,
    tile_sep: Coord,
    pos: Coord,
    field: HashSet<Coord>,
    colors: HashMap<Coord, Color>,
    state: State,
    clear: Blocks,
    lines: Lines,
}

#[derive(Clone, Eq, PartialEq)]
pub struct Figure {
    blocks: HashSet<Coord>,
    color: Color,
    // not the best way to identify figure
    pub index: u8,
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
    texture: RectData,
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
    pub fn init_square(pole_size: u8, tile_size: u8, tile_sep: u8, steps: i16, radius: i16, pos: Coord) -> Field {
        // alloc all size tiles
        let mut textures = HashMap::new();
        for i in (8..=tile_size + 2).step_by(2) {
            let block = build_rounded_rect(coord!(), coord!(i as i16), steps, radius);
            textures.insert(i as i16, block);
        }

        Field {
            field_size: coord!(pole_size as i16),
            tile_size: coord!(tile_size as i16),
            tile_sep: coord!(tile_sep as i16),
            field: HashSet::new(),
            colors: HashMap::new(),
            state: State::Wait,
            clear: Blocks::new(),
            lines: Lines::empty(),
            pos,
            textures,
        }
    }

    pub fn set(&mut self, pos: Coord, color: Color) {
        self.field.insert(pos);
        self.colors.insert(pos, color);
    }

    pub fn unset(&mut self, pos: &Coord) {
        self.field.remove(pos);
        self.colors.remove(pos);
    }

    pub fn is_set(&self, pos: &Coord) -> bool {
        self.field.contains(pos)
    }

    pub fn get_cell_index(&self, pos: &Coord) -> Coord {
        (*pos - self.pos).floor_frac(self.tile_size + self.tile_sep).normalize(coord!(), self.field_size)
    }

    pub fn get_point_in(&self, pos: &Coord, figure: &Figure) -> Coord {
        let norm = self.get_cell_index(pos).normalize(coord!(), self.field_size - figure.max() - 1_i16);
        norm * (self.tile_size + self.tile_sep) + self.pos
    }

    pub fn get_color(&self, pos: &Coord) -> Option<&Color> {
        self.colors.get(pos)
    }

    pub fn is_point_in(&self, pos: &Coord) -> bool {
        let v = (*pos - self.pos).floor_frac(self.tile_size + self.tile_sep);
        v.x >= 0 && v.x < self.field_size.x && v.y >= 0 && v.y < self.field_size.y
    }

    pub fn set_figure(&mut self, pos: &Coord, figure: &Figure) -> bool {
        if self.state != State::Wait {
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
                self.set(p, figure.color);
            }
        }
        not_intersect
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
        Some(counter == self.field_size.x)
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
        Some(counter == self.field_size.y)
    }

    fn remove_blocks(&mut self) {
        for p in mem::take(&mut self.clear) {
            self.unset(&p);
        }
    }

    pub fn can_set(&self, figures: Vec<Figure>) -> bool {
        if figures.is_empty() || self.state != State::Wait {
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

    pub fn is_empty(&self) -> bool {
        self.field.is_empty()
    }

    pub fn clear(&mut self) {
        self.state = State::Clear(TILE_CLEAN_ANIMATION_SIZE);
        self.lines = Lines::empty();
        self.clear = Blocks::new();

        for pos in &self.field {
            self.clear.insert(*pos);
        }
    }

    pub fn update_state(&mut self, x: i16, y: i16, p: u8) {
        match &self.state {
            State::Wait => self.state = State::Clear(p),
            State::Clear(_) => {}
        };
        self.clear.insert(coord!(x, y));
    }

    pub fn next_state(&mut self) -> Option<Lines> {
        let new_state = match self.state {
            State::Wait => {
                // calc x lines
                for x in 0..self.field_size.x {
                    if let Some(true) = self.check_line_v(x as u8) {
                        for y in 0..self.field_size.y {
                            self.update_state(x, y, TILE_CLEAN_ANIMATION_SIZE);
                        }
                        self.lines.x += 1;
                    }
                }
                // and y
                for y in 0..self.field_size.y {
                    if let Some(true) = self.check_line_h(y as u8) {
                        for x in 0..self.field_size.x {
                            self.update_state(x, y, TILE_CLEAN_ANIMATION_SIZE);
                        }
                        self.lines.y += 1;
                    }
                }
                self.state
            }
            State::Clear(p) => {
                if p > 0 {
                    // animation step
                    State::Clear(p - 1)
                } else {
                    // animation is finished
                    self.remove_blocks();
                    State::Wait
                }
            }
        };

        self.state = new_state;
        if self.state == State::Wait {
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

    pub fn render(&self, surface: &mut Canvas<Window>, empty: Color, bg: Color) -> Result<(), String> {
        for y in 0..self.field_size.y {
            for x in 0..self.field_size.x {
                let pos = coord!(x, y);
                let color =
                    if self.field.contains(&pos) { *self.colors.get(&pos).ok_or(GET_COLOR_ERROR)? } else { empty };
                let fg = fake_contrast(color, FAKE_K);
                let fbe = fake_contrast(empty, FAKE_K);

                let position = pos * (self.tile_size + self.tile_sep) + self.pos;
                // block shift size
                let shift_pos = match &self.state {
                    State::Clear(p) => {
                        if self.clear.contains(&coord!(x, y)) {
                            coord!(TILE_CLEAN_ANIMATION_SIZE as i16 - *p as i16)
                        } else {
                            coord!()
                        }
                    }
                    State::Wait => coord!(),
                };

                let (shadow_color, blend_color) = if !shift_pos.is_zero() {
                    // color in animation
                    (fbe.into(), BlendColor::blend(color, empty))
                } else {
                    // color in static
                    (BlendColor::blend(fg, bg), BlendColor::blend(color, fbe))
                };

                // draw shadow / field background
                let data = self.textures[&(self.tile_size.x + 2)].shift(position);
                fill_rounded_rect_from(surface, &data, shadow_color)?;

                // draw only set figures
                if self.field.contains(&pos) {
                    let tile = self.tile_size.x - 2 * shift_pos.x;
                    let data = self.textures[&tile].shift(position + shift_pos - 2_i16);
                    fill_rounded_rect_from(surface, &data, blend_color)?;
                }
            }
        }
        Ok(())
    }
}

impl Figure {
    pub fn from_slice(index: u8, coords: &[Coord], color: Color) -> Figure {
        let mut blocks = HashSet::new();
        for p in coords {
            blocks.insert(*p);
        }
        Figure { blocks, color, index }
    }

    pub fn shift(&self, pos: Coord) -> Figure {
        let mut blocks = HashSet::new();
        for block in &self.blocks {
            blocks.insert(pos + *block);
        }
        Figure { blocks, color: self.color, index: self.index }
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
        &self, surface: &mut Canvas<Window>, texture: &RectData, pos: Coord, size: Coord, sep: Coord, alpha: u8,
    ) -> Result<(), String> {
        let color = Color::RGBA(self.color.r, self.color.g, self.color.b, alpha);
        let fake = fake_contrast(color, FAKE_K);
        for c in &self.blocks {
            let position = *c * (size + sep) + pos;
            let tex = texture.shift(position);
            // draw shadow
            fill_rounded_rect_from(surface, &tex, fake.into())?;
            let tex = texture.shift(position - 2_i16);
            // draw figure
            fill_rounded_rect_from(surface, &tex, color.into())?;
        }
        Ok(())
    }
}

impl Basket {
    pub fn init_square(field_size: u8, tile_size: u8, tile_sep: u8, pos: Coord) -> Basket {
        Basket {
            field_size: coord!(field_size as i16),
            tile_size: coord!(tile_size as i16),
            tile_sep: coord!(tile_sep as i16),
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
        self.figure.take()
    }

    pub fn remove(&mut self) {
        self.figure = None;
    }

    pub fn figure(&self) -> Option<Figure> {
        self.figure.clone()
    }

    pub fn centering(&self, figure: &Figure) -> Coord {
        (self.field_size - figure.max()) >> 1_i16
    }

    pub fn render(
        &self, surface: &mut Canvas<Window>, texture: &RectData, empty: Color, bg: Color,
    ) -> Result<(), String> {
        let wsize = self.tile_size + self.tile_sep;
        let fake = fake_contrast(empty, FAKE_K);
        for y in 0..self.field_size.y {
            for x in 0..self.field_size.x {
                let position = coord!(x, y) * wsize + self.pos;
                // draw background
                let tex = texture.shift(position);
                fill_rounded_rect_from(surface, &tex, BlendColor::blend(fake, bg))?;
            }
        }
        if let Some(figure) = &self.figure {
            let color = figure.color;
            let fake = fake_contrast(color, FAKE_K);
            let cen = self.centering(figure);
            for pos in &figure.blocks {
                let position = (*pos + cen) * wsize + self.pos;
                // draw shadow
                let tex = texture.shift(position);
                fill_rounded_rect_from(surface, &tex, BlendColor::blend(fake, bg))?;
                // draw figure
                let tex = texture.shift(position - 1_i16);
                fill_rounded_rect_from(surface, &tex, BlendColor::blend(color, bg))?;
            }
        }
        Ok(())
    }
}

impl BasketSystem {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        count: u8, field_size: u8, tile_size: u8, tile_sep: u8, steps: i16, radius: i16, pos: Coord, shift: Coord,
    ) -> BasketSystem {
        let mut basket = Vec::new();
        let texture = build_rounded_rect(coord!(), coord!(tile_size as i16), steps, radius);
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
        BasketSystem { basket, current: None, rnd, texture }
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

    pub fn set(&mut self, index: usize, figure: Figure) {
        self.basket[index].push(figure);
    }

    pub fn pop(&mut self, index: usize) {
        self.basket[index].pop();
    }

    pub fn clear(&mut self) {
        for fig in self.basket.iter_mut() {
            fig.remove();
        }
    }

    pub fn ret(&mut self, figure: Figure) {
        if let Some(index) = self.current {
            self.set(index, figure);
            self.current = None;
        }
    }

    pub fn rnd_fill(&mut self, figures: &[Figure]) {
        let size = figures.len();
        for index in 0..self.basket.len() {
            let item = self.rnd.rand() as usize % size;
            self.set(index, figures[item].clone());
        }
    }

    pub fn check_and_refill(&mut self, figures: &[Figure]) {
        for item in &self.basket {
            if item.figure.is_some() {
                return;
            }
        }
        self.rnd_fill(figures);
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

    pub fn destroy(self) -> Vec<Basket> {
        self.basket
    }

    pub fn render(
        &self, surface: &mut Canvas<Window>, empty_field_color: Color, bg_color: Color,
    ) -> Result<(), String> {
        for item in &self.basket {
            item.render(surface, &self.texture, empty_field_color, bg_color)?;
        }
        Ok(())
    }
}
