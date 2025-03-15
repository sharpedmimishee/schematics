use std::{fs, path::Path, process, time::{SystemTime, UNIX_EPOCH}};

use macroquad::{prelude, rand::ChooseRandom, ui::{self, hash, root_ui, Skin}};
mod scene;
mod features;
#[macroquad::main("Schematics")]
async fn main() {
    let cells: Vec<features::celltype::Celltype> = features::register::main();
    let splash_table: Vec<&str> = vec!["What... Is it Cell Machine!?", "low quality...", "please please please please", "I think you don't know who made this.", "Struct-Rethink"];
    prelude::rand::srand(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
    let chosen = ChooseRandom::choose(&splash_table[..]).unwrap();

    let font = prelude::load_ttf_font_from_bytes(include_bytes!("assets/font_assets/HOMESPUN.TTF")).unwrap();
    // let editbox_style = root_ui()
    //         .style_builder()
    //         .background(
    //             prelude::Image::from_file_with_format(
    //                 include_bytes!("assets/ui_assets/button.png"),
    //                 None,
    //             )
    //             .unwrap(),
    //         )
    //         .background_margin(prelude::RectOffset::new(2., 2., 2., 2.))
    //         .with_font(&font)
    //         .unwrap()
    //         .text_color(prelude::Color::from_rgba(120, 120, 120, 255))
    //         .font_size(25)
    //         .build();
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
        .margin(prelude::RectOffset::new(8., 0., -4., -4.))
        .background_hovered(
            prelude::Image::from_file_with_format(
                include_bytes!("assets/ui_assets/button.png"),
                Some(prelude::ImageFormat::Png),
            )
            .unwrap(),
        )
        .background_clicked(
            prelude::Image::from_file_with_format(
                include_bytes!("assets/ui_assets/button.png"),
                Some(prelude::ImageFormat::Png),
            )
            .unwrap(),
        )
        .with_font(&font)
        .unwrap()
        .text_color(prelude::Color::from_rgba(230,230,230, 255))
        .font_size(30)
        .build();

    let skin: Skin = ui::Skin {
        // editbox_style,
        button_style,
        ..root_ui().default_skin()
    };
    
    let mut next_scene: u32 = 0;
    let mut x_input: String = String::new();
    let mut y_input: String = String::new();
    loop {
        root_ui().push_skin(&skin);
        match next_scene {
            0 => next_scene = scene::mainmenu::main(&chosen, &font, &cells).await,
            1 => next_scene = scene::playsettings::main(&font, &mut x_input, &mut y_input).await,
            2 => next_scene = scene::options::main(&font).await,
            3 => next_scene = scene::textures::main(&font).await,
            5 => next_scene = scene::credits::main(&font).await,
            6 => process::exit(0),
            _ => ()
        }
        prelude::next_frame().await
    }
}