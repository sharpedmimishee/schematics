use macroquad::{math::vec2, prelude, text::measure_text, ui::root_ui, window::screen_width};

use crate::features::celltype::Celltype;

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

pub async fn main(chosen: &str, font: &prelude::Font, celltypes: &Vec<Celltype>) -> u32 { 
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
    
    //TODO
    let scr_h = prelude::screen_height();
    let playsettings = root_ui().button(vec2(width_adjuster(32.0, (rectangle_width/16.0) as u32, 0), scr_h*0.2), "   Play   ");
    let options = root_ui().button(vec2(width_adjuster(32.0, (rectangle_width/16.0) as u32, 0), scr_h*0.2+60.0), "   Options   ");
    let textures = root_ui().button(vec2(width_adjuster(32.0, (rectangle_width/16.0) as u32, 0), scr_h*0.2+120.0), "   Textures   ");
    let editor = root_ui().button(vec2(width_adjuster(32.0, (rectangle_width/16.0) as u32, 0), scr_h*0.2+180.0), "   Editor   ");
    let credits = root_ui().button(vec2(width_adjuster(32.0, (rectangle_width/16.0) as u32, 0), scr_h*0.2+240.0), "   Credits   ");
    let quit = root_ui().button(vec2(width_adjuster(32.0, (rectangle_width/16.0) as u32, 0), scr_h*0.2+300.0), "   Quit   ");
    let cells = root_ui().button(vec2(width_adjuster(32.0, (rectangle_width/16.0) as u32, 0), scr_h*0.9), "   Cells   ");
    root_ui().pop_skin();
    // if cells {}
    let mut hegith = 50;
    prelude::draw_text_ex(format!("{} cells were detected", celltypes.len()).as_str(), screen_width()-250., 20 as f32,
            prelude::TextParams {
            font: Some(font),
            font_size: 16,
            color: prelude::WHITE,
            ..Default::default()
        });
    for cell in celltypes {
        // let mut authors = String::new();
        // authors = format!("{}", &cell.author[0]);
        // for author in 1..cell.author.len() { authors = format!("{}, {}", authors ,&cell.author[author])}
        let celldesc = format!("{}", &cell.id);
        prelude::draw_text_ex(&celldesc.as_str(), screen_width()-measure_text(&celldesc, None, 16, 1.0).width-20.0, hegith as f32,
            prelude::TextParams {
            font: Some(font),
            font_size: 16,
            color: prelude::WHITE,
            ..Default::default()
        });
        hegith+=20;
    }
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
    // } else if cells {
        
    } else {
        return 0
    }
    // screen_height = prelude::screen_height();
    // screen_width = prelude::screen_width();
}