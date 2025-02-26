use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::prelude::{rand, vec2};
use macroquad::ui::{self, root_ui};
use macroquad::{prelude, text::TextParams, time::get_fps};
use macroquad::rand::ChooseRandom;
fn width_adjuster(ratio: u32, max: u32, min: u32) -> f32 {
    let default: f32 = prelude::screen_width()/ratio as f32;
    if (max as f32) < default {return max as f32};
    if (min as f32) > default {return min as f32};
    return default;
}
fn adjuster(ratio: u32, max: u32, min: u32) -> f32 {
    let default: f32 = (prelude::screen_width())/ratio as f32;
    // println!("{}", default);
    if (max as f32) < default {return max as f32};
    if (min as f32) > default {return min as f32};
    return default;
}

#[macroquad::main("Schematics")]
async fn main() {
    
    let mut skin1 = {
        // let font = load_ttf_font("examples/ui_assets/HTOWERT.TTF")
        //     .await
        //     .unwrap();
        let button_style = root_ui()
            .style_builder()
            .background(
                prelude::Image::from_file_with_format(
                    include_bytes!("assets/ui_assets/button.png"),
                    Some(prelude::ImageFormat::Png),
                )
                .unwrap(),
            )
            .background_margin(prelude::RectOffset::new(0., 0.,0., 0.))
            .margin(prelude::RectOffset::new(8.0, 8.0, 8.0, 8.0))
            .background_hovered(
                prelude::Image::from_file_with_format(
                    include_bytes!("assets/ui_assets/button.png"),
                    None,
                )
                .unwrap(),
            )
            .background_clicked(
                prelude::Image::from_file_with_format(
                    include_bytes!("assets/ui_assets/button.png"),
                    None,
                )
                .unwrap(),
            )
            // .with_font(&font)
            // .unwrap()
            .text_color(prelude::Color::from_rgba(230,230,230, 255))
            .font_size(adjuster(22, 80, 35) as u16)
            .build();

        ui::Skin {
            button_style,
            ..root_ui().default_skin()
        }
    };
    

    let splash_table: Vec<&str> = vec!["What... Is it Cell Machine!?", "low quality...", "please please please please", "I think you don't know who made this.", "Struct-Rethink"];
    rand::srand(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
    let chosen = splash_table.choose().unwrap();
    let mut screen_height: f32 = 0.;
    let mut screen_width: f32 = 0.;
    loop {
        prelude::clear_background(prelude::Color::from_hex(0x241e47));

        let _rectangle = prelude::draw_rectangle(0.0, 0.0, width_adjuster(5, 1000, 200), prelude::screen_height(), prelude::Color::from_rgba(20, 20, 20, 200));
        let _rzectangle_border = prelude::draw_rectangle_lines(0., 0., width_adjuster(5, 1000, 200), prelude::screen_height(), 5.0, prelude::GRAY);
        // let mut centre = prelude::get_text_center("Schematics", Option::None, 80, 1.0, 0.0 * 2.0);
        let _title = prelude::draw_text("Schematics", 20.0, 50.0, adjuster(25, 80, 40), prelude::Color::from_hex(0x604394));
        let _splash = prelude::draw_text_ex(
            format!("{}", chosen).as_str(),
            width_adjuster(5, 1000, 200),
            80.0,
            TextParams {
                font_size: adjuster(25, 40, 23) as u16,
                font_scale: prelude::get_time().sin() as f32 / 6.0 + 1.0,
                color: prelude::YELLOW,
                rotation: -0.1,
                ..Default::default()
            },
        );
        let _something = prelude::draw_text("something", prelude::screen_width()/2., prelude::screen_height()/2., 60.0, prelude::GOLD);
        // println!("{}, {}", centre.x, centre.y);
        // println!("{}, {}", prelude::screen_width(), prelude::screen_height());
        prelude::draw_text(get_fps().to_string().as_str(), 20.0, 20.0, 30.0, prelude::WHITE);
        //Pixel-
        // let font = load_ttf_font("examples/ui_assets/MinimalPixel v2.ttf")
        //     .await
        //     .unwrap();
        if screen_height == 0.0 || screen_height != prelude::screen_height() || screen_width == 0.0 || screen_width != prelude::screen_width() {
            // println!("size changed!");
            skin1 = {
            // let font = load_ttf_font("examples/ui_assets/HTOWERT.TTF")
            //     .await
            //     .unwrap();
                let button_style = root_ui()
                .style_builder()
                .background(
                    prelude::Image::from_file_with_format(
                        include_bytes!("assets/ui_assets/button.png"),
                        Some(prelude::ImageFormat::Png),
                    )
                    .unwrap(),
                )
                .background_margin(prelude::RectOffset::new(16., 16., 16., 16.))
                .margin(prelude::RectOffset::new(8., 0., -2., -2.))
                .background_hovered(
                    prelude::Image::from_file_with_format(
                        include_bytes!("assets/ui_assets/button.png"),
                        None,
                    )
                    .unwrap(),
                )
                .background_clicked(
                    prelude::Image::from_file_with_format(
                        include_bytes!("assets/ui_assets/button.png"),
                        None,
                    )
                    .unwrap(),
                )
                // .with_font(&font)
                // .unwrap()
                .text_color(prelude::Color::from_rgba(230,230,230, 255))
                .font_size(adjuster(22, 80, 35) as u16)
                .build();
    
            ui::Skin {
                button_style,
                ..root_ui().default_skin()
            }
        };
        }
        let window1_skin = skin1.clone();
        root_ui().push_skin(&window1_skin);
        root_ui().button(vec2(width_adjuster(18, 65, 45), 100.0), "Play!");
        root_ui().pop_skin();

        screen_height = prelude::screen_height();
        screen_width = prelude::screen_width();
        prelude::next_frame().await
    }
}