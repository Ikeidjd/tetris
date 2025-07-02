use ggez::{glam::Vec2, graphics::{self, Canvas, Color, Drawable, Rect, Text, TextLayout}, Context};

pub struct Index {
    pub i: i32,
    pub j: i32
}

pub struct IVec2 {
    pub x: i32,
    pub y: i32
}

pub const GRID_SIZE: Index = Index { i: 20, j: 10 };
pub const CELL_SIZE: f32 = 30.0;
pub const GRID_PIXEL_SIZE: IVec2 = IVec2 { x: GRID_SIZE.j * CELL_SIZE as i32, y: GRID_SIZE.i * CELL_SIZE as i32 };
pub const UI_SIZE: Vec2 = Vec2 { x: GRID_PIXEL_SIZE.x as f32, y: GRID_PIXEL_SIZE.y as f32 };
pub const SCREEN_SIZE: Vec2 = Vec2 { x: UI_SIZE.x * 2.0 + GRID_PIXEL_SIZE.x as f32, y: GRID_PIXEL_SIZE.y as f32 };

pub fn draw(canvas: &mut Canvas, drawable: &impl Drawable, pos: Vec2, color: Color) {
    canvas.draw(drawable, graphics::DrawParam::new().dest(pos).color(color));
}

pub fn draw_rect(canvas: &mut Canvas, x: f32, y: f32, w: f32, h: f32, color: Color) {
    canvas.draw(&graphics::Quad, graphics::DrawParam::new().dest_rect(Rect::new(x, y, w, h)).color(color));
}

pub fn draw_cell(canvas: &mut Canvas, x: f32, y: f32, color: Color) {
    draw_rect(canvas, x, y, CELL_SIZE - 1.0, CELL_SIZE - 1.0, color);
}

pub fn draw_cell_indices(canvas: &mut Canvas, i: i32, j: i32, color: Color) {
    draw_cell(canvas, j as f32 * CELL_SIZE + UI_SIZE.x, i as f32 * CELL_SIZE, color);
}

pub fn draw_text_centered_on(canvas: &mut Canvas, text: &str, pos: Vec2, scale: f32, color: Color) {
    draw(canvas, Text::new(text).set_font("font").set_scale(scale).set_layout(TextLayout::center()), pos, color);
}

pub fn draw_text_centered_on_screen(canvas: &mut Canvas, text: &str, scale: f32, color: Color) {
    draw_text_centered_on(canvas, text, SCREEN_SIZE / 2.0, scale, color);
}

pub fn get_time(ctx: &Context) -> f32 {
    ctx.time.time_since_start().as_secs_f32()
}