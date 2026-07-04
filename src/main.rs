use minifb::{self, CursorStyle::ResizeAll, Key::P, Window, WindowOptions};

const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;

const SCALE: usize = 20;

const WINDOW_WIDTH: usize = CHIP8_WIDTH * SCALE;
const WINDOW_HEIGHT: usize = CHIP8_HEIGHT * SCALE;

const FONTSET: [u8; 80] = [
    // 0
    0xF0, 0x90, 0x90, 0x90, 0xF0,
    // 1
    0x20, 0x60, 0x20, 0x20, 0x70,
    // 2
    0xF0, 0x10, 0xF0, 0x80, 0xF0,
    // 3
    0xF0, 0x10, 0xF0, 0x10, 0xF0,
    // 4
    0x90, 0x90, 0xF0, 0x10, 0x10,
    // 5
    0xF0, 0x80, 0xF0, 0x10, 0xF0,
    // 6
    0xF0, 0x80, 0xF0, 0x90, 0xF0,
    // 7
    0xF0, 0x10, 0x20, 0x40, 0x40,
    // 8
    0xF0, 0x90, 0xF0, 0x90, 0xF0,
    // 9
    0xF0, 0x90, 0xF0, 0x10, 0xF0,
    // A
    0xF0, 0x90, 0xF0, 0x90, 0x90,
    // B
    0xE0, 0x90, 0xE0, 0x90, 0xE0,
    // C
    0xF0, 0x80, 0x80, 0x80, 0xF0,
    // D
    0xE0, 0x90, 0x90, 0x90, 0xE0,
    // E
    0xF0, 0x80, 0xF0, 0x80, 0xF0,
    // F
    0xF0, 0x80, 0xF0, 0x80, 0x80,
];

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
        let mut cpu = Cpu {
            registers: [0; 16],
            index: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            memory: [0; 4096],
            display: [[false; 64]; 32],
        };

        for (i, &byte) in FONTSET.iter().enumerate() {
            cpu.memory[0x50 + i] = byte;
        }

        cpu
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
            0x0000 => {  
                // CLS
                match opcode {
                    0x00E0 => {
                        // clear screen
                        for row in &mut self.display {
                            row.fill(false);
                        }
                    }

                    0x00EE => {
                        // return
                        self.sp -= 1;
                        self.pc = self.stack[self.sp as usize];
                    }

                    _ => {
                        unimplemented!("Opcode {:04X}", opcode)
                    }
                }
             } 
            0x1000 => {  self.pc = nnn } // JP addr
            0x2000 => { 
                // CALL addr
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            } 
            0x3000 => { 
                // SE Vx, byte
                if self.registers[vx] == kk {
                    self.pc += 2
                }
            } 
            0x4000 => { 
                // SNE Vx, byte
                if self.registers[vx] != kk {
                    self.pc += 2
                }
            } 
            0x5000 => { 
                // SE Vx, Vy
                if self.registers[vx] == self.registers[vy] {
                    self.pc += 2;
                }
             } 
            0x6000 => {  self.registers[vx] = kk; } // LD Vx, byte
            0x7000 => {  self.registers[vx] = self.registers[vx].wrapping_add(kk) } // ADD Vx, byte
            0x8000 => { 
                // 8xy0 - LD Vx, Vy  
                // 8xy1 - OR Vx, Vy  
                // 8xy2 - AND Vx, Vy  
                // 8xy3 - XOR Vx, Vy  
                // 8xy4 - ADD Vx, Vy  
                // 8xy5 - SUB Vx, Vy 
                // 8xy6 - SHR Vx {, Vy} 
                // 8xy7 - SUBN Vx, Vy 
                // 8xyE - SHL Vx {, Vy} 

                match n {
                    0x0 => { self.registers[vx] = self.registers[vy]; }
                    0x1 => { self.registers[vx] |= self.registers[vy]; }
                    0x2 => { self.registers[vx] &= self.registers[vy];}
                    0x3 => { self.registers[vx] ^= self.registers[vy]; }
                    0x4 => { 
                        let (result, carry) = self.registers[vx].overflowing_add(self.registers[vy]);
                        self.registers[vx] = result;
                        self.registers[0xF] = carry as u8;
                    }   
                    0x5 => {
                        let (result, borrow) = self.registers[vx].overflowing_sub(self.registers[vy]);
                        self.registers[vx] = result;
                        self.registers[0xF] = (!borrow) as u8;
                    }
                    0x6 => {
                        self.registers[0xF] = self.registers[vx] & 1;
                        self.registers[vx] >>= 1;
                    }
                    0x7 => {
                        let (result, borrow) = self.registers[vy].overflowing_sub(self.registers[vx]);
                        self.registers[vx] = result;
                        self.registers[0x0F] = (!borrow) as u8;
                    }
                    0xE => {
                        self.registers[0xF] = (self.registers[vx] >> 7) & 1;
                        self.registers[vx] <<= 1;
                    }
                    _ => { unimplemented!("Opcode {:04X}", opcode); }
                }
            } 
            0x9000 => { 
                // SNE Vx, Vy
                if self.registers[vx] != self.registers[vy] {
                    self.pc += 2;
                }
            } 
            0xA000 => {  self.index = nnn } //  LD I, addr
            0xD000 => { 
                // decoder already extracted vx, vy and n
                // example: the opcode is D341
                // which means, `Draw a sprite that is 1 byte tall at coordinates (V3, V4)`
                
                // read the coordinates , vx and vy are not coordinates but rather REGISTER NUMBERS
                let x  = self.registers[vx] as usize;
                let y = self.registers[vy] as usize;
                
                // Reset the collision flag, if collision - it'll be changed back to 1
                self.registers[0xF] = 0;

                // loop through each sprite row, suppose n=5
                // this becomes
                // row = 0
                // row = 1
                // row = 2
                // row = 3
                // row = 4
                for row in 0..n {
                    // read the byte
                    // suppose I = 0x300
                    // 300 : 11110000
                    // 301 : 10010000
                    // 302 : 10010000
                    // 303 : 10010000
                    // 304 : 11110000
                    // Each byte is one horizontal line of the sprite
                    let sprite = self.memory[self.index as usize + row as usize];

                    // loop through every bit. (every sprite has 8 bits)
                    for col in 0..8 {
                        // Extract one pixel
                        let pixel = (sprite >> (7 - col)) & 1;

                        // ignore 0 bits, If the sprite bit is then dont draw anything.
                        if pixel == 1 {
                            // compute screen position
                            let px = (x + col) % CHIP8_WIDTH;
                            let py = (y + row as usize) % CHIP8_HEIGHT;
                            
                            // detect collision, supposed before drawing 
                            // █ exists
                            // The sprite also wants to draw
                            // █ on top
                            // CHIP-8 uses XOR drawing, so.. `1 XOR 1 = 0` and the pixel disappears
                            if self.display[py][px] {
                                self.registers[0xF] = 1;
                            }
                            
                            // Toggle the pixel
                            self.display[py][px] = !self.display[py][px];
                        }
                    }
                }
            }
            0xF000 => { 
                match kk {
                    0x29 => {
                        self.index = 0x50 + (self.registers[vx] as u16 * 5);
                    }

                    0x15 => {
                        self.delay_timer = self.registers[vx];
                    }

                    0x07 => {
                        self.registers[vx] = self.delay_timer;
                    }

                    0x18 => {
                        self.sound_timer = self.registers[vx];
                    }

                    0x1E => {
                        self.index += self.registers[vx] as u16;
                    }

                    0x33 => {
                        let value = self.registers[vx];

                        self.memory[self.index as usize] = value / 100;
                        self.memory[self.index as usize + 1] = (value / 10) % 10;
                        self.memory[self.index as usize + 2] = value % 10;
                    }

                    0x55 => {
                        for i in 0..=vx {
                            self.memory[self.index as usize + i] = self.registers[i];
                        }
                    }

                    0x65 => {
                        for i in 0..=vx {
                            self.registers[i] = self.memory[self.index as usize + i];
                        }
                    }

                    _ => todo!("Opcode {:04X}", opcode),
                }
             }
            _ => { unimplemented!("Opcode {:04X}", opcode); }
        }
        
    }
}

fn main() {
    // get rom from file and put it in rom_bytes
    let rom_bytes = std::fs::read("roms/Chip8 Picture.ch8").expect("Failed to read ROM");
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
        println!("{:03X}: {:04X}", cpu.pc - 2, opcode);
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
