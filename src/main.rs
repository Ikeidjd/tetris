use std::{env, fs};

use ggez::{event, glam::Vec2, graphics::{self, Canvas, Color, DrawParam, FontData, Rect}, input::keyboard::KeyCode, ContextBuilder, GameResult};

use crate::{board::Board, teto::{Teto, TetoType}, util::{CELL_SIZE, SCREEN_SIZE, UI_SIZE}};

mod board;
mod teto;
mod util;

struct PlayState {
    board: Board,

    teto: Teto,
    next_teto: Teto,
    held_teto: Option<Teto>,
    tetos: Vec<TetoType>,

    can_hold: bool,
    score: i32,
    high_score: i32,
    game_over: bool,

    score_ui_pos: Vec2,
    next_piece_ui_pos: Vec2,
    held_piece_ui_pos: Vec2
}

impl PlayState {
    fn new() -> Self {
        let mut tetos = Teto::get_teto_bag();
        Self {
            board: Board::new(),

            teto: Teto::new(tetos.swap_remove(rand::random_range(0..tetos.len()))),
            next_teto: Teto::new(tetos.swap_remove(rand::random_range(0..tetos.len()))),
            held_teto: None,
            tetos: tetos,

            can_hold: true,
            score: 0,
            high_score: fs::read_to_string("./res/high_score.txt").unwrap_or("0".to_owned()).parse().unwrap_or(0),
            game_over: false,

            score_ui_pos: Vec2::new(UI_SIZE.x / 2.0, UI_SIZE.y / 2.0),
            next_piece_ui_pos: Vec2::new(SCREEN_SIZE.x - UI_SIZE.x / 2.0, UI_SIZE.y / 2.0 - CELL_SIZE * 3.25),
            held_piece_ui_pos: Vec2::new(SCREEN_SIZE.x - UI_SIZE.x / 2.0, UI_SIZE.y / 2.0 + CELL_SIZE * 2.75)
        }
    }

    //Returns the old teto, not the new one
    fn pop_teto(&mut self) -> Teto {
        if self.tetos.len() == 0 { self.tetos = Teto::get_teto_bag(); }
        let new_teto = Teto::new(self.tetos.swap_remove(rand::random_range(0..self.tetos.len())));
        std::mem::replace(&mut self.teto, std::mem::replace(&mut self.next_teto, new_teto))
    }

    fn update_high_score(&mut self) {
        if self.score > self.high_score {
            self.high_score = self.score;
            fs::write("./res/high_score.txt", self.score.to_string()).expect(&format!("Something went wrong when saving high score ({}).", self.high_score));
        }
    }

    fn restart(&mut self) {
        self.update_high_score();

        self.board = Board::new();

        self.tetos = Teto::get_teto_bag();
        self.teto = Teto::new(self.tetos.swap_remove(rand::random_range(0..self.tetos.len())));
        self.next_teto = Teto::new(self.tetos.swap_remove(rand::random_range(0..self.tetos.len())));
        self.held_teto = None;

        self.can_hold = true;
        self.score = 0;
        self.game_over = false;
    }
}

impl event::EventHandler<ggez::GameError> for PlayState {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        if self.game_over {
            if ctx.keyboard.is_key_just_released(KeyCode::R) { self.restart(); }
            return Ok(());
        }

        if self.can_hold && ctx.keyboard.is_key_just_pressed(KeyCode::W) {
            match self.held_teto.as_mut() {
                None => self.held_teto = Some(self.pop_teto()),
                Some(teto) => {
                    std::mem::swap(teto, &mut self.teto);
                    self.teto.reset();
                    self.can_hold = false;
                }
            }
            self.held_teto.as_mut().unwrap().reset();
        }

        self.board.update(ctx);
        self.teto.update(ctx, &self.board);

        if self.teto.is_dead() {
            let teto = self.pop_teto();
            self.board.add(ctx, teto, &mut self.score, &mut self.game_over);
            self.can_hold = true;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let mut canvas = Canvas::from_frame(ctx, Color::new(0.25, 0.25, 0.25, 1.0));

        self.board.draw(&ctx, &mut canvas);
        self.teto.draw(&mut canvas, &self.board);

        let score = self.score.to_string();
        let combo = self.board.get_combo().to_string();
        let high = self.high_score.to_string();
        let len = score.len().max(combo.len()).max(high.len());

        util::draw_text_centered_on(&mut canvas, &format!("SCORE: {:>len$}", score), self.score_ui_pos - Vec2::Y * CELL_SIZE * 1.5, CELL_SIZE * 0.75, Color::WHITE);
        util::draw_text_centered_on(&mut canvas, &format!("COMBO: {:>len$}", combo), self.score_ui_pos, CELL_SIZE * 0.75, Color::WHITE);
        util::draw_text_centered_on(&mut canvas, &format!("HIGH:  {:>len$}", high), self.score_ui_pos + Vec2::Y * CELL_SIZE * 1.5, CELL_SIZE * 0.75, Color::WHITE);

        util::draw_text_centered_on(&mut canvas, "NEXT", self.next_piece_ui_pos - Vec2::Y * CELL_SIZE, CELL_SIZE * 0.75, Color::WHITE);
        self.next_teto.draw_centered_at(&mut canvas, self.next_piece_ui_pos.x, self.next_piece_ui_pos.y);

        util::draw_text_centered_on(&mut canvas, "HOLD", self.held_piece_ui_pos - Vec2::Y * CELL_SIZE, CELL_SIZE * 0.75, Color::WHITE);
        if let Some(held_teto) = &self.held_teto { held_teto.draw_centered_at(&mut canvas, self.held_piece_ui_pos.x, self.held_piece_ui_pos.y); }

        if self.game_over {
            canvas.draw(&graphics::Quad, DrawParam::default().dest_rect(Rect::new(0.0, 0.0, SCREEN_SIZE.x, SCREEN_SIZE.y)).color(Color::new(0.0, 0.0, 0.0, 0.9)));
            util::draw_text_centered_on_screen(&mut canvas, &format!("SCORE: {}", score), CELL_SIZE, Color::WHITE);
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn quit_event(&mut self, _ctx: &mut ggez::Context) -> Result<bool, ggez::GameError> {
        self.update_high_score();
        Ok(false)
    }
}

fn main() -> GameResult {
    unsafe { env::set_var("RUST_BACKTRACE", "1") };
    let (mut ctx, event_loop) = ContextBuilder::new("tetris", "ikeidjd")
                                .window_setup(ggez::conf::WindowSetup::default().title("Tetris"))
                                .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.x, SCREEN_SIZE.y))
                                .add_resource_path("./res")
                                .build()?;
    ctx.gfx.add_font("font", FontData::from_path(&ctx, "/PixelOperatorMono8-Bold.ttf")?);
    let state = PlayState::new();
    event::run(ctx, event_loop, state)
}