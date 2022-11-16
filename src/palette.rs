use crate::wasm::{PALETTE, DRAW_COLORS};

pub type Palette = [u32; 4];

pub fn set_palette(palette: Palette) {
    unsafe {
        *PALETTE = palette;
    }
}

pub fn set_draw_colors(color: u16) {
    unsafe {
        *DRAW_COLORS = color;
    }
}