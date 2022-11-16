#![no_std]
#![feature(default_alloc_error_handler)]

extern crate alloc;
extern crate core;

use core::{arch::wasm32, panic};

use alloc::vec::Vec;
use buddy_alloc::{BuddyAllocParam, FastAllocParam, NonThreadsafeAlloc};
use palette::{set_draw_colors, set_palette, Palette};
use wasm::{rect, trace, MOUSE_BUTTONS, MOUSE_X, MOUSE_Y, SCREEN_SIZE};

mod palette;
mod wasm;

// app state
const DEFAULT_PALETTE: Palette = [0x211e20, 0x555568, 0xa0a08b, 0xe9efec];
static mut POINTS: Vec<(u8, u8)> = Vec::new();

// allocator
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
    trace("rust panic error");

    #[cfg(debug_assertions)]
    if let Some(cause) = _panic_info.payload().downcast_ref::<&str>() {
        trace(cause);
    }

    wasm32::unreachable()
}

#[no_mangle]
fn start() {
    set_palette(DEFAULT_PALETTE);
}

#[no_mangle]
fn update() {
    // draw background
    set_draw_colors(0x42);
    rect(0, 0, SCREEN_SIZE, SCREEN_SIZE);

    if left_pressed() {
        let x = mouse_x() as u8;
        let y = mouse_y() as u8;

        unsafe {
            if !POINTS.contains(&(x, y)) {
                POINTS.push((x, y));
            }
        }
    }

    set_draw_colors(0x44);
    for (x, y) in points() {
        rect((x - 1) as i32, (y - 1) as i32, 3, 3);
    }
}

fn points<'a>() -> &'a [(u8, u8)] {
    unsafe { POINTS.as_slice() }
}

fn left_pressed() -> bool {
    unsafe { (*MOUSE_BUTTONS & 0b001) != 0 }
}

fn mouse_x() -> i16 {
    unsafe { *MOUSE_X }
}

fn mouse_y() -> i16 {
    unsafe { *MOUSE_Y }
}
