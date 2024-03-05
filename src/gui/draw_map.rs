use crate::mapgeneration::jsondata::LevelData;
use crate::settings::Settings;
use notan::draw::*;
use notan::prelude::*;

pub fn draw_map(gfx: &mut Graphics, level_data: &mut LevelData, settings: &Settings) -> RenderTexture {
    let width = (level_data.size.width as f32 * settings.general.render_scale) as u32;
    let height = (level_data.size.height as f32 * settings.general.render_scale) as u32;

    let rt: RenderTexture = gfx
        .create_render_texture(width, height)
        .with_filter(TextureFilter::Linear, TextureFilter::Linear)
        .build()
        .unwrap();
    let mut draw = rt.create_draw();
    // let width = level_data.size.width as f32;
    // let height = level_data.size.height as f32;
    // println!("{} {} {} {}", width, height, &level_data.level_image.map_edges.as_ref().unwrap().tiles.len(), &level_data.level_image.map_edges.as_ref().unwrap().tiles[0].len());
    // draw.line((0.0, 0.0), (0.0, height)).width(5.0).color(Color::GREEN);
    // draw.line((0.0, height), (width, height)).width(5.0).color(Color::RED);
    // draw.line((width, height), (width, 0.0)).width(5.0).color(Color::PURPLE);
    // draw.line((0.0, 0.0), (width, 0.0)).width(5.0).color(Color::ORANGE);
    // if let Some(map_grid) = &level_data.level_image.map_grid {
    //     for (y, row) in map_grid.tiles.iter().rev().enumerate() {
    //         for (x, cell) in row.iter().enumerate() {
    //             if cell != &false {
    //                 let dot_x = x as f32 * settings.general.render_scale;
    //                 let dot_y = (level_data.size.height as f32 - y as f32) * settings.general.render_scale;

    //                 draw.rect((dot_x, dot_y), (settings.general.render_scale, settings.general.render_scale))
    //                     .color(Color::from_hex(0xAAAAAAFF))
    //                     .alpha(0.2);
    //             }
    //         }
    //     }
    //     gfx.render_to(&rt, &draw);
    // }

    if let Some(edge_grid) = &level_data.level_image.map_edges {
        for (y, row) in edge_grid.tiles.iter().rev().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if cell != &false {
                    let dot_x = x as f32 * settings.general.render_scale;
                    let dot_y = (level_data.size.height as f32 - y as f32) * settings.general.render_scale;

                    draw.rect((dot_x, dot_y), (settings.general.render_scale, settings.general.render_scale))
                        .color(Color::from_hex(0xAAAAAAFF))
                        .alpha(settings.visual.map_opacity);
                }
            }
        }
        gfx.render_to(&rt, &draw);
    }
    rt
}
