pub struct Opcode {
    // TODO Maybe have addressing-type as a field here? Would that actually help at all though?
    op_bytes: u8,
    op_cycles: u8,      // Important in allowing us to run the emulator at the correct speed
    mnemonic: str,   // For disassembling
    op_func: fn() -> (),
}

pub struct Emulator6502 {
    accumulator: u8,
    psr: u8,    // Processor status register (flags)
    sp: u8,     // Stack pointer; the lower byte of the stack location
    idx_x: u8,  // Index X register for indirect addressing
    idx_y: u8,  // Index Y register for indirect addressing
    pc: u16,    // Program counter; strictly, this was actually formed from two 8-bit registers (PCH and PCL)

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
            pc:0,
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
                    mnemonic: "ORA (${}, X)"
                },
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x05
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 3,
                    mnemonic: "ORA ${}",
                },
                // 0x06
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 1,
                    mnemonic: "ASL ${}",
                },
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x08
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 1,
                    mnemonic: "PHP",
                },
                // 0x08
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 3,
                    mnemonic: "PHP",
                },
                // 0x09
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "ORA #{}",
                },
                // 0x0A
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "ASL A",
                },
                // 0x0D
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "ORA ${}",
                },
                // 0x0E
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 6,
                    mnemonic: "ASL ${}",
                },
                // 0x10
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "BPL ${}",
                },
                // 0x11
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 5,
                    mnemonic: "ORA (${}), Y",
                },
                // 0x15
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 4,
                    mnemonic: "ORA ${}",
                },
                // 0x16
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 6,
                    mnemonic: "ASL ${}, X",
                },
                // 0x18
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "CLC",
                },
                // 0x19
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "ORA ${}, Y",
                },
                // 0x1D
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "ORA ${}, X",
                },
                // 0x1E
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 7,
                    mnemonic: "ASL ${}, X",
                },
                // 0x20
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 6,
                    mnemonic: "JSR ${}",
                },
                // 0x21
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 6,
                    mnemonic: "AND (${}, X)",
                },
                // 0x24
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 3,
                    mnemonic: "BIT ${}",
                },
                // 0x25
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 3,
                    mnemonic: "AND ${}",
                },
                // 0x26
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 5,
                    mnemonic: "ROL ${}",
                },
                // 0x28
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 4,
                    mnemonic: "PLP",
                },
                // 0x29
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "AND #{}",
                },
                // 0x2A
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "ROL A",
                },
                // 0x2C
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "BIT ${}",
                },
                // 0x2D
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "AND ${}",
                },
                // 0x2E
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 6,
                    mnemonic: "ROL ${}",
                },
                // 0x30
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "BMI ${}",
                },
                // 0x31
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 5,
                    mnemonic: "AND (${}), Y",
                },
                // 0x35
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 4,
                    mnemonic: "AND ${}, X",
                },
                // 0x36
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 6,
                    mnemonic: "ROL ${}, X",
                },
                // 0x38
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "SEC",
                },
                // 0x39
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "AND ${}, Y",
                },
                // 0x3D
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "AND ${}, X",
                },
                // 0x3E
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 7,
                    mnemonic: "ROL ${}, X",
                },
                // 0x40
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 6,
                    mnemonic: "RTI",
                },
                // 0x41
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 6,
                    mnemonic: "EOR (${}, X)",
                },
                // 0x45
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 3,
                    mnemonic: "EOR ${}",
                },
                // 0x46
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 5,
                    mnemonic: "LSR ${}",
                },
                // 0x48
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 3,
                    mnemonic: "PHA",
                },
                // 0x49
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "EOR #{}",
                },
                // 0x4A
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "LSR A",
                },
                // 0x4C
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 3,
                    mnemonic: "JMP ${}",
                },
                // 0x4D
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "EOR ${}",
                },
                // 0x4E
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 6,
                    mnemonic: "LSR ${}",
                },
                // 0x50
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "BVC ${}",
                },
                // 0x51
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 5,
                    mnemonic: "EOR (${}), Y",
                },
                // 0x55
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 4,
                    mnemonic: "EOR ${}, X",
                },
                // 0x56
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 6,
                    mnemonic: "LSR ${}, X",
                },
                // 0x58
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "CLI",
                },
                // 0x59
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "EOR ${}, Y",
                },
                // 0x5D
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "EOR ${}, X",
                },
                // 0x5E
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 7,
                    mnemonic: "LSR ${}, X",
                },
                // 0x60
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 6,
                    mnemonic: "RTS",
                },
                // 0x61
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 6,
                    mnemonic: "ADC (${}, X)",
                },
                // 0x65
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 5,
                    mnemonic: "ADC ${}",
                },
                // 0x66
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 5,
                    mnemonic: "ROR ${}",
                },
                // 0x68
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 4,
                    mnemonic: "PLA",
                },
                // 0x69
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "ADC #{}",
                },
                // 0x6A
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "ROR A",
                },
                // 0x6C
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 5,
                    mnemonic: "JMP (${})",
                },
                // 0x6D
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "ADC #{}",
                },
                // 0x6E
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 6,
                    mnemonic: "ROR ${}",
                },
                // 0x70
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "BVS ${}",
                },
                // 0x71
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 5,
                    mnemonic: "ADC (${}), Y",
                },
                // 0x75
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 4,
                    mnemonic: "ADC ${}, X",
                },
                // 0x76
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 6,
                    mnemonic: "ROR ${}, X",
                },
                // 0x78
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "SEI",
                },
                // 0x79
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "ADC ${}, Y",
                },
                // 0x7D
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "ADC ${}, X",
                },
                // 0x7E
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 7,
                    mnemonic: "ROR ${}, X",
                },
                // 0x81
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 6,
                    mnemonic: "STA (${}, X)",
                },
                // 0x86
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 3,
                    mnemonic: "STY ${}",
                },
                // 0x85
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 3,
                    mnemonic: "STA ${}",
                },
                // 0x86
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 3,
                    mnemonic: "STX ${}",
                },
                // 0x88
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "DEY",
                },
                // 0x8A
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "TXA",
                },
                // 0x8C
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "STY ${}",
                },
                // 0x8D
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "STA ${}",
                },
                // 0x8E
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "STX ${}",
                },
                // 0x90
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "BCC ${}",
                },
                // 0x91
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 6,
                    mnemonic: "STA (${}), Y",
                },
                // 0x96
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 4,
                    mnemonic: "STY ${}, X",
                },
                // 0x95
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 4,
                    mnemonic: "STA ${}, X",
                },
                // 0x96
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 4,
                    mnemonic: "STX ${}, Y",
                },
                // 0xA8
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "TYA",
                },
                // 0x99
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 5,
                    mnemonic: "STA ${}, Y",
                },
                // 0x9A
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "TXS",
                },
                // 0x9D
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 5,
                    mnemonic: "STA ${}, X",
                },
                // 0xA0
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "LDY #{}",
                },
                // 0xA1
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 6,
                    mnemonic: "LDA (${}, X)",
                },
                // 0xA2
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "LDX #{}",
                },
                // 0xA4
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 3,
                    mnemonic: "LDY ${}",
                },
                // 0xA5
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 3,
                    mnemonic: "LDA ${}",
                },
                // 0xA6
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 3,
                    mnemonic: "LDX ${}",
                },
                // 0xA8
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "TAY",
                },
                // 0xA9
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "LDA #{}",
                },
                // 0xAA
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "TAX",
                },
                // 0xAC
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "LDY ${}",
                },
                // 0xAD
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "LDA ${}",
                },
                // 0xAE
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "LDX ${}",
                },
                // 0xB0
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "BCS ${}",
                },
                // 0xB1
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 5,
                    mnemonic: "LDA (${}), Y",
                },
                // 0xB4
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 4,
                    mnemonic: "LDY ${}, X",
                },
                // 0xB5
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 4,
                    mnemonic: "LDA ${}, X",
                },
                // 0xB6
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 4,
                    mnemonic: "LDX ${}, Y",
                },
                // 0xB8
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "CLV",
                },
                // 0xB9
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "LDA ${}, Y",
                },
                // 0xBA
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "TSX",
                },
                // 0xBC
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "LDY ${}, X",
                },
                // 0xBD
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "LDA ${}, X",
                },
                // 0xBE
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "LDX ${}, Y",
                },
                // 0xC0
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "CPY #{}",
                },
                // 0xC1
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 6,
                    mnemonic: "CMP (${}, X)",
                },
                // 0xC4
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 3,
                    mnemonic: "CPY ${}",
                },
                // 0xC5
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 3,
                    mnemonic: "CMP ${}",
                },
                // 0xC6
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 5,
                    mnemonic: "DEC ${}",
                },
                // 0xC8
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "INY",
                },
                // 0xC9
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "CMP #{}",
                },
                // 0xCA
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "DEX",
                },
                // 0xCC
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "CPY ${}",
                },
                // 0xCD
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "CMP ${}",
                },
                // 0xCE
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 6,
                    mnemonic: "DEC ${}",
                },
                // 0xD0
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "BNE ${}",
                },
                // 0xD1
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 5,
                    mnemonic: "CMP (${}), Y",
                },
                // 0xD5
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 4,
                    mnemonic: "CMP ${}, X",
                },
                // 0xD6
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 6,
                    mnemonic: "DEC ${}, X",
                },
                // 0xD8
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "CLD",
                },
                // 0xD9
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "CMP ${}, Y",
                },
                // 0xDD
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "CMP ${}, X",
                },
                // 0xDE
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 7,
                    mnemonic: "DEC ${}, X",
                },
                // 0xE0
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "CPX #{}",
                },
                // 0xE1
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 6,
                    mnemonic: "SBC (${}, X)",
                },
                // 0xE4
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 3,
                    mnemonic: "CPX ${}",
                },
                // 0xE5
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 3,
                    mnemonic: "SBC ${}",
                },
                // 0xE6
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 5,
                    mnemonic: "INC ${}",
                },
                // 0xE8
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "INX",
                },
                // 0xE9
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "SBC #{}",
                },
                // 0xEA
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xEC
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "CPX ${}",
                },
                // 0xED
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "SBC ${}",
                },
                // 0xEE
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 6,
                    mnemonic: "INC ${}",
                },
                // 0xF0
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "BEQ ${}",
                },
                // 0xF1
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 5,
                    mnemonic: "SBC (${}), Y",
                },
                // 0xF5
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 4,
                    mnemonic: "SBC ${}, X",
                },
                // 0xF6
                opcodes::Opcode {
                    op_bytes: 2,
                    op_cycles: 6,
                    mnemonic: "INC ${}, X",
                },
                // 0xF8
                opcodes::Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "SED",
                },
                // 0xF9
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "SBC ${}, Y",
                },
                // 0xFD
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "SBC ${}, X",
                },
                // 0xFE
                opcodes::Opcode {
                    op_bytes: 3,
                    op_cycles: 7,
                    mnemonic: "INC ${}, X",
                },
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

    pub fn pcl(&self) -> u8 {
        (self.pc & 0x11111111) as u8
    }

    pub fn pch(&self) -> u8 {
        ((self.pc >> 8) & 0x11111111) as u8
    }
}




fn nop(state: &mut Emulator6502) -> () {
    state.pc += 1;
}