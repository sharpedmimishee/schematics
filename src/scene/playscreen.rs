use std::{cell, path};
use macroquad::{input::KeyCode, prelude::{self, *}, ui::{self, root_ui, Skin}};

use crate::features::{self, celltype::Celltype};
fn short_angle_dist(a0: f32, a1: f32) -> f32 {
    let max = 360.0;
    let da = (a1 - a0) % max;
    2.0 * da % max - da
}
fn angle_lerp(a0: f32, a1: f32, t: f32) -> f32 {
    a0 + short_angle_dist(a0, a1) * t
}

pub async fn main(x_range: u32, y_range: u32) -> u32 {
    let mut target = (0., 0.);
    let mut zoom = 0.0001;
    let mut rotation = 0.0;
    let mut smooth_rotation = 0.0;
    let void_texture = Texture2D::from_file_with_format(include_bytes!("../assets/textures/void.png"), None);
    let grid_texture = Texture2D::from_file_with_format(include_bytes!("../assets/textures/grid.png"), None);
    let test_texture = Texture2D::from_file_with_format(include_bytes!("../assets/textures/test.png"), None);
    let grid_center_x = x_range*64/2;
    let grid_center_y = y_range*64/2;
    let celltypes: Vec<features::celltype::Celltype> = features::register::main();
    let mut cell_pages: u32 = 1;
    let max_page = if celltypes.len() % 10 > 0 { 1 } else { 0 } + celltypes.len() / 10;

    let mut cell_1: &Celltype;
    let mut cell_2: &Celltype;
    let mut cell_3: &Celltype;
    let mut cell_4: &Celltype;
    let mut cell_5: &Celltype;
    let mut cell_6: &Celltype;
    let mut cell_7: &Celltype;
    let mut cell_8: &Celltype;
    let mut cell_9: &Celltype;
    let mut cell_10: &Celltype;

    let mut selected_cell: Celltype;
    loop {
        let font = prelude::load_ttf_font_from_bytes(include_bytes!("../assets/font_assets/HOMESPUN.TTF")).unwrap();
        prelude::draw_text_ex(prelude::get_fps().to_string().as_str(), screen_width()-45.0, 20.0,
        prelude::TextParams {
        font: Some(&font),
        font_size: 30,
        color: prelude::WHITE,
        ..Default::default()
        });
        if is_key_down(KeyCode::Z) {
            if (cell_pages - 10) <= 0 {
                cell_pages = max_page.try_into().unwrap();
            } else {
                cell_pages -= 10;
            }
        }
        if is_key_down(KeyCode::C) {
            if (cell_pages + 10) >= max_page.try_into().unwrap() {
                cell_pages = 1;
            } else {
                cell_pages += 10;
            }
        }
        for  i in cell_pages..cell_pages+9 {
            if i > celltypes.len().try_into().unwrap() { break; }
            
            let indexx: usize = (i-1).try_into().unwrap();
            // println!("{}, {}", i, &indexx);
            let celltype = &celltypes.get(indexx).unwrap().reference;
            let cell_path = &celltype.split(r"\");
            match i {
                val if val == cell_pages => {cell_1=&celltypes[indexx]},
                val if val == cell_pages+1 => {cell_2=&celltypes[indexx]},
                val if val == cell_pages+2 => {cell_3=&celltypes[indexx]},
                val if val == cell_pages+3 => {cell_4=&celltypes[indexx]},
                val if val == cell_pages+4 => {cell_5=&celltypes[indexx]},
                val if val == cell_pages+5 => {cell_6=&celltypes[indexx]},
                val if val == cell_pages+6 => {cell_7=&celltypes[indexx]},
                val if val == cell_pages+7 => {cell_8=&celltypes[indexx]},
                val if val == cell_pages+8 => {cell_9=&celltypes[indexx]},
                val if val == cell_pages+9 => {cell_10=&celltypes[indexx]},
                _ => ()
            }
        }
        

        let cellbutton_1 = root_ui().button(vec2(screen_width()*0.8-500.0, screen_height()*0.9), "");
        let cellbutton_2 = root_ui().button(vec2(screen_width()*0.8-450.0, screen_height()*0.9), "");
        let cellbutton_3 = root_ui().button(vec2(screen_width()*0.8-400.0, screen_height()*0.9), "");
        let cellbutton_4 = root_ui().button(vec2(screen_width()*0.8-350.0, screen_height()*0.9), "");
        let cellbutton_5 = root_ui().button(vec2(screen_width()*0.8-300.0, screen_height()*0.9), "");
        let cellbutton_6 = root_ui().button(vec2(screen_width()*0.8-250.0, screen_height()*0.9), "");
        let cellbutton_7 = root_ui().button(vec2(screen_width()*0.8-200.0, screen_height()*0.9), "");
        let cellbutton_8 = root_ui().button(vec2(screen_width()*0.8-150.0, screen_height()*0.9), "");
        let cellbutton_9 = root_ui().button(vec2(screen_width()*0.8-100.0, screen_height()*0.9), "");
        let cellbutton_10 = root_ui().button(vec2(screen_width()*0.8-50.0, screen_height()*0.9), "");

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
        if is_key_down(KeyCode::W) {
            if is_key_down(KeyCode::LeftShift) {target.1 -= 15.} else {target.1 -= 10.}
        }
        if is_key_down(KeyCode::S) {
            if is_key_down(KeyCode::LeftShift) {target.1 += 15.} else {target.1 += 10.}
        }
        if is_key_down(KeyCode::D) {
            if is_key_down(KeyCode::LeftShift) {target.0 += 15.} else {target.0 += 10.}
        }
        if is_key_down(KeyCode::A) {
            if is_key_down(KeyCode::LeftShift) {target.0 -= 15.} else {target.0 -= 10.}
        }
        match mouse_wheel() {
            (_x, y) => 
                zoom *= 1.009f32.powf(y)
            
        }
        smooth_rotation = angle_lerp(smooth_rotation, rotation, 0.1);
        let a = &Camera2D {
            target: vec2(target.0, target.1),
            zoom: vec2(zoom*300.0, zoom*300.0),
            // rotation: smooth_rotation,
            ..Default::default()
        };
        set_camera(a);
        for x in 0..x_range {
            for y in 0..y_range {
                draw_texture_ex(
                    &grid_texture,
                    (x*64) as f32,
                    (y*64) as f32,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(64.0, 64.0)),
                        ..Default::default()
                    },
                );
                // println!("{}", x);
                // println!("{}", y);
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
            set_default_camera();
            return 0
        }
        draw_text(format!("{}", a.zoom).as_str(), 30.0, 200.0, 30.0, prelude::WHITE);
        draw_text(format!("{}", target.0).as_str(), 30.0, 200.0, 30.0, prelude::WHITE);
        draw_text(format!("{}", target.1).as_str() , 30.0, 250.0, 30.0, prelude::WHITE);
        next_frame().await
    }
}