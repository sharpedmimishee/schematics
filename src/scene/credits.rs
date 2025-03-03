use macroquad::{math::vec2, prelude, text::Font, ui::root_ui};
fn adjuster(ratio: u32, max: u32, min: u32) -> f32 {
    let default: f32 = prelude::screen_width()/ratio as f32;
    if (max as f32) < default {return max as f32};
    if (min as f32) > default {return min as f32};
    return default;
}
pub async fn main(font: &Font) -> u32 {
    let credits_msg: String = String::from(
        "programs - mimisheesharp(The Sharp Cell, etc.)
assets(includes concept, etc.) - The Flipper Cell, The Single Cell Generator, plasticgaming99, mimisheesharp(The Sharp Cell, etc.)
code contributes - plasticgaming99,
musics - I'll do perhaps, but if you want to do, please do it.
concepts - The Flipper Cell, The Single Cell Generator, plasticgaming99, mimisheesharp(The Sharp Cell, etc.)"
            );
    loop {
        let exit = root_ui().button(vec2(20.0, 25.0), "   Exit   ");
        let _title = prelude::draw_text_ex("Credits", 20.0, 50.0, 
        prelude::TextParams {
            font: Some(font),
            font_size: adjuster(5, 55, 40) as u16,
            color: prelude::Color::from_hex(0x604394),
            ..Default::default()
        });

        // root_ui().scroll().
        let credits = prelude::draw_text_ex(&credits_msg, 20.0, 50.0, 
        prelude::TextParams {
            font: Some(font),
            font_size: adjuster(5, 55, 40) as u16,
            color: prelude::Color::from_hex(0x604394),
            ..Default::default()
        });
        if exit {
            return 0;
        }
    }
}