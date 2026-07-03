struct Cpu {
    pub registers: [u8; 16], // V0 through VF (VF is often used as a flag)
    pub index: u16, // I register — usually holds a memory address
    pub pc: u16, // program counter — points at the next instruction
    pub stack: [u16; 16], // return addresses for subroutine calls
    pub sp: u8, // stack pointer — index into the stack array
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub memory: [u8; 4096]
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
            0x0000 => { 
                match opcode {
                    0x00E0 => {
                        // clear screen
                    }

                    0x00EE => {
                        // return
                    }

                    _ => {
                        unimplemented!("Opcode {:04X}", opcode)
                    }
                }
             } // CLS
            0x1000 => {  self.pc = nnn } // JP addr
            0x6000 => {  self.registers[vx] = kk; } // LD Vx, byte
            0x7000 => {  self.registers[vx] = self.registers[vx].wrapping_add(kk) } // ADD Vx, byte
            0xA000 => {  self.index = nnn } //  LD I, addr
            _ => { unimplemented!("Opcode {:04X}", opcode); }
        }
        
    }
}

fn main() {
    let rom_bytes = std::fs::read("roms/Puzzle.ch8").expect("Failed to read ROM");
    let mut cpu = Cpu::new();
    cpu.load_rom(&rom_bytes);

    loop {
        let opcode = cpu.fetch();
        // println!("{:04X}", opcode);
        // decode + execute 
        cpu.execute(opcode);
                    
    }
}
