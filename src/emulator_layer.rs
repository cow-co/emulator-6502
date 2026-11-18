
pub struct Emulator6502 {
    accumulator: u8,
    psr: u8,    // Processor status register (flags)
    sp: u8,     // Stack pointer; the lower byte of the stack location
    idx_x: u8,  // Index X register for indirect addressing
    idx_y: u8,  // Index Y register for indirect addressing
    pcl: u8,    // Low byte of the program counter
    pch: u8,    // High byte of the program counter
}

impl Emulator6502 {
    pub fn new() -> Self {
        Self { 
            
        }
    }

    pub fn run_emulation(&self) {

    }
}