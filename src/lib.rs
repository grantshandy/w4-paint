#![no_std]
#![feature(default_alloc_error_handler)]

extern crate alloc;
extern crate core;

use core::{arch::wasm32, panic};

use alloc::vec::Vec;
use buddy_alloc::{BuddyAllocParam, FastAllocParam, NonThreadsafeAlloc};

// platform constants
pub const SCREEN_SIZE: u32 = 160;

// memory addresses
pub static mut PALETTE: *mut [u32; 4] = 0x04 as *mut [u32; 4];
pub const DRAW_COLORS: *mut u16 = 0x14 as *mut u16;
pub static mut FRAMEBUFFER: *mut [u8; 6400] = 0xa0 as *mut [u8; 6400];

pub const MOUSE_X: *const i16 = 0x1a as *const i16;
pub const MOUSE_Y: *const i16 = 0x1c as *const i16;
pub const MOUSE_BUTTONS: *const u8 = 0x1e as *const u8;

// allocator bs
const ALLOC_FAST_HEAP_SIZE: usize = 4 * 1024; // 4 KB
const ALLOC_HEAP_SIZE: usize = 16 * 1024; // 16 KB
const ALLOC_LEAF_SIZE: usize = 16;

static mut ALLOC_FAST_HEAP: [u8; ALLOC_FAST_HEAP_SIZE] = [0u8; ALLOC_FAST_HEAP_SIZE];
static mut ALLOC_HEAP: [u8; ALLOC_HEAP_SIZE] = [0u8; ALLOC_HEAP_SIZE];

#[global_allocator]
static ALLOC: NonThreadsafeAlloc = unsafe {
    NonThreadsafeAlloc::new(
        FastAllocParam::new(ALLOC_FAST_HEAP.as_ptr(), ALLOC_FAST_HEAP_SIZE),
        BuddyAllocParam::new(ALLOC_HEAP.as_ptr(), ALLOC_HEAP_SIZE, ALLOC_LEAF_SIZE),
    )
};

#[panic_handler]
fn panic_handler(_panic_info: &panic::PanicInfo<'_>) -> ! {
    wasm32::unreachable()
}

// application specifics
static mut POINTS: Vec<(u8, u8)> = Vec::new();
const HEADER_HEIGHT: u8 = 17;
const BLOCK_SIZE: u8 = 4;
const TITLE: &str = "Paint!";
const BUTTON: &str = "Clear";

#[no_mangle]
unsafe fn start() {
    *PALETTE = [0xd0d058, 0xa0a840, 0x708028, 0x405010];
}

#[no_mangle]
unsafe fn update() {
    // draw background
    *DRAW_COLORS = 0x41;
    rect(0, 0, SCREEN_SIZE, SCREEN_SIZE);

    // draw title
    *DRAW_COLORS = 0x42;
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

    if (*MOUSE_BUTTONS & 0b001) != 0 {
        let x: u8 = *MOUSE_X as u8 / BLOCK_SIZE;
        let y: u8 = *MOUSE_Y as u8 / BLOCK_SIZE;

        // draw points
        if (y > (HEADER_HEIGHT / BLOCK_SIZE) - 1) && !POINTS.contains(&(x, y)) {
            POINTS.push((x, y));
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

    // draw over borders
    *DRAW_COLORS = 0x40;
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
