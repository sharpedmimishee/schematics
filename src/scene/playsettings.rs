use std::vec;

use macroquad::{math::vec2, prelude, text::Font, ui::{self, hash, root_ui, widgets}, window::{screen_height, screen_width}};

use super::{mainmenu, playscreen};

fn width_adjuster(ratio: f32, max: u32, min: u32) -> f32 {
    let default: f32 = prelude::screen_width()/ratio as f32;
    if (max as f32) < default {return max as f32};
    if (min as f32) > default {return min as f32};
    return default;
}
fn adjuster(ratio: u32, max: u32, min: u32) -> f32 {
    let default: f32 = prelude::screen_width()/ratio as f32;
    if (max as f32) < default {return max as f32};
    if (min as f32) > default {return min as f32};
    return default;
}

pub async fn main(font: &Font, x_input: &mut String, y_input: &mut String) -> u32 {
    prelude::clear_background(prelude::Color::from_hex(0x241e47));
    let _title = prelude::draw_text_ex("PlaySettings", 20.0, 50.0, 
    prelude::TextParams {
        font: Some(font),
        font_size: adjuster(5, 55, 40) as u16,
        color: prelude::Color::from_hex(0x604394),
        ..Default::default()
    });
    // if warn
    // let _warn = prelude::draw_text_ex("Must input the x and y.", screen_width()/2.0*0.9, 50.0, 
    //     prelude::TextParams {
    //             font: Some(font),
    //             font_size: adjuster(5, 55, 40) as u16,
    //             color: prelude::Color::from_hex(0x604394),
    //             ..Default::default()
    //         });
    //let play = root_ui().button(vec2(screen_width()/2.0, screen_height()*0.6), "   Create   ");
    let play = widgets::Button::new("   Create   ")
        .position(vec2(screen_width()/2.-(200./2.), screen_height()*0.58))
        .size(vec2(200., 70.))
        .ui(&mut *root_ui());

    //root_ui().input_text(hash!(), "x", x_input);
    //root_ui().input_text(hash!(), "y", y_input);
    let size = vec2(150., 50.);
    widgets::InputText::new(hash!())
        .position(vec2((screen_width()/2.)-size.x-8., screen_height()*0.45))
        .size(size)
        .ui(&mut *root_ui(), x_input);
    widgets::InputText::new(hash!())
        .position(vec2((screen_width()/2.)/*-(size.x/2.)+size.x+5.*/+8.,screen_height()*0.45))
        .size(size)
        .ui(&mut *root_ui(), y_input);
    
    if play {
        if x_input.is_empty() || y_input.is_empty() {
            return 1;
        } else {
            return playscreen::main(x_input.parse().unwrap(), y_input.parse().unwrap()).await;
        }
    } else {
        return 1
    }
}