use minifb::{self, Key::P, Window, WindowOptions};

const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;

const SCALE: usize = 20;

const WINDOW_WIDTH: usize = CHIP8_WIDTH * SCALE;
const WINDOW_HEIGHT: usize = CHIP8_HEIGHT * SCALE;

struct Cpu {
    pub registers: [u8; 16], // V0 through VF (VF is often used as a flag)
    pub index: u16, // I register — usually holds a memory address
    pub pc: u16, // program counter — points at the next instruction
    pub stack: [u16; 16], // return addresses for subroutine calls
    pub sp: u8, // stack pointer — index into the stack array
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub memory: [u8; 4096],


    // 64x32 pixel display
    pub display: [[bool; 64]; 32],
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: [0; 16],
            index: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            memory: [0; 4096],
            display: [[false; 64]; 32],
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        let start = 0x200;
        for (i, &byte) in rom.iter().enumerate() {
            self.memory[start + i] = byte;
        }
    }

    pub fn fetch(&mut self) -> u16 {
        // gets first byte, example.. 6A
        let high_byte = *self.memory.get(self.pc as usize).unwrap_or(&0) as u16;
        // gets second byte, example.. 12
        let low_byte = *self.memory.get((self.pc + 1) as usize).unwrap_or(&0) as u16;
        // the opcode is basically an entire instruction.. such as, 0x6A12
        // (high_byte << 8) basically sets it to 0x6A00
        // the OR (|) basically combines non overlapping piecies into one number.
        // basically:
        // 0000 0000 0000 0010
        // 1101 0101 0001 0001
        // ------------------- OR
        // 1101 0101 0001 0011 
        let opcode = (high_byte << 8) | low_byte;
        // moves the program counter two steps forward, 
        // such as:
        // i is currently 0
        // [i] [i+1]
        // [6A][12] 9D62 1D47 2C4A
        // i is currently 2
        //           [i] [i+1]
        // 6A12 9D62 [1D][47] 2C4A

        self.pc += 2;
        opcode
    }

    pub fn execute(&mut self, opcode: u16) {
        // `0x0F00` is a "filter", AND-ing anything with it will return the value sitting at the F's position.
        // for example, (0x6A12 & 0x00F0) will return 0x0010
        // now, since we want the nibble to be at the end of the byte, we use the `>>` operator which
        // essentially pushes the 1 (aka F) two nibbles to the right (since, 8 bits is 2 nibbles)
        let vx = ((opcode & 0x0F00) >> 8) as usize;
        // therefore,  ((0x6A12 & 0x00F0) >> 4) is 0x0001
        let vy   = ((opcode & 0x00F0) >> 4) as usize;   
        let n   = (opcode & 0x000F) as u8;          
        let kk = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;      

        match opcode & 0xF000 {
            0x0000 => {  // CLS
                match opcode {
                    0x00E0 => {
                        // clear screen
                        for row in &mut self.display {
                            row.fill(false);
                        }
                    }

                    0x00EE => {
                        // return
                        todo!()
                    }

                    _ => {
                        unimplemented!("Opcode {:04X}", opcode)
                    }
                }
             } 
            0x1000 => {  self.pc = nnn } // JP addr
            0x2000 => { todo!() }
            0x3000 => { todo!() }
            0x4000 => { todo!() }
            0x5000 => { todo!() }
            0x6000 => {  self.registers[vx] = kk; } // LD Vx, byte
            0x7000 => {  self.registers[vx] = self.registers[vx].wrapping_add(kk) } // ADD Vx, byte
            0x9000 => { todo!() }
            0xA000 => {  self.index = nnn } //  LD I, addr
            0xD000 => { todo!() }
            0xF000 => { todo!() }
            _ => { unimplemented!("Opcode {:04X}", opcode); }
        }
        
    }
}

fn main() {
    // get rom from file and put it in rom_bytes
    let rom_bytes = std::fs::read("roms/test.ch8").expect("Failed to read ROM");
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
        // fetch instructs from rom and place them into opcode var
        let opcode = cpu.fetch();
        // execute opcodes
        cpu.execute(opcode);
        
        for y in 0..CHIP8_HEIGHT { 
            for x in 0..CHIP8_WIDTH {
                let color = if cpu.display[y][x] {
                    0xFFFFFFFF
                } else {
                    0x00000000
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

        // update window with buffer
        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }


}
