mod nes;
use nes::{bus::Bus, disassembler::disassemble, mos_6502::Mos6502};

use std::{cell::RefCell, rc::Rc, time::Duration};

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureCreator, TextureQuery},
    sys::KeyCode,
    ttf::{Font, Sdl2TtfContext},
    video::{Window, WindowContext},
    Sdl,
};

extern crate sdl2;

static SCREEN_WIDTH: u32 = 600;
static SCREEN_HEIGHT: u32 = 400;

macro_rules! rect {
    ($x:expr, $y:expr, $w:expr, $h:expr) => {
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    };
}

struct SDLEngine {
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    ttf_context: Sdl2TtfContext,
    sdl_context: Sdl,
}
impl SDLEngine {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init().map_err(|e| e.to_string())?;

        let video_subsystem = sdl_context.video().map_err(|e| e.to_string())?;

        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

        let window = video_subsystem
            .window("Rust NES Emulator", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        let texture_creator = canvas.texture_creator();

        Ok(Self {
            canvas,
            texture_creator,
            ttf_context,
            sdl_context,
        })
    }

    pub fn draw(&mut self, app: &mut App) -> Result<(), String> {
        let mut event_pump = self.sdl_context.event_pump().map_err(|e| e.to_string())?;
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyUp {
                        keycode: Some(Keycode::Space),
                        ..
                    } => app.key_up(Keycode::Space),
                    _ => {}
                }
            }

            self.canvas.set_draw_color(Color::BLACK);
            self.canvas.clear();

            app.draw(self)?;

            self.canvas.present();

            self.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60))
        }
        Ok(())
    }

    fn draw_text(&mut self, text: String, x: isize, y: isize) -> Result<(), String> {
        let mut font = self.ttf_context.load_font("SourceCodePro-Light.otf", 16)?;
        font.set_style(sdl2::ttf::FontStyle::BOLD);
        let surface = font
            .render(text.as_str())
            .blended_wrapped(Color::WHITE, SCREEN_WIDTH)
            .map_err(|e| e.to_string())?;
        let texture = self
            .texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = texture.query();

        let target = rect!(x, y, width, height);
        self.canvas.copy(&texture, None, Some(target))?;
        Ok(())
    }
}

struct App {
    cpu: Mos6502,
}

impl App {
    fn new() -> Self {
        let bus = Rc::new(RefCell::new(Bus::new()));
        let mut cpu = Mos6502::new(Rc::clone(&bus));
        cpu.pc = 0x8000;

        for (i, item) in [
            0xa9, 0x01, 0x8d, 0x00, 0x02, 0xa9, 0x05, 0x8d, 0x01, 0x02, 0xa9, 0x08, 0x8d, 0x02,
            0x02,
        ]
        .iter()
        .enumerate()
        {
            bus.borrow_mut().write(0x8000 + i as u16, *item);
        }

        Self { cpu }
    }

    fn key_up(&mut self, keycode: Keycode) {
        match keycode {
            Keycode::Space => {
                self.cpu.clock();
                while self.cpu.cycles != 0 {
                    self.cpu.clock();
                }
                println!("Step!")
            }
            _ => {}
        }
    }

    fn draw(&mut self, engine: &mut SDLEngine) -> Result<(), String> {
        let disassembled_program = disassemble(&self.cpu, self.cpu.pc, self.cpu.pc + 20).join("\n");

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
            self.cpu.pc,
            self.cpu.a,
            self.cpu.a,
            self.cpu.x,
            self.cpu.x,
            self.cpu.y,
            self.cpu.y,
            self.cpu.stack_ptr,
        );
        engine.draw_text(debug_text.trim().into(), 0, 0)?;

        let debug_text = format!("Program:\n-> {}", disassembled_program,);
        engine.draw_text(debug_text.trim().into(), SCREEN_WIDTH as isize / 2, 0)?;

        Ok(())
    }
}

fn main() -> Result<(), String> {
    let mut app = App::new();
    let mut engine = SDLEngine::new()?;
    engine.draw(&mut app)?;
    Ok(())
}
