use notan::draw::*;
use notan::math::Rect;
use notan::prelude::*;

pub fn draw_text(
    draw: &mut Draw,
    font: &Font,
    text: &String,
    pos_x: f32,
    pos_y: f32,
    font_size: f32,
    color: Color,
    drop_shadow: bool,
    black_bg: bool,
) {
    if black_bg {
        draw.text(font, &text)
            .position(pos_x, pos_y)
            .size(font_size)
            .color(Color::TRANSPARENT)
            .h_align_center()
            .v_align_bottom();

        let bounds: Rect = draw.last_text_bounds();
        let pad = font_size / 10.0;

        draw.rect((bounds.x - pad, bounds.y - pad), (bounds.width + pad + pad, bounds.height + pad + pad))
            .color(Color::from_hex(0x000000AA));
    }
    if drop_shadow {
        draw.text(font, &text)
            .position(pos_x + 1.5, pos_y + 1.5)
            .size(font_size)
            .color(Color::BLACK)
            .h_align_center()
            .v_align_bottom();
    }
    draw.text(font, &text)
        .position(pos_x, pos_y)
        .size(font_size)
        .color(color)
        .h_align_center()
        .v_align_bottom();
}

pub fn draw_text_left(
    draw: &mut Draw,
    font: &Font,
    text: &String,
    pos_x: f32,
    pos_y: f32,
    font_size: f32,
    color: Color,
    drop_shadow: bool,
    black_bg: bool,
) {
    if black_bg {
        draw.text(font, &text)
            .position(pos_x, pos_y)
            .size(font_size)
            .color(Color::TRANSPARENT)
            .h_align_left()
            .v_align_top();

        let bounds: Rect = draw.last_text_bounds();
        let pad = font_size / 10.0;

        draw.rect((bounds.x - pad, bounds.y - pad), (bounds.width + pad + pad, bounds.height + pad + pad))
            .color(Color::from_hex(0x00000088));
    }
    if drop_shadow {
        draw.text(font, &text)
            .position(pos_x + 2.0, pos_y + 2.0)
            .size(font_size)
            .color(Color::BLACK)
            .h_align_left()
            .v_align_top();
    }
    draw.text(font, &text)
        .position(pos_x, pos_y)
        .size(font_size)
        .color(color)
        .h_align_left()
        .v_align_top();
}

// who cares about enums?
pub fn get_attached_levels(level_id: &u32) -> Vec<u32> {
    match level_id {
        1 => vec![1, 2, 3, 4, 17],
        2 => vec![1, 2, 3, 4, 17],
        3 => vec![1, 2, 3, 4, 17],
        4 => vec![1, 2, 3, 4, 17],
        17 => vec![1, 2, 3, 4, 17],
        5 => vec![5, 6, 7, 26, 27, 28],
        6 => vec![5, 6, 7, 26, 27, 28],
        7 => vec![5, 6, 7, 26, 27, 28],
        26 => vec![5, 6, 7, 26, 27, 28],
        27 => vec![5, 6, 7, 26, 27, 28],
        28 => vec![5, 6, 7, 26, 27, 28],
        32 => vec![32, 33],
        33 => vec![32, 33],

        // act 2
        40 => vec![40, 41, 42, 43, 44, 45],
        41 => vec![40, 41, 42, 43, 44, 45],
        42 => vec![40, 41, 42, 43, 44, 45],
        43 => vec![40, 41, 42, 43, 44, 45],
        44 => vec![40, 41, 42, 43, 44, 45],
        45 => vec![40, 41, 42, 43, 44, 45],

        // act 3
        75 => vec![75, 76, 77, 78, 79, 80, 81, 82, 83],
        76 => vec![75, 76, 77, 78, 79, 80, 81, 82, 83],
        77 => vec![75, 76, 77, 78, 79, 80, 81, 82, 83],
        78 => vec![75, 76, 77, 78, 79, 80, 81, 82, 83],
        79 => vec![75, 76, 77, 78, 79, 80, 81, 82, 83],
        80 => vec![75, 76, 77, 78, 79, 80, 81, 82, 83],
        81 => vec![75, 76, 77, 78, 79, 80, 81, 82, 83],
        82 => vec![75, 76, 77, 78, 79, 80, 81, 82, 83],
        83 => vec![75, 76, 77, 78, 79, 80, 81, 82, 83],

        //act 4
        103 => vec![103, 104, 105, 106],
        104 => vec![103, 104, 105, 106],
        105 => vec![103, 104, 105, 106],
        106 => vec![103, 104, 105, 106],
        107 => vec![107, 108],
        108 => vec![107, 108],

        //act 5
        109 => vec![109, 110, 111, 112],
        110 => vec![109, 110, 111, 112],
        111 => vec![109, 110, 111, 112],
        112 => vec![109, 110, 111, 112],
        _ => vec![*level_id],
    }
}
