use crate::emulator_layer::instructions::INSTRUCTIONS;

pub struct Emulator6502 {
    accumulator: u8,
    psr: u8,   // Processor status register (flags)
    sp: u8,    // Stack pointer; the lower byte of the stack location
    idx_x: u8, // Index X register for indirect addressing
    idx_y: u8, // Index Y register for indirect addressing
    pc: u16, // Program counter; strictly, this was actually formed from two 8-bit registers (PCH and PCL)

    memory: [u8; 2 ^ 16], // The 650X series can address 65K of memory

    // Various constants for the processor
    micros_per_cycle: u8,
}

impl Emulator6502 {
    pub fn new() -> Self {
        Self {
            accumulator: 0,
            psr: 0,
            sp: 0,
            idx_x: 0,
            idx_y: 0,
            pc: 0,
            memory: [0; 2 ^ 16],
            micros_per_cycle: 1, // The baseline 6502 was a 1 MHz processor
        }
    }

    fn negative_flag(&self) -> bool {
        (self.psr >> 7) & 1 == 1
    }

    fn overflow_flag(&self) -> bool {
        (self.psr >> 6) & 1 == 1
    }

    fn break_flag(&self) -> bool {
        (self.psr >> 4) & 1 == 1
    }

    fn decimal_flag(&self) -> bool {
        (self.psr >> 3) & 1 == 1
    }

    fn interrupt_disable_flag(&self) -> bool {
        (self.psr >> 2) & 1 == 1
    }

    fn zero_flag(&self) -> bool {
        (self.psr >> 1) & 1 == 1
    }

    fn carry_flag(&self) -> bool {
        self.psr & 1 == 1
    }

    fn pcl(&self) -> u8 {
        (self.pc & 0x11111111) as u8
    }

    fn pch(&self) -> u8 {
        ((self.pc >> 8) & 0x11111111) as u8
    }

    /// Making a function for this allows us to control access to memory (to implement things like actual read-only memory sections)
    pub fn write_memory(&mut self, addr: u16, value: u8) {
        // TODO Will need to investigate which areas (if any) are actually read-only. For now, just pass it through.
        self.memory[addr as usize] = value
    }

    pub fn read_memory(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn set_carry(&mut self) {
        self.psr = self.psr | 1;
    }

}