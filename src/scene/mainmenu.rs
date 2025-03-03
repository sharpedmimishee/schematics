use macroquad::{math::vec2, prelude, ui::root_ui, window::screen_width};

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

pub async fn main(chosen: &str, font: &prelude::Font) -> u32 { 
    prelude::clear_background(prelude::Color::from_hex(0x241e47));
    let rectangle_width = width_adjuster(5.0, 1000, 300);
    let _rectangle = prelude::draw_rectangle(0.0, 0.0, rectangle_width, prelude::screen_height(), prelude::Color::from_rgba(20, 20, 20, 200));
    let _rzectangle_border = prelude::draw_rectangle_lines(0., 0., width_adjuster(5.0, 1000, 300), prelude::screen_height(), 5.0, prelude::GRAY);
    // let mut centre = prelude::get_text_center("Schematics", Option::None, 80, 1.0, 0.0 * 2.0);
    let _title = prelude::draw_text_ex("Schematics", 20.0, 50.0, 
    prelude::TextParams {
        font: Some(font),
        font_size: adjuster(5, 55, 40) as u16,
        color: prelude::Color::from_hex(0x604394),
        ..Default::default()
    });
    let _splash = prelude::draw_text_ex(
        format!("{}", chosen).as_str(),
        width_adjuster(5.0, 1000, 200),
        80.0,
        prelude::TextParams {
            font: Some(font),
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
    prelude::draw_text_ex(prelude::get_fps().to_string().as_str(), screen_width()-45.0, 20.0,
        prelude::TextParams {
        font: Some(font),
        font_size: 30,
        color: prelude::WHITE,
        ..Default::default()
    });
    // Pixel-
    // let font = load_ttf_font("examples/ui_assets/MinimalPixel v2.ttf")
    //     .await
    //     .unwrap();
    // if screen_height != prelude::screen_height() 
    // || screen_width != prelude::screen_width()
    // {
        // println!("size changed!");
        // let font = load_ttf_font("examples/ui_assets/HTOWERT.TTF")
        //     .await
        //     .unwrap();
        // button_style = root_ui()
        //     .style_builder()
        //     .background(
        //         prelude::Image::from_file_with_format(
        //             include_bytes!("assets/ui_assets/button.png"),
        //             Some(prelude::ImageFormat::Png),
        //         )
        //         .unwrap(),
        //     )
        //     .background_margin(prelude::RectOffset::new(16., 16., 16., 16.))
        //     .margin(prelude::RectOffset::new(8., 0., -4., -4.))
        //     .background_hovered(
        //         prelude::Image::from_file_with_format(
        //             include_bytes!("assets/ui_assets/button.png"),
        //             None,
        //         )
        //         .unwrap(),
        //     )
        //     .background_clicked(
        //         prelude::Image::from_file_with_format(
        //             include_bytes!("assets/ui_assets/button.png"),
        //             None,
        //         )
        //         .unwrap(),
        //     )
        //     // .with_font(&font)
        //     // .unwrap()
        //     .text_color(prelude::Color::from_rgba(230,230,230, 255))
        //     .font_size(adjuster(32, 50, 30) as u16)
        //     .build();

        // skin = ui::Skin {
        //     button_style,
        //     ..root_ui().default_skin()
        // };
    // };
    
    // let window1_skin = .clone();
    
    //TODO
    let scr_h = prelude::screen_height();
    let playsettings = root_ui().button(vec2(width_adjuster(32.0, (rectangle_width/16.0) as u32, 0), scr_h*0.2), "   Play   ");
    let options = root_ui().button(vec2(width_adjuster(32.0, (rectangle_width/16.0) as u32, 0), scr_h*0.2+60.0), "   Options   ");
    let textures = root_ui().button(vec2(width_adjuster(32.0, (rectangle_width/16.0) as u32, 0), scr_h*0.2+120.0), "   Textures   ");
    let editor = root_ui().button(vec2(width_adjuster(32.0, (rectangle_width/16.0) as u32, 0), scr_h*0.2+180.0), "   Editor   ");
    let credits = root_ui().button(vec2(width_adjuster(32.0, (rectangle_width/16.0) as u32, 0), scr_h*0.2+240.0), "   Credits   ");
    let quit = root_ui().button(vec2(width_adjuster(32.0, (rectangle_width/16.0) as u32, 0), scr_h*0.2+300.0), "   Quit   ");
    root_ui().pop_skin();

    if playsettings {
        return 1
    } else if options {
        return 2
    } else if textures {
        return 3
    } else if editor {
        return 4
    } else if credits {
        return 5
    } else if quit {
        return 6
    // } else if {
    } else {
        return 0
    }
    // screen_height = prelude::screen_height();
    // screen_width = prelude::screen_width();
}