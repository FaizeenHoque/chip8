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
        let high_byte = *self.memory.get(self.pc as usize).unwrap_or(&0) as u16;
        let low_byte = *self.memory.get((self.pc + 1) as usize).unwrap_or(&0) as u16;
        let opcode = (high_byte << 8) | low_byte;
        self.pc += 2;
        opcode
    }
}

fn main() {
    let rom_bytes = std::fs::read("roms/Puzzle.ch8").expect("Failed to read ROM");
    let mut cpu = Cpu::new();
    cpu.load_rom(&rom_bytes);

    loop {
        let opcode = cpu.fetch();
        println!("{:04X}", opcode);
        // decode + execute 
    }
}
