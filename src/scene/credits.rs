use macroquad::{camera::Camera2D, math::vec2, prelude, text::{Font, TextParams}, ui::root_ui, window::next_frame};
fn adjuster(ratio: u32, max: u32, min: u32) -> f32 {
    let default: f32 = prelude::screen_width()/ratio as f32;
    if (max as f32) < default {return max as f32};
    if (min as f32) > default {return min as f32};
    return default;
}
pub async fn main(font: &Font) -> u32 {
        
    loop {
        
        // prelude::set_default_camera();
        prelude::clear_background(prelude::Color::from_hex(0x241e47));
        let exit = root_ui().button(vec2(20.0, 25.0), "   Exit   ");
        // root_ui().scroll().
        prelude::draw_text_ex("programs - mimisheesharp(The Sharp Cell, etc.)", 300.0, 50.0, 
        TextParams {
            font: Some(font),
             ..Default::default()});
        prelude::draw_text_ex("assets(includes concept, etc.) - The Flipper Cell, The Single Cell Generator, plasticgaming99, mimisheesharp(The Sharp Cell, etc.)", 300.0, 70.0, 
        TextParams {
            font: Some(font),
             ..Default::default()});
        prelude::draw_text_ex("code contributes - plasticgaming99", 300.0, 90.0, 
        TextParams {
            font: Some(font),
             ..Default::default()});
        prelude::draw_text_ex("musics - I'll do perhaps, but if you want to do, please do it.", 300.0, 110.0, 
        TextParams {
            font: Some(font),
             ..Default::default()});
        prelude::draw_text_ex("concepts - The Flipper Cell, The Single Cell Generator, plasticgaming99, mimisheesharp(The Sharp Cell, etc.)", 300.0, 130.0, 
        TextParams {
            font: Some(font),
             ..Default::default()});
        if exit {
            return 0;
        }
        next_frame().await
    }
}