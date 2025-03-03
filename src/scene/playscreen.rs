use std::path;
use macroquad::{prelude::*, ui::root_ui};

pub async fn main(x_range: u32, y_range: u32) -> u32 {
    let texture = Texture2D::from_file_with_format(include_bytes!("../assets/textures/void.png"), None);
    loop {
        let exit = root_ui().button(vec2(20.0, 25.0), "   Exit   ");
        // draw_texture_ex(
        //     &Texture2D::from_file_with_format(
        //         include_bytes!("../assets/textures/void.png"),
        //         None,
        //         ),
        //           (x+1) as f32,
        //           (y+1) as f32,
        //       WHITE,
        //       DrawTextureParams {
        //                   dest_size: Some(vec2(screen_width(), screen_height())),
        //                   ..Default::default()
        // });
        for x in 0..x_range-1 {
            for y in 0..y_range-1 {
                draw_texture_ex(
                    &texture,
                    (x*64) as f32,
                    (y*64) as f32,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(64.0, 64.0)),
                        ..Default::default()
                    },
                );
                println!("{}", x);
                println!("{}", y);
            }
        }

        // for x in 0..x_range {
        //     for y in 0..y_range {
                
        //         println!("drew!")
        //     }
        // }
        // println!("{}", &exit);
        if exit {
            // println!("will return 0");
            return 0
        }
        next_frame().await
    }
}