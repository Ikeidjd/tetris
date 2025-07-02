use std::{collections::HashMap, fs};

use ggez::{graphics::{Canvas, Color}, input::keyboard::KeyCode, Context};

use crate::{board::Board, util::{self, CELL_SIZE, GRID_SIZE}};

type TetoRot = Vec<(i32, i32)>;
type TetoRots = Vec<TetoRot>;

#[derive(Debug, Clone)]
pub enum TetoType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L
}

pub struct Teto {
    t: TetoType,
    rots: TetoRots,
    rot: usize,
    i: i32,
    j: i32,
    fall_duration: f32,
    prev_fall_time: f32,
    dead: bool,
    keys_last_time_pressed: HashMap<KeyCode, f32>
}

impl Teto {
    pub fn new(teto_type: TetoType) -> Self {
        let rots = Self::parse_teto(&teto_type);

        let mut teto = Self {
            t: teto_type,
            rots: rots,
            rot: 0,
            i: 0,
            j: 0,
            fall_duration: 0.5,
            prev_fall_time: 0.0,
            dead: false,
            keys_last_time_pressed: HashMap::new()
        };
        teto.reset();

        teto
    }
    
    pub fn get_teto_bag() -> Vec<TetoType> {
        Vec::from_iter([TetoType::I, TetoType::O, TetoType::T, TetoType::S, TetoType::Z, TetoType::J, TetoType::L])
    }

    fn parse_teto(teto_type: &TetoType) -> TetoRots {
        let filepath = format!("./res/{:?}", teto_type);
        let mut all = false;

        let mut i = 0;
        let mut rots: TetoRots = Vec::new();

        for line in fs::read_to_string(filepath).expect("Teto file failed :(").lines() {
            if line.starts_with("--") {
                if line == "--all" { all = true; }
                rots.push(Vec::new());
                i = 0;
                continue;
            }
            for c in line.bytes().enumerate() {
                if c.1 == b' ' { continue; }
                rots.last_mut().expect("Invalid Teto :(").push((i, c.0 as i32));
            }
            i += 1;
        }
        
        if all {
            rots.push(rots[0].clone());
            rots.push(rots[0].clone());
            rots.push(rots[0].clone());
        }
        rots
    }

    pub fn reset(&mut self) {
        let width = self.rots[0].iter().map(|pos| pos.1).max().expect("Empty Teto?! :O") - self.rots[0].iter().map(|pos| pos.1).min().expect("Empty Teto?! :O") + 1;
        self.i = -self.rots[0].iter().map(|pos| pos.0).max().expect("Empty Teto?! :O") - 1;
        self.j = (GRID_SIZE.j - width) / 2 - 1;
        self.rot = 0;
    }

    fn is_key_repeat(&mut self, ctx: &Context, key: KeyCode) -> bool {
        if !self.keys_last_time_pressed.contains_key(&key) { self.keys_last_time_pressed.insert(key, util::get_time(ctx) + 0.2); }
        if ctx.keyboard.is_key_just_pressed(key) {
            self.keys_last_time_pressed.insert(key, util::get_time(ctx) + 0.2);
            return true;
        }
        else if ctx.keyboard.is_key_pressed(key) && util::get_time(ctx) - self.keys_last_time_pressed[&key] > 0.1 {
            self.keys_last_time_pressed.insert(key, util::get_time(ctx));
            return true;
        }
        false
    }

    pub fn update(&mut self, ctx: &Context, board: &Board) {
        self.rotate(ctx, board);
        self.do_move(ctx, board);
        self.try_fall(ctx, board);
    }

    pub fn is_dead(&self) -> bool {
        self.dead
    }

    pub fn get_rot(&self) -> TetoRot {
        self.rots[self.rot].iter().map(|pos| (pos.0 + self.i, pos.1 + self.j)).collect()
    }

    fn rotate(&mut self, ctx: &Context, board: &Board) {
        self.rotate_dir(ctx, board, KeyCode::Left, -1);
        self.rotate_dir(ctx, board, KeyCode::Right, 1);
    }

    fn rotate_dir(&mut self, ctx: &Context, board: &Board, key: KeyCode, dir: i32) {
        if ctx.keyboard.is_key_just_pressed(key) {
            self.reset_fall(ctx);
            self.try_wall_kick(board, dir);
        }
    }

    fn try_wall_kick(&mut self, board: &Board, dir: i32) {
        let prev_rot = self.rot;
        self.rot = ((self.rot + self.rots.len()) as i32 + dir) as usize % self.rots.len();

        if self.try_rot(board, 0, 0) { return; }

        let wall_kicks = self.get_wall_kicks();
        for wall_kick in wall_kicks {
            if self.try_rot(board, wall_kick.0, wall_kick.1) { return; }
        }

        self.rot = prev_rot;
    }

    fn get_wall_kicks(&mut self) -> Vec<(i32, i32)> {
        match self.t {
            TetoType::I => [(-2, 0), (1, 0), (-2, 1), (1, -2)].into(),
            TetoType::O => [].into(),
            _ => [(-1, 0), (-1, -1), (0, 2), (-1, 2)].into()
        }
    }

    fn try_rot(&mut self, board: &Board, i_offset: i32, j_offset: i32) -> bool {
        for pos in self.rots[self.rot].iter() {
            if self.collides_cell(board, pos.0 + i_offset, pos.1 + j_offset) { return false; }
        }

        self.i += i_offset;
        self.j += j_offset;

        true
    }

    fn do_move(&mut self, ctx: &Context, board: &Board) {
        self.move_dir(ctx, board, KeyCode::A, -1);
        self.move_dir(ctx, board, KeyCode::D, 1);
    }

    fn move_dir(&mut self, ctx: &Context, board: &Board, key: KeyCode, dir: i32) {
        if self.is_key_repeat(ctx, key) {
            self.reset_fall(ctx);
            self.j += dir;
            if self.collides(board) { self.j -= dir; }
        }
    }

    fn try_fall(&mut self, ctx: &Context, board: &Board) {
        if ctx.keyboard.is_key_just_pressed(KeyCode::Space) {
            while !self.dead { self.fall(ctx, board); }
        } else if self.is_key_repeat(ctx, KeyCode::S) {
            self.reset_fall(ctx);
            self.fall(ctx, board);
        } else if util::get_time(ctx) - self.prev_fall_time > self.fall_duration { self.fall(ctx, board); }
    }

    fn fall(&mut self, ctx: &Context, board: &Board) {
        self.reset_fall(ctx);
        self.i += 1;
        if self.collides(board) { self.die(); }
    }

    pub fn draw_centered_at(&self, canvas: &mut Canvas, x: f32, y: f32) {
        let starting_i_offset = self.rots[self.rot].iter().map(|pos| pos.0).min().unwrap() as f32;
        let width = (self.rots[self.rot].iter().map(|pos| pos.1).max().unwrap() - self.rots[self.rot].iter().map(|pos| pos.1).min().unwrap() + 1) as f32;
        for pos in self.rots[self.rot].iter() {
            util::draw_cell(canvas, x + (pos.1 as f32 - width / 2.0) * CELL_SIZE, y + (pos.0 as f32 - starting_i_offset / 2.0) * CELL_SIZE, self.color());
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, board: &Board) {
        let ghost_i_offset = self.get_ghost_i_offset(board);
        let mut ghost_color = self.color();
        ghost_color.a = 0.1;
        for pos in self.rots[self.rot].iter() {
            util::draw_cell_indices(canvas, self.i + pos.0 + ghost_i_offset, self.j + pos.1, ghost_color);
            util::draw_cell_indices(canvas, self.i + pos.0, self.j + pos.1, self.color());
        }
    }

    pub fn color(&self) -> Color {
        match self.t {
            TetoType::I => Color::CYAN,
            TetoType::O => Color::YELLOW,
            TetoType::T => Color::MAGENTA,
            TetoType::S => Color::GREEN,
            TetoType::Z => Color::RED,
            TetoType::J => Color::BLUE,
            TetoType::L => Color::from_rgb(255, 140, 0) //orange
        }
    }

    fn get_ghost_i_offset(&self, board: &Board) -> i32 {
        let mut i_offset = 0;
        while !self.collides_ghost(board, i_offset) { i_offset += 1; }
        i_offset -= 1;
        i_offset
    }

    fn collides_ghost(&self, board: &Board, i_offset: i32) -> bool {
        for pos in self.rots[self.rot].iter() {
            if self.collides_cell(board, pos.0 + i_offset, pos.1) { return true; }
        }
        false
    }

    fn reset_fall(&mut self, ctx: &Context) {
        self.prev_fall_time = util::get_time(ctx);
    }

    fn collides(&self, board: &Board) -> bool {
        for pos in self.rots[self.rot].iter() {
            if self.collides_cell(board, pos.0, pos.1) { return true; }
        }
        false
    }

    fn collides_cell(&self, board: &Board, i_offset: i32, j_offset: i32) -> bool {
        board.filled(self.i + i_offset, self.j + j_offset)
    }

    fn die(&mut self) {
        self.dead = true;
        self.i -= 1;
    }
}