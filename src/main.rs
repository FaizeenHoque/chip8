mod cpu;

use rodio::{
    OutputStreamBuilder,
    Sink,
    Source,
    source::SineWave,
};
use std::time::Duration;

use minifb::{Window, WindowOptions};

use crate::cpu::Cpu;

pub const CHIP8_WIDTH: usize = 64;
pub const CHIP8_HEIGHT: usize = 32;

const SCALE: usize = 20;

const WINDOW_WIDTH: usize = CHIP8_WIDTH * SCALE;
const WINDOW_HEIGHT: usize = CHIP8_HEIGHT * SCALE;

const CPU_CYCLES_PER_FRAME: usize = 10;

const PIXEL_ON: u32 = 0x66FF99;
const PIXEL_OFF: u32 = 0x081208;

fn main() {

    let rom_path = std::env::args()
    .nth(1)
    .expect("Usage: chip8-emulator <rom>");

    let stream = OutputStreamBuilder::open_default_stream().unwrap();
    let sink = Sink::connect_new(stream.mixer());
    sink.append(
        SineWave::new(440.0)
            .amplify(0.15)
            .repeat_infinite()
    );
    sink.pause();

    // get rom from file and put it in rom_bytes
    let rom_bytes = std::fs::read(&rom_path).expect("Failed to read ROM");
    // create a new cpu
    let mut cpu = Cpu::new();
    // load the rom using the rom_bytes var
    cpu.load_rom(&rom_bytes);

    // crate a buffer, essentially a sheet of pixels
    let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

    // Create a new window.. hahahahah!
    let mut window = Window::new(
        "CHIP-8 Emulator",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .expect("Failed to create window");

    // You cant be that dumb.. this is self explanatory 
    window.set_target_fps(60);

    // close window if ESCAPE key is pressed
    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {

        update_keypad(&mut cpu, &window);

        // fetch instructs from rom and place them into opcode var
        for _ in 0..CPU_CYCLES_PER_FRAME {
            cpu.cycle();
        }

        if cpu.sound_timer > 0 {
            sink.play();
        } else {
            sink.pause();
        }

        cpu.update_timers();
        
        draw(&cpu, & mut buffer);

        // update window with buffer
        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }


}

fn update_keypad(cpu: &mut Cpu, window: &Window) {
        cpu.keypad[0x0] = window.is_key_down(minifb::Key::X);
        cpu.keypad[0x1] = window.is_key_down(minifb::Key::Key1);
        cpu.keypad[0x2] = window.is_key_down(minifb::Key::Key2);
        cpu.keypad[0x3] = window.is_key_down(minifb::Key::Key3);

        cpu.keypad[0x4] = window.is_key_down(minifb::Key::Q);
        cpu.keypad[0x5] = window.is_key_down(minifb::Key::W);
        cpu.keypad[0x6] = window.is_key_down(minifb::Key::E);
        cpu.keypad[0x7] = window.is_key_down(minifb::Key::A);

        cpu.keypad[0x8] = window.is_key_down(minifb::Key::S);
        cpu.keypad[0x9] = window.is_key_down(minifb::Key::D);
        cpu.keypad[0xA] = window.is_key_down(minifb::Key::Z);
        cpu.keypad[0xB] = window.is_key_down(minifb::Key::C);

        cpu.keypad[0xC] = window.is_key_down(minifb::Key::Key4);
        cpu.keypad[0xD] = window.is_key_down(minifb::Key::R);
        cpu.keypad[0xE] = window.is_key_down(minifb::Key::F);
        cpu.keypad[0xF] = window.is_key_down(minifb::Key::V);
}


fn draw(cpu: &Cpu, buffer: &mut [u32]) {
    for y in 0..CHIP8_HEIGHT { 
            for x in 0..CHIP8_WIDTH {
                let color = if cpu.display[y][x] {
                    PIXEL_ON
                } else {
                    PIXEL_OFF
                };

                // scale pixels according to the window 
                for dy in 0..SCALE {
                    for dx in 0..SCALE {
                        let px = x * SCALE + dx;
                        let py = y * SCALE + dy;

                        buffer[py * WINDOW_WIDTH + px] = color;
                    }
                }
            }
        }
}