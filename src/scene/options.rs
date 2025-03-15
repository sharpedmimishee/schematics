use macroquad::{math::vec2, prelude, text::Font, ui::root_ui, window::next_frame};

pub async fn main(font: &Font) -> u32 {
    loop {
        prelude::clear_background(prelude::Color::from_hex(0x241e47));
        let exit = root_ui().button(vec2(20.0, 25.0), "   Exit   ");
        // draw_line(-0.4, 0.4, -0.8, 0.9, 0.05, BLUE);
        if exit {
            return 0;
        }
        next_frame().await
    }
}