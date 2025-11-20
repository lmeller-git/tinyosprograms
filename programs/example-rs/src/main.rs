#![no_std]
#![no_main]

extern crate alloc;

use core::time::Duration;

use libtinyos::{
    println,
    syscalls::{self, exit},
};
use tinygraphics::{
    backend::{GraphicsBackend, PrimitiveDrawer},
    mono_font::{self, MonoTextStyleBuilder},
    pixelcolor::Rgb888,
    prelude::{Point, Primitive, RgbColor, Size},
    primitives::{PrimitiveStyleBuilder, Rectangle},
    text::Text,
};

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let path = "/proc/kernel/io/serial";
    let serial = unsafe {
        syscalls::open(
            path.as_ptr(),
            path.bytes().len(),
            syscalls::OpenOptions::WRITE,
        )
    }
    .unwrap();
    println!("Now printing to default stdout");
    unsafe { syscalls::dup(serial, Some(syscalls::STDOUT_FILENO)) }.unwrap();
    println!("Now printing to serial");

    let mut graphics_backend = PrimitiveDrawer::default();
    let glyph = tinygraphics::primitives::Circle::new(Point::new(500, 100), 42);
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb888::RED)
        .stroke_width(4)
        .build();
    graphics_backend
        .draw_primitive(&glyph.into_styled(style))
        .unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&mono_font::ascii::FONT_10X20)
        .text_color(Rgb888::WHITE)
        .background_color(Rgb888::MAGENTA)
        .underline()
        .build();
    let text = Text::new(
        "Hello World from example-rs graphics!",
        Point::new(500, 200),
        text_style,
    );
    graphics_backend.draw_primitive(&text).unwrap();

    // This currently does nothing, as (default) fb == kernel fb
    graphics_backend
        .flush(Rectangle::new(Point::new(500, 100), Size::new(50, 200)))
        .unwrap();

    println!("Hello World from example-rs");
    unsafe { exit(0) }
}
