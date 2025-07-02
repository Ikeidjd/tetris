use ggez::{glam::Vec2, graphics::{Canvas, Color}, Context};

use crate::{teto::Teto, util::{self, CELL_SIZE, GRID_SIZE, SCREEN_SIZE}};

pub struct Board {
    data: [[Option<Color>; GRID_SIZE.j as usize]; GRID_SIZE.i as usize],
    is_cleared: [bool; GRID_SIZE.i as usize],
    clear_start_time: f32,
    clear_time: f32,
    clear_count: i32,
    clear_i: f32,
    combo_count: i32
}

impl Board {
    pub fn new() -> Self {
        Self {
            data: [[None; GRID_SIZE.j as usize]; GRID_SIZE.i as usize],
            is_cleared: [false; GRID_SIZE.i as usize],
            clear_start_time: 0.0,
            clear_time: 0.25,
            clear_count: 0,
            clear_i: -1.0,
            combo_count: 0
        }
    }

    pub fn add(&mut self, ctx: &Context, teto: Teto, score: &mut i32, game_over: &mut bool) {
        for pos in teto.get_rot() {
            if pos.0 < 0 || pos.0 >= GRID_SIZE.i || pos.1 < 0 || pos.1 >= GRID_SIZE.j { *game_over = true; }
            else { self.data[pos.0 as usize][pos.1 as usize] = Some(teto.color()); }
        }

        self.clear_count = 0;
        'outer: for row in self.data.iter().enumerate() {
            for cell in row.1.iter() {
                if matches!(cell, None) { continue 'outer; }
            }

            self.clear_count += 1;
            self.is_cleared[row.0] = true;
            self.clear_start_time = util::get_time(ctx);
            if self.clear_i == -1.0 { self.clear_i = row.0 as f32; }
        }

        if self.clear_count == 0 {
            self.combo_count = 0;
            return;
        }

        self.combo_count += 1;
        *score += self.get_score();
    }

    pub fn update(&mut self, ctx: &Context) {
        if util::get_time(ctx) - self.clear_start_time <= self.clear_time { return; }

        for i in 0..(GRID_SIZE.i as usize) {
            if !self.is_cleared[i] { continue; }

            self.lower(i);
            self.is_cleared[i] = false;
        }

        self.clear_i = -1.0;
    }

    fn lower(&mut self, until: usize) {
        for i in (0..=until).rev() {
            for j in 0..(GRID_SIZE.j as usize) {
                self.data[i][j] = if i == 0 { None } else { self.data[i - 1][j] }
            }
        }
    }

    pub fn draw(&self, ctx: &Context, canvas: &mut Canvas) {
        let clear_time_elapsed = util::get_time(ctx) - self.clear_start_time;
        let opacity = 1.0 - clear_time_elapsed / self.clear_time;

        for row in self.data.iter().enumerate() {
            if self.is_cleared[row.0] {
                for j in 0..GRID_SIZE.j { util::draw_cell_indices(canvas, row.0 as i32, j, Color::new(1.0, 1.0, 1.0, opacity)); }
                continue;
            }

            for cell in row.1.iter().enumerate() {
                let color = match cell.1 {
                    None => Color::BLACK,
                    Some(color) => *color
                };
                util::draw_cell_indices(canvas, row.0 as i32, cell.0 as i32, color);
            }
        }

        if clear_time_elapsed <= self.clear_time {
            let text = match self.clear_count {
                1 => format!("single! {} score", self.get_score()),
                2 => format!("double! {} score", self.get_score()),
                3 => format!("triple! {} score", self.get_score()),
                4 => format!("tetris! {} score", self.get_score()),
                _ => "".to_owned()
            };
            util::draw_text_centered_on(canvas, &text, Vec2::new(SCREEN_SIZE.x / 2.0, (self.clear_i - 0.5) * CELL_SIZE), CELL_SIZE * 0.6, Color::new(opacity, opacity, opacity, 1.0));
        }
    }

    fn get_score(&self) -> i32 {
        (self.combo_count - 1) * 50 + match self.clear_count {
            1 => 100,
            2 => 300,
            3 => 500,
            4 => 800,
            _ => 0
        }
    }

    fn empty(&self, i: i32, j: i32) -> bool {
        if i < 0 { return j >= 0 && j < GRID_SIZE.j; }
        i < GRID_SIZE.i && j >= 0 && j < GRID_SIZE.j && matches!(self.data[i as usize][j as usize], None)
    }

    pub fn filled(&self, i: i32, j: i32) -> bool {
        !self.empty(i, j)
    }

    pub fn get_combo(&self) -> i32 {
        self.combo_count
    }
}