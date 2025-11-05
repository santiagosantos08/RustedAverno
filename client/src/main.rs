mod gui;
mod game;
mod network;

use raylib::prelude::*;

fn draw_ui_layer(d: &mut RaylibDrawHandle) {
    d.draw_text("This is the UI Layer!", 12, 12, 20, Color::BLACK);
}

fn draw_game_layer(d: &mut RaylibDrawHandle) {
    d.draw_text("This is the Game Layer!", 12, 40, 20, Color::RED);
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Drawing Functions")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        draw_ui_layer(&mut d);
        draw_game_layer(&mut d);

    }
}