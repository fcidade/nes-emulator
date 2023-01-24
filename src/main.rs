mod nes;
use nes::{Bus, Mos6502};

use std::{cell::RefCell, time::Duration};

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::TextureQuery};

extern crate sdl2;

static SCREEN_WIDTH: u32 = 600;
static SCREEN_HEIGHT: u32 = 400;

macro_rules! rect {
    ($x:expr, $y:expr, $w:expr, $h:expr) => {
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    };
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let window = video_subsystem
        .window("Rust NES Emulator", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    // Load font
    let mut font = ttf_context.load_font("SourceCodePro-Light.otf", 16)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    let bus = RefCell::new(Bus::new());
    let cpu = Mos6502::new(bus);

    let debug_text = format!(
        "
PC: ${:04X}
A: ${:02X} [{}]
X: ${:02X} [{}]
Y: ${:02X} [{}]
Stack Ptr: ${:04X} 
Status: N V - B D I Z C
        0 0   0 0 0 0 0
Space: Step Instruction
R: Reset
I: IRQ
N: NMI
",
        cpu.pc, cpu.a, cpu.a, cpu.x, cpu.x, cpu.y, cpu.y, cpu.stack_ptr,
    );
    let debug_text = debug_text.trim();

    let surface = font
        .render(debug_text)
        .blended_wrapped(Color::WHITE, SCREEN_WIDTH)
        .unwrap();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();

    let TextureQuery { width, height, .. } = texture.query();

    let target = rect!(0, 0, width, height);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        canvas.copy(&texture, None, Some(target))?;
        canvas.present();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60))
    }

    Ok(())
}
