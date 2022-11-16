// platform constants
pub const SCREEN_SIZE: u32 = 160;

// memory addresses
pub static mut PALETTE: *mut [u32; 4] = 0x04 as *mut [u32; 4];
pub const DRAW_COLORS: *mut u16 = 0x14 as *mut u16;
pub static mut FRAMEBUFFER: *mut [u8; 6400] = 0xa0 as *mut [u8; 6400];

pub const MOUSE_X: *const i16 = 0x1a as *const i16;
pub const MOUSE_Y: *const i16 = 0x1c as *const i16;
pub const MOUSE_BUTTONS: *const u8 = 0x1e as *const u8;

pub fn trace<T: AsRef<str>>(text: T) {
    let text_ref = text.as_ref();

    unsafe { extern_trace(text_ref.as_ptr(), text_ref.len()) }
}

pub fn rect(x: i32, y: i32, width: u32, height: u32) {
    unsafe {
        extern_rect(x, y, width, height);
    }
}

pub fn text<T: AsRef<str>>(text: T, x: i32, y: i32) {
    let text_ref = text.as_ref();

    unsafe {
        extern_text(text_ref.as_ptr(), text_ref.len(), x, y);
    }
}

extern "C" {
    #[link_name = "traceUtf8"]
    fn extern_trace(trace: *const u8, length: usize);
    #[link_name = "rect"]
    fn extern_rect(x: i32, y: i32, width: u32, height: u32);
    #[link_name = "textUtf8"]
    fn extern_text(text: *const u8, length: usize, x: i32, y: i32);
}
