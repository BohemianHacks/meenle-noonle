#![no_std]
#![feature(panic_info_message)]

const MNFB_WIDTH: u32 = 500;
type ColorYUYV = u32; // 0xYYUUYYVV; YUYV color format encodes two pixels in four bytes

#[global_allocator]
static RUSTALLOC: RustAlloc = RustAlloc {};
extern crate alloc;
use alloc::{ffi::CString, format};
use core::{
    alloc::{GlobalAlloc, Layout},
    ffi::c_char,
    ptr::null_mut,
};

#[allow(dead_code)]
extern "C" {
    fn OSAllocFromHeap(heap: i32, size: u32) -> *mut u8;
    fn OSFreeToHeap(heap: i32, ptr: *mut u8);
    fn OSPanic(file: *const c_char, line: i32, msg: *const c_char, ...) -> !;
    fn RGBAtoYUYV(color1: ColorRGBA, color2: ColorRGBA) -> ColorYUYV;
    fn OSReport(msg: *const c_char, ...);
    fn sin(n: f64) -> f64;
    fn cos(n: f64) -> f64;
}

#[repr(C)]
#[rustfmt::skip]
#[derive(Clone, Copy)]
pub struct ColorRGBA { r: u8, g: u8, b: u8, a: u8, }

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn drawMNFBIntoXFB(
    mnfb: *const [[ColorRGBA; MNFB_WIDTH as usize]; MNFB_WIDTH as usize],
    xfb: *mut [[ColorYUYV; 320]; 480], // 640 x 480
    fb_width: u32,
) {
    let bg_color = ColorRGBA {
        r: 11,
        g: 11,
        b: 68,
        a: 255,
    };
    let bg_color: ColorYUYV = RGBAtoYUYV(bg_color, bg_color);
    let vbar_width: ColorYUYV = (fb_width - MNFB_WIDTH) / 4; // = 35; Pillarboxing
    for (idx_row, row) in (*xfb).iter_mut().enumerate() {
        for (idx_col, yuyv) in row.iter_mut().enumerate() {
            if idx_col as u32 <= vbar_width || idx_col as u32 >= vbar_width + MNFB_WIDTH / 2 {
                *yuyv = bg_color;
            } else {
                let mn_col = (idx_col - vbar_width as usize) * 2;
                let color1: ColorRGBA = (*mnfb)[idx_row][mn_col];
                let color2: ColorRGBA = (*mnfb)[idx_row + 1][mn_col];
                *yuyv = RGBAtoYUYV(color1, color2);
            }
        }
    }
}

pub trait MSLmaths {
    fn sin(self) -> Self;
    fn cos(self) -> Self;
}
impl MSLmaths for f64 {
    fn sin(self) -> Self {
        unsafe { sin(self) }
    }
    fn cos(self) -> Self {
        unsafe { cos(self) }
    }
}

struct RustAlloc {}
#[rustfmt::skip]
impl RustAlloc { const HEAP_HANDLE: i32 = 0; }
unsafe impl GlobalAlloc for RustAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = u32::try_from(layout.size()).expect("allocation size invalid!");
        let ptr = OSAllocFromHeap(RustAlloc::HEAP_HANDLE, size);
        assert_ne!(ptr, null_mut());
        ptr
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _: Layout) {
        OSFreeToHeap(RustAlloc::HEAP_HANDLE, ptr);
    }
}

#[panic_handler]
unsafe fn panic(info: &core::panic::PanicInfo) -> ! {
    let loc = info.location().unwrap();
    let filename = CString::new(loc.file()).unwrap();
    let msg = CString::new(
        format!(
            "[{file}:{line}]: {msg}",
            file = loc.file(),
            line = loc.line(),
            msg = info.message().unwrap_unchecked()
        )
        .as_str(),
    )
    .unwrap_unchecked();
    OSPanic(filename.as_ptr(), loc.line() as i32, msg.as_ptr());
    #[allow(unreachable_code)]
    loop {} // just in case
}
