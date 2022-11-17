#![no_std]
extern crate core;

use core::{arch::wasm32, panic::PanicInfo};

use heapless::Vec;

// platform constants
pub const SCREEN_SIZE: u32 = 160;

// memory addresses
pub static mut PALETTE: *mut [u32; 4] = 0x04 as *mut [u32; 4];
pub const DRAW_COLORS: *mut u16 = 0x14 as *mut u16;
pub static mut FRAMEBUFFER: *mut [u8; 6400] = 0xa0 as *mut [u8; 6400];

pub const MOUSE_X: *const i16 = 0x1a as *const i16;
pub const MOUSE_Y: *const i16 = 0x1c as *const i16;
pub const MOUSE_BUTTONS: *const u8 = 0x1e as *const u8;

// application specifics
static mut POINTS: Vec<(u8, u8), 1024> = Vec::new();
const HEADER_HEIGHT: u8 = 16;
const BLOCK_SIZE: u8 = 4;
const TITLE: &str = "Paint";
const BUTTON: &str = "Clear";

#[no_mangle]
unsafe fn start() {
    *PALETTE = [0xd0d058, 0xa0a840, 0x708028, 0x405010];
}

#[no_mangle]
unsafe fn update() {
    // draw background
    *DRAW_COLORS = 0x01;
    rect(0, 0, SCREEN_SIZE, SCREEN_SIZE);

    // draw title
    *DRAW_COLORS = 0x02;
    rect(0, 0, SCREEN_SIZE, HEADER_HEIGHT as u32);
    *DRAW_COLORS = 0x21;
    text(TITLE.as_ptr(), TITLE.len(), 4, 5);

    // draw clear button
    if (*MOUSE_BUTTONS & 0b001) != 0
        && (*MOUSE_X > 107 && *MOUSE_X < 158)
        && (*MOUSE_Y > 1 && *MOUSE_Y < 15)
    {
        *DRAW_COLORS = 0x44;

        POINTS.clear();
    } else {
        *DRAW_COLORS = 0x33;
    }
    rect(108, 2, 50, (HEADER_HEIGHT - 4) as u32);

    *DRAW_COLORS = 0x01;
    text(BUTTON.as_ptr(), BUTTON.len(), 114, 5);

    let x: u8 = *MOUSE_X as u8 / BLOCK_SIZE;
    let y: u8 = *MOUSE_Y as u8 / BLOCK_SIZE;

    // add content from mouse
    if (*MOUSE_BUTTONS & 0b001) != 0 {
        if (y > (HEADER_HEIGHT / BLOCK_SIZE) - 1) && !POINTS.contains(&(x, y)) {
            POINTS.push((x, y)).unwrap();
        }
    }

    // remove content from mouse
    if (*MOUSE_BUTTONS & 0b010) != 0 {
        if (y > (HEADER_HEIGHT / BLOCK_SIZE) - 1) && POINTS.contains(&(x, y)) {
            // im sorry everyone who is reading this code
            // this is a really bad move
            POINTS.sort_unstable();
                        
            POINTS.remove(POINTS.binary_search(&(x, y)).unwrap());
        }
    }

    // draw content
    *DRAW_COLORS = 0x33;
    for (x, y) in POINTS.as_slice() {
        rect(
            (x * BLOCK_SIZE) as i32,
            (y * BLOCK_SIZE) as i32,
            BLOCK_SIZE as u32,
            BLOCK_SIZE as u32,
        );
    }

    // draw over borders again because I'm lazy
    *DRAW_COLORS = 0x00;
    rect(0, 0, SCREEN_SIZE, HEADER_HEIGHT as u32);
    rect(
        0,
        HEADER_HEIGHT as i32,
        SCREEN_SIZE,
        SCREEN_SIZE - HEADER_HEIGHT as u32,
    );
}

extern "C" {
    fn rect(x: i32, y: i32, width: u32, height: u32);
    fn hline(x: i32, y: i32, length: usize);
    #[link_name = "textUtf8"]
    fn text(text: *const u8, length: usize, x: i32, y: i32);
}

#[panic_handler]
fn phandler(_panic_info: &PanicInfo<'_>) -> ! {
    wasm32::unreachable()
}
