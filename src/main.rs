use raylib::prelude::*;
use std::thread;
use std::time::Duration;
 
struct Framebuffer {
    width: i32,
    height: i32,
    image: Image,
    current_color: Color,
    background_color: Color,
}
 
impl Framebuffer {
    fn new(width: u32, height: u32) -> Self {
        let background_color = Color::BLACK;
        let image = Image::gen_image_color(width as i32, height as i32, background_color);
        Framebuffer {
            width: width as i32,
            height: height as i32,
            image,
            current_color: Color::WHITE,
            background_color,
        }
    }
 
    fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }
 
    fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }
 
    fn clear(&mut self) {
        self.image = Image::gen_image_color(self.width, self.height, self.background_color);
    }
 
    fn point(&mut self, x: i32, y: i32) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.image.draw_pixel(x, y, self.current_color);
        }
    }
 
    fn get_color(&mut self, x: i32, y: i32) -> Color {
        self.image.get_color(x, y)
    }
 
    fn export_image(&self, filename: &str) {
        self.image.export_image(filename);
    }
 
    fn swap_buffers(&mut self, window: &mut RaylibHandle, thread: &RaylibThread) {
        let screen_w = window.get_screen_width() as f32;
        let screen_h = window.get_screen_height() as f32;
 
        let texture = window
            .load_texture_from_image(thread, &self.image)
            .expect("No se pudo crear la textura del framebuffer");
 
        let mut d = window.begin_drawing(thread);
        d.clear_background(Color::BLACK);
 
        let source = Rectangle::new(0.0, 0.0, self.width as f32, self.height as f32);
        let dest = Rectangle::new(0.0, 0.0, screen_w, screen_h);
        d.draw_texture_pro(&texture, source, dest, Vector2::new(0.0, 0.0), 0.0, Color::WHITE);
    }
}
 
fn is_alive(color: Color) -> bool {
    color.r > 128
}
 
fn count_alive_neighbors(fb: &mut Framebuffer, x: i32, y: i32, wrap: bool) -> u8 {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
 
            let (nx, ny) = if wrap {
                (
                    (x + dx).rem_euclid(fb.width),
                    (y + dy).rem_euclid(fb.height),
                )
            } else {
                (x + dx, y + dy)
            };
 
            if nx < 0 || nx >= fb.width || ny < 0 || ny >= fb.height {
                continue;
            }
 
            if is_alive(fb.get_color(nx, ny)) {
                count += 1;
            }
        }
    }
    count
}
 
fn step_game_of_life(fb: &mut Framebuffer, wrap: bool) {
    let width = fb.width;
    let height = fb.height;
    let mut next_alive = vec![false; (width * height) as usize];
 
    for y in 0..height {
        for x in 0..width {
            let alive = is_alive(fb.get_color(x, y));
            let neighbors = count_alive_neighbors(fb, x, y, wrap);
            let will_live = matches!((alive, neighbors), (true, 2) | (true, 3) | (false, 3));
            next_alive[(y * width + x) as usize] = will_live;
        }
    }
 
    for y in 0..height {
        for x in 0..width {
            let alive = next_alive[(y * width + x) as usize];
            fb.set_current_color(if alive { Color::WHITE } else { Color::BLACK });
            fb.point(x, y);
        }
    }
}
 
fn block(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_current_color(Color::WHITE);
    for (dx, dy) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
        fb.point(x + dx, y + dy);
    }
}
 
fn beehive(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_current_color(Color::WHITE);
    for (dx, dy) in [(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2)] {
        fb.point(x + dx, y + dy);
    }
}
 
fn loaf(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_current_color(Color::WHITE);
    for (dx, dy) in [(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (3, 2), (2, 3)] {
        fb.point(x + dx, y + dy);
    }
}
 
fn boat(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_current_color(Color::WHITE);
    for (dx, dy) in [(0, 0), (1, 0), (0, 1), (2, 1), (1, 2)] {
        fb.point(x + dx, y + dy);
    }
}
 
fn tub(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_current_color(Color::WHITE);
    for (dx, dy) in [(1, 0), (0, 1), (2, 1), (1, 2)] {
        fb.point(x + dx, y + dy);
    }
}
 
fn blinker(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_current_color(Color::WHITE);
    for (dx, dy) in [(0, 0), (0, 1), (0, 2)] {
        fb.point(x + dx, y + dy);
    }
}
 
fn toad(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_current_color(Color::WHITE);
    for (dx, dy) in [(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)] {
        fb.point(x + dx, y + dy);
    }
}
 
fn beacon(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_current_color(Color::WHITE);
    for (dx, dy) in [
        (0, 0), (1, 0), (0, 1), (1, 1),
        (2, 2), (3, 2), (2, 3), (3, 3),
    ] {
        fb.point(x + dx, y + dy);
    }
}
 
fn pulsar(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_current_color(Color::WHITE);
    let arm = [
        (2, 0), (3, 0), (4, 0),
        (0, 2), (5, 2), (0, 3), (5, 3), (0, 4), (5, 4),
        (2, 5), (3, 5), (4, 5),
    ];
    for (dx, dy) in arm.iter() {
        fb.point(x + dx, y + dy);
        fb.point(x - dx, y + dy);
        fb.point(x + dx, y - dy);
        fb.point(x - dx, y - dy);
    }
}
 
fn glider(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_current_color(Color::WHITE);
    for (dx, dy) in [(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)] {
        fb.point(x + dx, y + dy);
    }
}
 
fn lwss(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_current_color(Color::WHITE);
    for (dx, dy) in [
        (1, 0), (4, 0),
        (0, 1),
        (0, 2), (4, 2),
        (0, 3), (1, 3), (2, 3), (3, 3),
    ] {
        fb.point(x + dx, y + dy);
    }
}
 
fn gosper_glider_gun(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_current_color(Color::WHITE);
    let cells = [
        (0, 4), (0, 5), (1, 4), (1, 5),
        (10, 4), (10, 5), (10, 6),
        (11, 3), (11, 7),
        (12, 2), (12, 8),
        (13, 2), (13, 8),
        (14, 5),
        (15, 3), (15, 7),
        (16, 4), (16, 5), (16, 6),
        (17, 5),
        (20, 2), (20, 3), (20, 4),
        (21, 2), (21, 3), (21, 4),
        (22, 1), (22, 5),
        (24, 0), (24, 1), (24, 5), (24, 6),
        (34, 2), (34, 3), (35, 2), (35, 3),
    ];
    for (dx, dy) in cells.iter() {
        fb.point(x + dx, y + dy);
    }
}
 
fn setup_initial_pattern(fb: &mut Framebuffer) {
    fb.set_background_color(Color::BLACK);
    fb.clear();
 
    block(fb, 5, 5);
    beehive(fb, 15, 5);
    loaf(fb, 26, 5);
    boat(fb, 37, 5);
    tub(fb, 45, 5);
 
    blinker(fb, 5, 20);
    toad(fb, 15, 20);
    beacon(fb, 26, 20);
    pulsar(fb, 45, 25);
 
    glider(fb, 5, 45);
    glider(fb, 60, 60);
    lwss(fb, 20, 50);
 
    gosper_glider_gun(fb, 90, 10);
}
 
fn main() {
    let grid_width = 150;
    let grid_height = 100;
    let scale = 7;
 
    let (mut window, raylib_thread) = raylib::init()
        .size(grid_width * scale, grid_height * scale)
        .resizable()
        .title("Lab 2 - Game of Life")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();
 
    let mut framebuffer = Framebuffer::new(grid_width as u32, grid_height as u32);
    setup_initial_pattern(&mut framebuffer);
 
    let wrap_edges = true;
    let mut screenshot_count = 0;
 
    while !window.window_should_close() {
        if window.is_key_pressed(KeyboardKey::KEY_P) {
            screenshot_count += 1;
            let filename = format!("screenshot_{}.png", screenshot_count);
            framebuffer.export_image(&filename);
            println!("Screenshot guardado como '{}'", filename);
        }
 
        if window.is_key_pressed(KeyboardKey::KEY_R) {
            setup_initial_pattern(&mut framebuffer);
        }
 
        step_game_of_life(&mut framebuffer, wrap_edges);
 
        framebuffer.swap_buffers(&mut window, &raylib_thread);
 
        thread::sleep(Duration::from_millis(80));
    }
}