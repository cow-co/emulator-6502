pub struct Opcode {
    op_bytes: u8,
    op_cycles: u8, // Important in allowing us to run the emulator at the correct speed
    mnemonic: &'static str, // For disassembling
    op_func: fn(&mut Emulator6502) -> (),
}

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
    instructions: [Opcode; 256],
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
            instructions: [
                // 0x00
                Opcode {
                    op_bytes: 1,
                    op_cycles: 7,
                    mnemonic: "BRK",
                },
                // 0x01
                Opcode {
                    op_bytes: 2,
                    op_cycles: 6,
                    mnemonic: "ORA (${}, X)",
                },
                // 0x02
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x03
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x04
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x05
                Opcode {
                    op_bytes: 2,
                    op_cycles: 3,
                    mnemonic: "ORA ${}",
                },
                // 0x06
                Opcode {
                    op_bytes: 1,
                    op_cycles: 1,
                    mnemonic: "ASL ${}",
                },
                // 0x07
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x08
                Opcode {
                    op_bytes: 1,
                    op_cycles: 3,
                    mnemonic: "PHP",
                },
                // 0x09
                Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "ORA #{}",
                },
                // 0x0A
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "ASL A",
                },
                // 0x0B
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x0C
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x0D
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "ORA ${}",
                },
                // 0x0E
                Opcode {
                    op_bytes: 3,
                    op_cycles: 6,
                    mnemonic: "ASL ${}",
                },
                // 0x0F
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x10
                Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "BPL ${}",
                },
                // 0x11
                Opcode {
                    op_bytes: 2,
                    op_cycles: 5,
                    mnemonic: "ORA (${}), Y",
                },
                // 0x12
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x13
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x14
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x15
                Opcode {
                    op_bytes: 2,
                    op_cycles: 4,
                    mnemonic: "ORA ${}",
                },
                // 0x16
                Opcode {
                    op_bytes: 2,
                    op_cycles: 6,
                    mnemonic: "ASL ${}, X",
                },
                // 0x17
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x18
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "CLC",
                },
                // 0x19
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "ORA ${}, Y",
                },
                // 0x1A
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x1B
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x1C
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x1D
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "ORA ${}, X",
                },
                // 0x1E
                Opcode {
                    op_bytes: 3,
                    op_cycles: 7,
                    mnemonic: "ASL ${}, X",
                },
                // 0x1F
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x20
                Opcode {
                    op_bytes: 3,
                    op_cycles: 6,
                    mnemonic: "JSR ${}",
                },
                // 0x21
                Opcode {
                    op_bytes: 2,
                    op_cycles: 6,
                    mnemonic: "AND (${}, X)",
                },
                // 0x22
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x23
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x24
                Opcode {
                    op_bytes: 2,
                    op_cycles: 3,
                    mnemonic: "BIT ${}",
                },
                // 0x25
                Opcode {
                    op_bytes: 2,
                    op_cycles: 3,
                    mnemonic: "AND ${}",
                },
                // 0x26
                Opcode {
                    op_bytes: 2,
                    op_cycles: 5,
                    mnemonic: "ROL ${}",
                },
                // 0x27
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x28
                Opcode {
                    op_bytes: 1,
                    op_cycles: 4,
                    mnemonic: "PLP",
                },
                // 0x29
                Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "AND #{}",
                    op_func: and_imm,
                },
                // 0x2A
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "ROL A",
                },
                // 0x2B
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x2C
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "BIT ${}",
                },
                // 0x2D
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "AND ${}",
                },
                // 0x2E
                Opcode {
                    op_bytes: 3,
                    op_cycles: 6,
                    mnemonic: "ROL ${}",
                },
                // 0x2F
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x30
                Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "BMI ${}",
                },
                // 0x31
                Opcode {
                    op_bytes: 2,
                    op_cycles: 5,
                    mnemonic: "AND (${}), Y",
                },
                // 0x32
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x33
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x34
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x35
                Opcode {
                    op_bytes: 2,
                    op_cycles: 4,
                    mnemonic: "AND ${}, X",
                },
                // 0x36
                Opcode {
                    op_bytes: 2,
                    op_cycles: 6,
                    mnemonic: "ROL ${}, X",
                },
                // 0x37
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x38
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "SEC",
                },
                // 0x39
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "AND ${}, Y",
                },
                // 0x3A
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x3B
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x3C
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x3D
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "AND ${}, X",
                },
                // 0x3E
                Opcode {
                    op_bytes: 3,
                    op_cycles: 7,
                    mnemonic: "ROL ${}, X",
                },
                // 0x3F
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x40
                Opcode {
                    op_bytes: 1,
                    op_cycles: 6,
                    mnemonic: "RTI",
                },
                // 0x41
                Opcode {
                    op_bytes: 2,
                    op_cycles: 6,
                    mnemonic: "EOR (${}, X)",
                },
                // 0x42
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x43
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x44
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x45
                Opcode {
                    op_bytes: 2,
                    op_cycles: 3,
                    mnemonic: "EOR ${}",
                },
                // 0x46
                Opcode {
                    op_bytes: 2,
                    op_cycles: 5,
                    mnemonic: "LSR ${}",
                },
                // 0x47
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x48
                Opcode {
                    op_bytes: 1,
                    op_cycles: 3,
                    mnemonic: "PHA",
                },
                // 0x49
                Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "EOR #{}",
                },
                // 0x4A
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "LSR A",
                },
                // 0x4B
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x4C
                Opcode {
                    op_bytes: 3,
                    op_cycles: 3,
                    mnemonic: "JMP ${}",
                },
                // 0x4D
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "EOR ${}",
                },
                // 0x4E
                Opcode {
                    op_bytes: 3,
                    op_cycles: 6,
                    mnemonic: "LSR ${}",
                },
                // 0x4F
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x50
                Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "BVC ${}",
                },
                // 0x51
                Opcode {
                    op_bytes: 2,
                    op_cycles: 5,
                    mnemonic: "EOR (${}), Y",
                },
                // 0x52
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x53
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x54
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x55
                Opcode {
                    op_bytes: 2,
                    op_cycles: 4,
                    mnemonic: "EOR ${}, X",
                },
                // 0x56
                Opcode {
                    op_bytes: 2,
                    op_cycles: 6,
                    mnemonic: "LSR ${}, X",
                },
                // 0x57
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x58
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "CLI",
                },
                // 0x59
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "EOR ${}, Y",
                },
                // 0x5A
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x5B
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x5C
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x5D
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "EOR ${}, X",
                },
                // 0x5E
                Opcode {
                    op_bytes: 3,
                    op_cycles: 7,
                    mnemonic: "LSR ${}, X",
                },
                // 0x5F
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x60
                Opcode {
                    op_bytes: 1,
                    op_cycles: 6,
                    mnemonic: "RTS",
                },
                // 0x61
                Opcode {
                    op_bytes: 2,
                    op_cycles: 6,
                    mnemonic: "ADC (${}, X)",
                    op_func: adc_imm,
                },
                // 0x62
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x63
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x64
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x65
                Opcode {
                    op_bytes: 2,
                    op_cycles: 5,
                    mnemonic: "ADC ${}",
                },
                // 0x66
                Opcode {
                    op_bytes: 2,
                    op_cycles: 5,
                    mnemonic: "ROR ${}",
                },
                // 0x67
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x68
                Opcode {
                    op_bytes: 1,
                    op_cycles: 4,
                    mnemonic: "PLA",
                },
                // 0x69
                Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "ADC #{}",
                },
                // 0x6A
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "ROR A",
                },
                // 0x6B
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x6C
                Opcode {
                    op_bytes: 3,
                    op_cycles: 5,
                    mnemonic: "JMP (${})",
                },
                // 0x6D
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "ADC #{}",
                },
                // 0x6E
                Opcode {
                    op_bytes: 3,
                    op_cycles: 6,
                    mnemonic: "ROR ${}",
                },
                // 0x6F
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x70
                Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "BVS ${}",
                },
                // 0x71
                Opcode {
                    op_bytes: 2,
                    op_cycles: 5,
                    mnemonic: "ADC (${}), Y",
                },
                // 0x72
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x73
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x74
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x75
                Opcode {
                    op_bytes: 2,
                    op_cycles: 4,
                    mnemonic: "ADC ${}, X",
                },
                // 0x76
                Opcode {
                    op_bytes: 2,
                    op_cycles: 6,
                    mnemonic: "ROR ${}, X",
                },
                // 0x77
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x78
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "SEI",
                },
                // 0x79
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "ADC ${}, Y",
                },
                // 0x7A
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x7B
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x7C
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x7D
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "ADC ${}, X",
                },
                // 0x7E
                Opcode {
                    op_bytes: 3,
                    op_cycles: 7,
                    mnemonic: "ROR ${}, X",
                },
                // 0x7F
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x80
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x81
                Opcode {
                    op_bytes: 2,
                    op_cycles: 6,
                    mnemonic: "STA (${}, X)",
                },
                // 0x82
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x83
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x84
                Opcode {
                    op_bytes: 2,
                    op_cycles: 3,
                    mnemonic: "STY ${}",
                },
                // 0x85
                Opcode {
                    op_bytes: 2,
                    op_cycles: 3,
                    mnemonic: "STA ${}",
                },
                // 0x86
                Opcode {
                    op_bytes: 2,
                    op_cycles: 3,
                    mnemonic: "STX ${}",
                },
                // 0x87
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x88
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "DEY",
                },
                // 0x89
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x8A
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "TXA",
                },
                // 0x8B
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x8C
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "STY ${}",
                },
                // 0x8D
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "STA ${}",
                },
                // 0x8E
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "STX ${}",
                },
                // 0x8F
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x90
                Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "BCC ${}",
                },
                // 0x91
                Opcode {
                    op_bytes: 2,
                    op_cycles: 6,
                    mnemonic: "STA (${}), Y",
                },
                // 0x92
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x93
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x94
                Opcode {
                    op_bytes: 2,
                    op_cycles: 4,
                    mnemonic: "STY ${}, X",
                },
                // 0x95
                Opcode {
                    op_bytes: 2,
                    op_cycles: 4,
                    mnemonic: "STA ${}, X",
                },
                // 0x96
                Opcode {
                    op_bytes: 2,
                    op_cycles: 4,
                    mnemonic: "STX ${}, Y",
                },
                // 0x97
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x98
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "TYA",
                },
                // 0x99
                Opcode {
                    op_bytes: 3,
                    op_cycles: 5,
                    mnemonic: "STA ${}, Y",
                },
                // 0x9A
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "TXS",
                },
                // 0x9B
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x9C
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x9D
                Opcode {
                    op_bytes: 3,
                    op_cycles: 5,
                    mnemonic: "STA ${}, X",
                },
                // 0x9E
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0x9F
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xA0
                Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "LDY #{}",
                },
                // 0xA1
                Opcode {
                    op_bytes: 2,
                    op_cycles: 6,
                    mnemonic: "LDA (${}, X)",
                },
                // 0xA2
                Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "LDX #{}",
                },
                // 0xA3
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xA4
                Opcode {
                    op_bytes: 2,
                    op_cycles: 3,
                    mnemonic: "LDY ${}",
                },
                // 0xA5
                Opcode {
                    op_bytes: 2,
                    op_cycles: 3,
                    mnemonic: "LDA ${}",
                },
                // 0xA6
                Opcode {
                    op_bytes: 2,
                    op_cycles: 3,
                    mnemonic: "LDX ${}",
                },
                // 0xA7
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xA8
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "TAY",
                },
                // 0xA9
                Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "LDA #{}",
                },
                // 0xAA
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "TAX",
                },
                // 0xAB
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xAC
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "LDY ${}",
                },
                // 0xAD
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "LDA ${}",
                },
                // 0xAE
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "LDX ${}",
                },
                // 0xAF
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xB0
                Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "BCS ${}",
                },
                // 0xB1
                Opcode {
                    op_bytes: 2,
                    op_cycles: 5,
                    mnemonic: "LDA (${}), Y",
                },
                // 0xB2
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xB3
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xB4
                Opcode {
                    op_bytes: 2,
                    op_cycles: 4,
                    mnemonic: "LDY ${}, X",
                },
                // 0xB5
                Opcode {
                    op_bytes: 2,
                    op_cycles: 4,
                    mnemonic: "LDA ${}, X",
                },
                // 0xB6
                Opcode {
                    op_bytes: 2,
                    op_cycles: 4,
                    mnemonic: "LDX ${}, Y",
                },
                // 0xB7
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xB8
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "CLV",
                },
                // 0xB9
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "LDA ${}, Y",
                },
                // 0xBA
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "TSX",
                },
                // 0xBB
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xBC
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "LDY ${}, X",
                },
                // 0xBD
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "LDA ${}, X",
                },
                // 0xBE
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "LDX ${}, Y",
                },
                // 0xBF
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xC0
                Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "CPY #{}",
                },
                // 0xC1
                Opcode {
                    op_bytes: 2,
                    op_cycles: 6,
                    mnemonic: "CMP (${}, X)",
                },
                // 0xC2
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xC3
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xC4
                Opcode {
                    op_bytes: 2,
                    op_cycles: 3,
                    mnemonic: "CPY ${}",
                },
                // 0xC5
                Opcode {
                    op_bytes: 2,
                    op_cycles: 3,
                    mnemonic: "CMP ${}",
                },
                // 0xC6
                Opcode {
                    op_bytes: 2,
                    op_cycles: 5,
                    mnemonic: "DEC ${}",
                },
                // 0xC7
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xC8
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "INY",
                },
                // 0xC9
                Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "CMP #{}",
                },
                // 0xCA
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "DEX",
                },
                // 0xCB
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xCC
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "CPY ${}",
                },
                // 0xCD
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "CMP ${}",
                },
                // 0xCE
                Opcode {
                    op_bytes: 3,
                    op_cycles: 6,
                    mnemonic: "DEC ${}",
                },
                // 0xCF
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xD0
                Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "BNE ${}",
                },
                // 0xD1
                Opcode {
                    op_bytes: 2,
                    op_cycles: 5,
                    mnemonic: "CMP (${}), Y",
                },
                // 0xD2
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xD3
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xD4
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xD5
                Opcode {
                    op_bytes: 2,
                    op_cycles: 4,
                    mnemonic: "CMP ${}, X",
                },
                // 0xD6
                Opcode {
                    op_bytes: 2,
                    op_cycles: 6,
                    mnemonic: "DEC ${}, X",
                },
                // 0xD7
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xD8
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "CLD",
                },
                // 0xD9
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "CMP ${}, Y",
                },
                // 0xDA
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xDB
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xDC
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xDD
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "CMP ${}, X",
                },
                // 0xDE
                Opcode {
                    op_bytes: 3,
                    op_cycles: 7,
                    mnemonic: "DEC ${}, X",
                },
                // 0xDF
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xE0
                Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "CPX #{}",
                },
                // 0xE1
                Opcode {
                    op_bytes: 2,
                    op_cycles: 6,
                    mnemonic: "SBC (${}, X)",
                },
                // 0xE2
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xE3
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xE4
                Opcode {
                    op_bytes: 2,
                    op_cycles: 3,
                    mnemonic: "CPX ${}",
                },
                // 0xE5
                Opcode {
                    op_bytes: 2,
                    op_cycles: 3,
                    mnemonic: "SBC ${}",
                },
                // 0xE6
                Opcode {
                    op_bytes: 2,
                    op_cycles: 5,
                    mnemonic: "INC ${}",
                },
                // 0xE7
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xE8
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "INX",
                },
                // 0xE9
                Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "SBC #{}",
                },
                // 0xEA
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xEB
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xEC
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "CPX ${}",
                },
                // 0xED
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "SBC ${}",
                },
                // 0xEE
                Opcode {
                    op_bytes: 3,
                    op_cycles: 6,
                    mnemonic: "INC ${}",
                },
                // 0xEF
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xF0
                Opcode {
                    op_bytes: 2,
                    op_cycles: 2,
                    mnemonic: "BEQ ${}",
                },
                // 0xF1
                Opcode {
                    op_bytes: 2,
                    op_cycles: 5,
                    mnemonic: "SBC (${}), Y",
                },
                // 0xF2
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xF3
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xF4
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xF5
                Opcode {
                    op_bytes: 2,
                    op_cycles: 4,
                    mnemonic: "SBC ${}, X",
                },
                // 0xF6
                Opcode {
                    op_bytes: 2,
                    op_cycles: 6,
                    mnemonic: "INC ${}, X",
                },
                // 0xF7
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xF8
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "SED",
                },
                // 0xF9
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "SBC ${}, Y",
                },
                // 0xFA
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xFB
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xFC
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
                // 0xFD
                Opcode {
                    op_bytes: 3,
                    op_cycles: 4,
                    mnemonic: "SBC ${}, X",
                },
                // 0xFE
                Opcode {
                    op_bytes: 3,
                    op_cycles: 7,
                    mnemonic: "INC ${}, X",
                },
                // 0xFF
                Opcode {
                    op_bytes: 1,
                    op_cycles: 2,
                    mnemonic: "NOP",
                    op_func: nop,
                },
            ],
        }
    }

    pub fn run_emulation(&self) {}

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

}

/// No-op! Does nothing!
/// 
/// Define this explicitly for two reasons:
/// - The opcodes require a function to execute (and this is easier and safer than allowing them to have that field empty)
/// - This allows us to potentially slot some debugging in here to run whenever we get a NOP
fn nop(_state: &mut Emulator6502) -> () {
    
}

/// Fetches a byte from the given memory location in page zero.
/// 
/// Strictly, this is just an index directly into the `memory` array on the `state` object, so we could skip this,
/// but having a function for it allows us to do debugging and other stuff.
fn zero_page_value(state: &Emulator6502, addr: u8) -> u8 {
    state.memory[addr as usize]
}

/// Fetches a value from memory, via the indexed indirect addressing technique, which uses the X register.
/// 
/// Not to be confused with indirect indexed addressing!
/// 
/// Indexed indirect (pre-indexed) addressing basically constructs an "indexed" address from the second operand of the instruction, 
/// plus the value in the X register. This then points to a location in memory which is where the *effective* address is stored.
/// 
/// `indexed_addr` should be *without* the X register added (this function takes care of that internally)
fn indexed_indirect_value(state: &Emulator6502, indexed_addr: u8) -> u8 {
    let indirected = indexed_addr + state.idx_x;
    let low_byte = zero_page_value(state, indirected);
    let high_byte = zero_page_value(state, indirected+1);
    let effective_addr = double_byte_from_two_single_bytes(low_byte, high_byte);
    state.memory[effective_addr as usize]
}

/// Fetches a value from memory, via the indirect indexed addressing technique, which uses the Y register.
/// 
/// Not to be confused with indexed indirect addressing!
/// 
/// With indirect indexed (post-indexed) addressing, we take the second operand of the instruction, use that as a pointer to 
/// where the indirect address is stored (in page 0); once that address is fetched, we index (add) it with the value stored in the 
/// Y register, and use the result as our effective address to retrieve our final output.
fn indirect_indexed_value(state: &Emulator6502, addr: u8) -> u8 {
    let low_byte = zero_page_value(state, addr);
    let high_byte = zero_page_value(state, addr+1);
    let indirect_addr = double_byte_from_two_single_bytes(low_byte, high_byte);
    let effective_addr = indirect_addr + state.idx_y as u16;
    state.memory[effective_addr as usize]
}

/// Concatenates two single-byte values into one 16-bit value. Usually used for constructing memory addresses.
///
/// Note the order of the low and high bytes.
fn double_byte_from_two_single_bytes(low_byte: u8, high_byte: u8) -> u16 {
    let mut result = low_byte as u16;
    result = result & (high_byte as u16) << 8;
    result
}

fn adc(state: &mut Emulator6502, value: u8) -> () {
    // TODO Determine if there'll be a carry
    state.accumulator = state.accumulator + value + (state.carry_flag() as u8);
    // TODO Set flags
}

fn adc_imm(state: &mut Emulator6502) -> () {
    adc(state, state.memory[(state.pc + 1) as usize]);
}

fn and(state: &mut Emulator6502, value: u8) -> () {
    state.accumulator = state.accumulator & value;
    // TODO Set flags
}

fn and_imm(state: &mut Emulator6502) -> () {
    and(state, state.memory[(state.pc + 1) as usize]);
}
