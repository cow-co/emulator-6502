mod opcodes;

pub struct Emulator6502 {
    accumulator: u8,
    psr: u8,    // Processor status register (flags)
    sp: u8,     // Stack pointer; the lower byte of the stack location
    idx_x: u8,  // Index X register for indirect addressing
    idx_y: u8,  // Index Y register for indirect addressing
    pcl: u8,    // Low byte of the program counter
    pch: u8,    // High byte of the program counter

    memory: [u8; 2^16], // The 650X series can address 65K of memory

    // Various constants for the processor
    micros_per_cycle: u8,
    instructions: [opcodes::Opcode; 256],
}

impl Emulator6502 {
    pub fn new() -> Self {
        Self { 
            accumulator: 0,
            psr: 0,
            sp: 0,
            idx_x: 0,
            idx_y: 0,
            pcl: 0,
            pch: 0,
            memory: [0; 2^16],
            micros_per_cycle: 1,    // The baseline 6502 was a 1 MHz processor
            instructions: [
                // 0x00
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 7,
                    mnemonic: "BRK"
                },
                // 0x01
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 6,
                    mnemonic: "ORA ($NN, X)"
                },
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 1,
                    mnemonic: "NOP"
                },
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 1,
                    mnemonic: "NOP"
                },
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 1,
                    mnemonic: "NOP"
                },
                // 0x05
            ]
        }
    }

    pub fn run_emulation(&self) {

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
}