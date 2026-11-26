pub struct Opcode {
    op_cycles: u8, // Important in allowing us to run the emulator at the correct speed
    /// The `bool` is to denote whether we are in disassemble mode 
    pub op_func: fn(&mut CPU6502, bool) -> (),
}

pub struct CPU6502 {
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

impl CPU6502 {
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
                op_cycles: 7,
                op_func: CPU6502::brk,
            },
            // 0x01
            Opcode {
                op_cycles: 6,
                op_func: CPU6502::ora_indirect_x,
            },
            // 0x02
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x03
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x04
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x05
            Opcode {
                op_cycles: 3,
                op_func: CPU6502::ora_zero_page,
            },
            // 0x06
            Opcode {
                op_cycles: 1,
                op_func: CPU6502::asl_zero_page,
            },
            // 0x07
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x08
            Opcode {
                op_cycles: 3,
                op_func: CPU6502::php,
            },
            // 0x09
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::ora_imm,
            },
            // 0x0A
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::asl_a,
            },
            // 0x0B
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x0C
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x0D
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::ora_absolute,
            },
            // 0x0E
            Opcode {
                op_cycles: 6,
                op_func: CPU6502::asl_absolute,
            },
            // 0x0F
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x10
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::bpl,
            },
            // 0x11
            Opcode {
                op_cycles: 5,
                op_func: CPU6502::ora_indirect_y,
            },
            // 0x12
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x13
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x14
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x15
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::ora_zero_page_x,
            },
            // 0x16
            Opcode {
                op_cycles: 6,
                op_func: CPU6502::asl_zero_page_x,
            },
            // 0x17
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x18
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::clc,
            },
            // 0x19
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::ora_absolute_y,
            },
            // 0x1A
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x1B
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x1C
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x1D
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::ora_absolute_x,
            },
            // 0x1E
            Opcode {
                op_cycles: 7,
                op_func: CPU6502::asl_absolute_x,
            },
            // 0x1F
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x20
            Opcode {
                op_cycles: 6,
                op_func: CPU6502::jsr,
            },
            // 0x21
            Opcode {
                op_cycles: 6,
                op_func: CPU6502::and_indirect_x,
            },
            // 0x22
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x23
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x24
            Opcode {
                op_cycles: 3,
                op_func: CPU6502::bit_zero_page,
            },
            // 0x25
            Opcode {
                op_cycles: 3,
                op_func: CPU6502::and_zero_page,
            },
            // 0x26
            Opcode {
                op_cycles: 5,
                op_func: CPU6502::rol_zero_page,
            },
            // 0x27
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x28
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::plp,
            },
            // 0x29
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::and_imm,
            },
            // 0x2A
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::rol_a,
            },
            // 0x2B
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x2C
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::bit_absolute,
            },
            // 0x2D
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::and_absolute,
            },
            // 0x2E
            Opcode {
                op_cycles: 6,
                op_func: CPU6502::rol_absolute,
            },
            // 0x2F
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x30
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::bmi,
            },
            // 0x31
            Opcode {
                op_cycles: 5,
                op_func: CPU6502::and_indirect_y,
            },
            // 0x32
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x33
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x34
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x35
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::and_zero_page_x,
            },
            // 0x36
            Opcode {
                op_cycles: 6,
                op_func: CPU6502::rol_zero_page_x,
            },
            // 0x37
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x38
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::sec,
            },
            // 0x39
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::and_absolute_y,
            },
            // 0x3A
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x3B
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x3C
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x3D
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::and_absolute_x,
            },
            // 0x3E
            Opcode {
                op_cycles: 7,
                op_func: CPU6502::rol_absolute_x,
            },
            // 0x3F
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x40
            Opcode {
                op_cycles: 6,
                op_func: CPU6502::rti,
            },
            // 0x41
            Opcode {
                op_cycles: 6,
                op_func: CPU6502::eor_indirect_x,
            },
            // 0x42
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x43
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x44
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x45
            Opcode {
                op_cycles: 3,
                op_func: CPU6502::eor_zero_page,
            },
            // 0x46
            Opcode {
                op_cycles: 5,
                op_func: CPU6502::lsr_zero_page,
            },
            // 0x47
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x48
            Opcode {
                op_cycles: 3,
                op_func: CPU6502::pha,
            },
            // 0x49
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::eor_imm,
            },
            // 0x4A
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::lsr_a,
            },
            // 0x4B
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x4C
            Opcode {
                op_cycles: 3,
                op_func: CPU6502::jmp_absolute,
            },
            // 0x4D
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::eor_absolute,
            },
            // 0x4E
            Opcode {
                op_cycles: 6,
                op_func: CPU6502::lsr_absolute,
            },
            // 0x4F
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x50
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::bvc,
            },
            // 0x51
            Opcode {
                op_cycles: 5,
                op_func: CPU6502::eor_indirect_y,
            },
            // 0x52
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x53
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x54
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x55
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::eor_zero_page_x,
            },
            // 0x56
            Opcode {
                op_cycles: 6,
                op_func: CPU6502::lsr_zero_page_x,
            },
            // 0x57
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x58
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::cli,
            },
            // 0x59
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::eor_absolute_y,
            },
            // 0x5A
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x5B
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x5C
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x5D
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::eor_absolute_x,
            },
            // 0x5E
            Opcode {
                op_cycles: 7,
                op_func: CPU6502::lsr_absolute_x,
            },
            // 0x5F
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x60
            Opcode {
                op_cycles: 6,
                op_func: CPU6502::rts,
            },
            // 0x61
            Opcode {
                op_cycles: 6,
                op_func: CPU6502::adc_indirect_x,
            },
            // 0x62
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x63
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x64
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x65
            Opcode {
                op_cycles: 5,
                op_func: CPU6502::adc_zero_page,
            },
            // 0x66
            Opcode {
                op_cycles: 5,
                op_func: CPU6502::ror_zero_page,
            },
            // 0x67
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x68
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::pla,
            },
            // 0x69
            Opcode {
                op_cycles: 2,
            },
            // 0x6A
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::ror_a,
            },
            // 0x6B
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x6C
            Opcode {
                op_cycles: 5,
                op_func: CPU6502::jmp_indirect,
            },
            // 0x6D
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::adc_imm,
            },
            // 0x6E
            Opcode {
                op_cycles: 6,
                op_func: CPU6502::ror_absolute,
            },
            // 0x6F
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x70
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::bvs,
            },
            // 0x71
            Opcode {
                op_cycles: 5,
                op_func: CPU6502::adc_indirect_y,
            },
            // 0x72
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x73
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x74
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x75
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::adc_zero_page_x,
            },
            // 0x76
            Opcode {
                op_cycles: 6,
                op_func: CPU6502::ror_zero_page_x,
            },
            // 0x77
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x78
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::sei,
            },
            // 0x79
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::adc_absolute_y,
            },
            // 0x7A
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x7B
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x7C
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x7D
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::adc_absolute_x,
            },
            // 0x7E
            Opcode {
                op_cycles: 7,
                op_func: CPU6502::ror_absolute_x,
            },
            // 0x7F
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x80
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x81
            Opcode {
                op_cycles: 6,
                op_func: CPU6502::sta_indirect_x,
            },
            // 0x82
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x83
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x84
            Opcode {
                op_cycles: 3,
                op_func: CPU6502::sty_zero_page,
            },
            // 0x85
            Opcode {
                op_cycles: 3,
                op_func: CPU6502::sta_zero_page,
            },
            // 0x86
            Opcode {
                op_cycles: 3,
                op_func: CPU6502::stx_zero_page,
            },
            // 0x87
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x88
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::dey,
            },
            // 0x89
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x8A
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::txa,
            },
            // 0x8B
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x8C
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::sty_absolute,
            },
            // 0x8D
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::sta_absolute,
            },
            // 0x8E
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::stx_absolute,
            },
            // 0x8F
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x90
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::bcc,
            },
            // 0x91
            Opcode {
                op_cycles: 6,
                op_func: CPU6502::sta_indirect_y,
            },
            // 0x92
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x93
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x94
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::sty_zero_page_x,
            },
            // 0x95
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::sta_zero_page_x,
            },
            // 0x96
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::stx_zero_page_y,
            },
            // 0x97
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x98
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::tya,
            },
            // 0x99
            Opcode {
                op_cycles: 5,
                op_func: CPU6502::sta_absolute_y,
            },
            // 0x9A
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::txs,
            },
            // 0x9B
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x9C
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x9D
            Opcode {
                op_cycles: 5,
                op_func: CPU6502::sta_absolute_x,
            },
            // 0x9E
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0x9F
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xA0
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::ldy_imm,
            },
            // 0xA1
            Opcode {
                op_cycles: 6,
                op_func: CPU6502::lda_indirect_x,
            },
            // 0xA2
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::ldx_imm,
            },
            // 0xA3
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xA4
            Opcode {
                op_cycles: 3,
                op_func: CPU6502::ldy_zero_page,
            },
            // 0xA5
            Opcode {
                op_cycles: 3,
                op_func: CPU6502::lda_zero_page,
            },
            // 0xA6
            Opcode {
                op_cycles: 3,
                op_func: CPU6502::ldx_zero_page,
            },
            // 0xA7
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xA8
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::tay,
            },
            // 0xA9
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::lda_imm,
            },
            // 0xAA
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::tax,
            },
            // 0xAB
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xAC
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::ldy_absolute,
            },
            // 0xAD
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::lda_absolute,
            },
            // 0xAE
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::ldx_absolute,
            },
            // 0xAF
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xB0
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::bcs,
            },
            // 0xB1
            Opcode {
                op_cycles: 5,
                op_func: CPU6502::lda_indirect_y,
            },
            // 0xB2
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xB3
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xB4
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::ldy_zero_page_x,
            },
            // 0xB5
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::lda_zero_page_x,
            },
            // 0xB6
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::ldx_zero_page_y,
            },
            // 0xB7
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xB8
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::clv,
            },
            // 0xB9
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::lda_absolute_y,
            },
            // 0xBA
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::tsx,
            },
            // 0xBB
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xBC
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::ldy_absolute_x,
            },
            // 0xBD
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::lda_absolute_x,
            },
            // 0xBE
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::ldx_absolute_y,
            },
            // 0xBF
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xC0
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::cpy_imm,
            },
            // 0xC1
            Opcode {
                op_cycles: 6,
                op_func: CPU6502::cmp_indirect_x,
            },
            // 0xC2
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xC3
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xC4
            Opcode {
                op_cycles: 3,
                op_func: CPU6502::cpy_zero_page,
            },
            // 0xC5
            Opcode {
                op_cycles: 3,
                op_func: CPU6502::cmp_zero_page,
            },
            // 0xC6
            Opcode {
                op_cycles: 5,
                op_func: CPU6502::dec_zero_page,
            },
            // 0xC7
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xC8
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::iny,
            },
            // 0xC9
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::cmp_indirect_x,
            },
            // 0xCA
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::dex,
            },
            // 0xCB
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xCC
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::cpy_absolute,
            },
            // 0xCD
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::cmp_absolute,
            },
            // 0xCE
            Opcode {
                op_cycles: 6,
                op_func: CPU6502::dec_absolute,
            },
            // 0xCF
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xD0
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::bne,
            },
            // 0xD1
            Opcode {
                op_cycles: 5,
                op_func: CPU6502::cmp_indirect_y,
            },
            // 0xD2
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xD3
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xD4
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xD5
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::cmp_zero_page_x,
            },
            // 0xD6
            Opcode {
                op_cycles: 6,
                op_func: CPU6502::dec_zero_page_x,
            },
            // 0xD7
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xD8
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::cld,
            },
            // 0xD9
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::cmp_absolute_y,
            },
            // 0xDA
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xDB
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xDC
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xDD
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::cmp_absolute_x,
            },
            // 0xDE
            Opcode {
                op_cycles: 7,
                op_func: CPU6502::dec_absolute_x,
            },
            // 0xDF
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xE0
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::cpx_imm,
            },
            // 0xE1
            Opcode {
                op_cycles: 6,
                op_func: CPU6502::sbc_indirect_x,
            },
            // 0xE2
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xE3
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xE4
            Opcode {
                op_cycles: 3,
                op_func: CPU6502::cpx_zero_page,
            },
            // 0xE5
            Opcode {
                op_cycles: 3,
                op_func: CPU6502::sbc_zero_page,
            },
            // 0xE6
            Opcode {
                op_cycles: 5,
                op_func: CPU6502::inc_zero_page,
            },
            // 0xE7
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xE8
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::inx,
            },
            // 0xE9
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::sbc_imm,
            },
            // 0xEA
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xEB
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xEC
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::cpx_absolute,
            },
            // 0xED
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::sbc_absolute,
            },
            // 0xEE
            Opcode {
                op_cycles: 6,
                op_func: CPU6502::inc_absolute,
            },
            // 0xEF
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xF0
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::beq,
            },
            // 0xF1
            Opcode {
                op_cycles: 5,
                op_func: CPU6502::sbc_indirect_y,
            },
            // 0xF2
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xF3
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xF4
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xF5
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::sbc_zero_page_x,
            },
            // 0xF6
            Opcode {
                op_cycles: 6,
                op_func: CPU6502::inc_zero_page_x,
            },
            // 0xF7
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xF8
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::sed,
            },
            // 0xF9
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::sbc_absolute_y,
            },
            // 0xFA
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xFB
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xFC
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
            // 0xFD
            Opcode {
                op_cycles: 4,
                op_func: CPU6502::sbc_absolute_x,
            },
            // 0xFE
            Opcode {
                op_cycles: 7,
                op_func: CPU6502::inc_absolute_x,
            },
            // 0xFF
            Opcode {
                op_cycles: 2,
                op_func: CPU6502::nop,
            },
        ]
        }
    }

    pub fn run_instruction(self: &mut CPU6502, with_disassembly: bool) {
        let instr_byte = self.memory[self.pc as usize];
        (self.instructions[instr_byte as usize].op_func)(self, with_disassembly);
    }

    pub fn load_binary() {}

    /// Making a function for this allows us to control access to memory (to implement things like actual read-only memory sections)
    pub fn write_memory(&mut self, addr: u16, value: u8) {
        // TODO Will need to investigate which areas (if any) are actually read-only. For now, just pass it through.
        self.memory[addr as usize] = value
    }

    pub fn read_memory(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn carry_flag(&self) -> bool {
        self.psr & 1 == 1
    }

    fn zero_flag(&self) -> bool {
        self.psr & 0b00000010 > 0
    }

    fn interrupt_disable_flag(&self) -> bool {
        self.psr & 0b00000100 > 0
    }

    fn decimal_flag(&self) -> bool {
        self.psr & 0b00001000 > 0
    }

    fn break_flag(&self) -> bool {
        self.psr & 0b00010000 > 0
    }

    fn overflow_flag(&self) -> bool {
        self.psr & 0b01000000 > 0
    }

    fn negative_flag(&self) -> bool {
        self.psr & 0b10000000 > 0
    }

    fn pcl(&self) -> u8 {
        (self.pc & 0xFF) as u8
    }

    fn pch(&self) -> u8 {
        ((self.pc & 0xFF00) >> 8) as u8
    }

    fn assign_carry(&mut self, set: bool) {
        if set {
            self.psr = self.psr | 1;
        } else {
            self.psr = self.psr & 0b11111110;
        }
    }

    fn assign_zero(&mut self, set: bool) {
        if set {
            self.psr = self.psr | 2;
        } else {
            self.psr = self.psr & 0b11111101;
        }
    }

    fn assign_interrupt(&mut self, set: bool) {
        if set {
            self.psr = self.psr | 4;
        } else {
            self.psr = self.psr & 0b11111011;
        }
    }

    fn assign_decimal(&mut self, set: bool) {
        if set {
            self.psr = self.psr | 8;
        } else {
            self.psr = self.psr & 0b11110111;
        }
    }

    fn assign_overflow(&mut self, set: bool) {
        if set {
            self.psr = self.psr | 64;
        } else {
            self.psr = self.psr & 0b11011111;
        }
    }

    fn assign_negative(&mut self, set: bool) {
        if set {
            self.psr = self.psr | 128;
        } else {
            self.psr = self.psr & 0b01111111;
        }
    }

    // NOP

    /// No-op! Does nothing!
    /// 
    /// Define this explicitly for two reasons:
    /// - The opcodes require a function to execute (and this is easier and safer than allowing them to have that field empty)
    /// - This allows us to potentially slot some debugging in here to run whenever we get a NOP
    fn nop(&mut self, disassemble: bool) -> () {
        if disassemble {
            println!("{} NOP", self.pc);
        }
    }

    // Flag interactions

    fn clc(&mut self, disassemble: bool) {
        if disassemble {
            println!("{} CLC", self.pc);
        } else {
            self.assign_carry(false);
        }
        self.pc += 1;
    }

    fn sec(&mut self, disassemble: bool) {
        if disassemble {
            println!("{} SEC", self.pc);
        } else {
            self.assign_carry(true);
        }
        self.pc += 1;
    }
    
    fn cld(&mut self, disassemble: bool) {
        if disassemble {
            println!("{} CLD", self.pc);
        } else {
            self.assign_decimal(false);
        }
        self.pc += 1;
    }

    fn sed(&mut self, disassemble: bool) {
        if disassemble {
            println!("{} SED", self.pc);
        } else {
            self.assign_decimal(true);
        }
        self.pc += 1;
    }
    
    fn cli(&mut self, disassemble: bool) {
        if disassemble {
            println!("{} CLI", self.pc);
        } else {
            self.assign_interrupt(false);
        }
        self.pc += 1;
    }

    fn sei(&mut self, disassemble: bool) {
        if disassemble {
            println!("{} SEI", self.pc);
        } else {
            self.assign_interrupt(true);
        }
        self.pc += 1;
    }
    
    fn clv(&mut self, disassemble: bool) {
        if disassemble {
            println!("{} CLV", self.pc);
        } else {
            self.assign_overflow(false);
        }
        self.pc += 1;
    }

    // ARITHMETIC

    /// Addition with carry
    /// 
    /// Sets all the necessary flags; does *not* increment PC.
    fn adc(&mut self, value: u8) -> () {
        // TODO Determine if there'll be a carry
        self.accumulator = self.accumulator + value + (self.carry_flag() as u8);
        // TODO Set flags
    }

    fn adc_imm(&mut self, disassemble: bool) -> () {
        let imm = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} ADC #{}", self.pc, imm);
        } else {
            self.adc(imm);
        }
        self.pc += 2;

    }

    fn adc_zero_page(&mut self, disassemble: bool) -> () {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} ADC ${}", self.pc, addr);
        } else {
            let val = self.read_memory(addr as u16);
            self.adc(val);
        }
        self.pc += 2;
    }

    fn adc_zero_page_x(&mut self, disassemble: bool) -> () {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} ADC ${}, X", self.pc, addr);
        } else {
            let val = self.read_memory((addr + self.idx_x) as u16);
            self.adc(val);
        }
        self.pc += 2;
    }

    fn adc_absolute(&mut self, disassemble: bool) -> () {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} ADC ${}", self.pc, addr);
        } else {
            let val = self.read_memory(addr);
            self.adc(val);
        }
        self.pc += 3;
    }

    fn adc_absolute_x(&mut self, disassemble: bool) -> () {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} ADC ${}, X", self.pc, addr);
        } else {
            let val = self.read_memory(addr + self.idx_x as u16);
            self.adc(val);
        }
        self.pc += 3;
    }

    fn adc_absolute_y(&mut self, disassemble: bool) -> () {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} ADC ${}, Y", self.pc, addr);
        } else {
            let val = self.read_memory(addr + self.idx_y as u16);
            self.adc(val);
        }
        self.pc += 3;
    }

    fn adc_indirect_x(&mut self, disassemble: bool) -> () {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} ADC (${}, X)", self.pc, addr);
        } else {
            // TODO IMPL
            let val = self.read_memory((addr + self.idx_x) as u16);
            self.adc(val);
        }
        self.pc += 2;
    }

    fn adc_indirect_y(&mut self, disassemble: bool) -> () {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} ADC (${}), Y", self.pc, addr);
        } else {
            // TODO IMPL
            let val = self.read_memory((addr + self.idx_x) as u16);
            self.adc(val);
        }
        self.pc += 2;
    }

    /// Decrement memory by one
    /// 
    /// Sets the necessary flags but does *not* increment PC
    fn dec(&mut self, addr: u16) -> () {
        self.write_memory(addr, self.read_memory(addr) - 1);
        // TODO Set flags
    }

    fn dec_zero_page(&mut self, disassemble: bool) {
        let addr: u8 = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} DEC ${}", self.pc, addr);
        } else {
            self.dec(addr as u16);
        }
        self.pc += 2;
    }

    fn dec_zero_page_x(&mut self, disassemble: bool) {
        let addr: u8 = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} DEC ${}, X", self.pc, addr);
        } else {
            let effective = (addr + self.idx_x) as u16;
            self.dec(effective);
        }
        self.pc += 2;
    }

    fn dec_absolute(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} DEC ${}", self.pc, addr);
        } else {
            self.dec(addr);
        }
        self.pc += 3;
    }

    fn dec_absolute_x(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} DEC ${}, X", self.pc, addr);
        } else {
            self.dec(addr + self.idx_x as u16);
        }
        self.pc += 3;
    }

    fn dex(&mut self, disassemble: bool) {
        if disassemble {
            println!("{} DEX", self.pc);
        } else {
            self.idx_x -= 1;
        }
        self.pc += 1;
    }

    fn dey(&mut self, disassemble: bool) {
        if disassemble {
            println!("{} DEY", self.pc);
        } else {
            self.idx_y -= 1;
        }
        self.pc += 1;
    }

    /// Increment memory by one
    /// 
    /// Sets the necessary flags but does *not* increment PC
    fn inc(&mut self, addr: u16) -> () {
        self.write_memory(addr, self.read_memory(addr) + 1);
        // TODO Set flags
    }

    fn inc_zero_page(&mut self, disassemble: bool) {
        let addr: u8 = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} INC ${}", self.pc, addr);
        } else {
            self.inc(addr as u16);
        }
        self.pc += 2;
    }

    fn inc_zero_page_x(&mut self, disassemble: bool) {
        let addr: u8 = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} INC ${}, X", self.pc, addr);
        } else {
            let effective = (addr + self.idx_x) as u16;
            self.inc(effective);
        }
        self.pc += 2;
    }

    fn inc_absolute(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} INC ${}", self.pc, addr);
        } else {
            self.inc(addr);
        }
        self.pc += 3;
    }

    fn inc_absolute_x(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} INC ${}, X", self.pc, addr);
        } else {
            self.inc(addr + self.idx_x as u16);
        }
        self.pc += 3;
    }

    fn inx(&mut self, disassemble: bool) {
        if disassemble {
            println!("{} INX", self.pc);
        } else {
            self.idx_x += 1;
        }
        self.pc += 1;
    }

    fn iny(&mut self, disassemble: bool) {
        if disassemble {
            println!("{} INY", self.pc);
        } else {
            self.idx_y += 1;
        }
        self.pc += 1;
    }

    /// Subtract with borrow
    /// 
    /// Sets all the necessary flags; does *not* increment PC.
    fn sbc(&mut self, value: u8) -> () {
        // TODO Determine if there'll be a carry
        self.accumulator = self.accumulator - value - (self.carry_flag() as u8);
        // TODO Set flags
    }

    fn sbc_imm(&mut self, disassemble: bool) -> () {
        let imm = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} SBC #{}", self.pc, imm);
        } else {
            self.sbc(imm);
        }
        self.pc += 2;

    }

    fn sbc_zero_page(&mut self, disassemble: bool) -> () {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} SBC ${}", self.pc, addr);
        } else {
            let val = self.read_memory(addr as u16);
            self.sbc(val);
        }
        self.pc += 2;
    }

    fn sbc_zero_page_x(&mut self, disassemble: bool) -> () {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} SBC ${}, X", self.pc, addr);
        } else {
            let val = self.read_memory((addr + self.idx_x) as u16);
            self.sbc(val);
        }
        self.pc += 2;
    }

    fn sbc_absolute(&mut self, disassemble: bool) -> () {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} SBC ${}", self.pc, addr);
        } else {
            let val = self.read_memory(addr);
            self.sbc(val);
        }
        self.pc += 3;
    }

    fn sbc_absolute_x(&mut self, disassemble: bool) -> () {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} SBC ${}, X", self.pc, addr);
        } else {
            let val = self.read_memory(addr + self.idx_x as u16);
            self.sbc(val);
        }
        self.pc += 3;
    }

    fn sbc_absolute_y(&mut self, disassemble: bool) -> () {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} SBC ${}, Y", self.pc, addr);
        } else {
            let val = self.read_memory(addr + self.idx_y as u16);
            self.sbc(val);
        }
        self.pc += 3;
    }

    fn sbc_indirect_x(&mut self, disassemble: bool) -> () {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} SBC (${}, X)", self.pc, addr);
        } else {
            // TODO IMPL
            let val = self.read_memory((addr + self.idx_x) as u16);
            self.sbc(val);
        }
        self.pc += 2;
    }

    fn sbc_indirect_y(&mut self, disassemble: bool) -> () {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} SBC (${}), Y", self.pc, addr);
        } else {
            // TODO IMPL
            let val = self.read_memory((addr + self.idx_x) as u16);
            self.sbc(val);
        }
        self.pc += 2;
    }

    // LOGICAL


    /// Logical AND with accumulator
    /// 
    /// Sets all the necessary flags; does *not* increment PC.
    fn and(&mut self, value: u8) -> () {
        self.accumulator = self.accumulator & value;
        // TODO Set flags
    }

    fn and_imm(&mut self, disassemble: bool) -> () {
        let imm = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} AND #{}", self.pc, imm);
        } else {
            self.and(imm);
        }
        self.pc += 2;
    }

    fn and_zero_page(&mut self, disassemble: bool) -> () {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} AND ${}", self.pc, addr);
        } else {
            let val = self.read_memory(addr as u16);
            self.and(val);
        }
        self.pc += 2;
    }

    fn and_zero_page_x(&mut self, disassemble: bool) -> () {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} AND ${}, X", self.pc, addr);
        } else {
            let val = self.read_memory((addr + self.idx_x) as u16);
            self.and(val);
        }
        self.pc += 2;
    }

    fn and_absolute(&mut self, disassemble: bool) -> () {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} AND ${}", self.pc, addr);
        } else {
            let val = self.read_memory(addr);
            self.and(val);
        }
        self.pc += 3;
    }

    fn and_absolute_x(&mut self, disassemble: bool) -> () {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} AND ${}, X", self.pc, addr);
        } else {
            let val = self.read_memory(addr + self.idx_x as u16);
            self.and(val);
        }
        self.pc += 3;
    }

    fn and_absolute_y(&mut self, disassemble: bool) -> () {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} AND ${}, Y", self.pc, addr);
        } else {
            let val = self.read_memory(addr + self.idx_y as u16);
            self.and(val);
        }
        self.pc += 3;
    }

    fn and_indirect_x(&mut self, disassemble: bool) -> () {
        // TODO IMPL
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} AND (${}, X)", self.pc, addr);
        } else {
            let val = self.read_memory(addr);
            self.and(val);
        }
        self.pc += 2;
    }

    fn and_indirect_y(&mut self, disassemble: bool) -> () {
        // TODO IMPL
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} AND (${}), Y", self.pc, addr);
        } else {
            let val = self.read_memory(addr);
            self.and(val);
        }
        self.pc += 2;
    }

    /// Logical OR with accumulator
    /// 
    /// Sets the necessary flags but does *not* increment PC
    fn ora(&mut self, value: u8) {
        self.accumulator = self.accumulator | value;
        // TODO Set flags
    }

    fn ora_imm(&mut self, disassemble: bool) {
        let imm = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} ORA #{}", self.pc, imm);
        } else {
            self.ora(imm);
        }
        self.pc += 2;
    }

    fn ora_zero_page(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} ORA ${}", self.pc, addr);
        } else {
            let val = self.read_memory(addr as u16);
            self.ora(val);
        }
        self.pc += 2;
    }

    fn ora_zero_page_x(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} ORA ${}, X", self.pc, addr);
        } else {
            let val = self.read_memory((addr + self.idx_x) as u16);
            self.ora(val);
        }
        self.pc += 2;
    }

    fn ora_absolute(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} ORA ${}", self.pc, addr);
        } else {
            let val = self.read_memory(addr);
            self.ora(val);
        }
        self.pc += 3;
    }

    fn ora_absolute_x(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} ORA ${}, X", self.pc, addr);
        } else {
            let val = self.read_memory(addr + self.idx_x as u16);
            self.ora(val);
        }
        self.pc += 3;
    }

    fn ora_absolute_y(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} ORA ${}, Y", self.pc, addr);
        } else {
            let val = self.read_memory(addr + self.idx_y as u16);
            self.ora(val);
        }
        self.pc += 3;
    }

    fn ora_indirect_x(&mut self, disassemble: bool) {
        // TODO IMPL
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} ORA (${}, X)", self.pc, addr);
        } else {
            let val = self.read_memory(addr as u16);
            self.ora(val);
        }
        self.pc += 2;
    }

    fn ora_indirect_y(&mut self, disassemble: bool) {
        // TODO IMPL
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} ORA (${}), Y", self.pc, addr);
        } else {
            let val = self.read_memory(addr as u16);
            self.ora(val);
        }
        self.pc += 2;
    }

    /// Exclusive OR with accumulator
    /// 
    /// Sets the necessary flags but does *not* increment PC
    fn eor(&mut self, value: u8) {
        self.accumulator = self.accumulator ^ value;
        // TODO Set flags
    }

    fn eor_imm(&mut self, disassemble: bool) {
        let imm = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} EOR #{}", self.pc, imm);
        } else {
            self.eor(imm);
        }
        self.pc += 2;
    }

    fn eor_zero_page(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} EOR ${}", self.pc, addr);
        } else {
            let val = self.read_memory(addr as u16);
            self.eor(val);
        }
        self.pc += 2;
    }

    fn eor_zero_page_x(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} EOR ${}, X", self.pc, addr);
        } else {
            let val = self.read_memory((addr + self.idx_x) as u16);
            self.eor(val);
        }
        self.pc += 2;
    }

    fn eor_absolute(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} EOR ${}", self.pc, addr);
        } else {
            let val = self.read_memory(addr);
            self.eor(val);
        }
        self.pc += 3;
    }

    fn eor_absolute_x(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} EOR ${}, X", self.pc, addr);
        } else {
            let val = self.read_memory(addr + self.idx_x as u16);
            self.eor(val);
        }
        self.pc += 3;
    }

    fn eor_absolute_y(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} EOR ${}, Y", self.pc, addr);
        } else {
            let val = self.read_memory(addr + self.idx_y as u16);
            self.eor(val);
        }
        self.pc += 3;
    }

    fn eor_indirect_x(&mut self, disassemble: bool) {
        // TODO IMPL
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} EOR (${}, X)", self.pc, addr);
        } else {
            let val = self.read_memory(addr as u16);
            self.eor(val);
        }
        self.pc += 2;
    }

    fn eor_indirect_y(&mut self, disassemble: bool) {
        // TODO IMPL
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} EOR (${}), Y", self.pc, addr);
        } else {
            let val = self.read_memory(addr as u16);
            self.eor(val);
        }
        self.pc += 2;
    }


    // Interrupts etc

    fn brk(&mut self, disassemble: bool) {
        if disassemble {
            println!("{} BRK", self.pc);
        } else {
            // TODO IMPL
        }
        self.pc += 1;
    }

    fn bit(&mut self, value: u8) {
        let res = self.accumulator & value;
        self.assign_negative((res & 128) > 0);
        self.assign_overflow((res & 64) > 0);
        self.assign_zero(res == 0);
    }

    fn bit_zero_page(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} BIT ${}", self.pc, addr);
        } else {
            self.bit(self.read_memory(addr as u16));
        }
        self.pc += 2;
    }

    fn bit_absolute(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} BIT ${}", self.pc, addr);
        } else {
            self.bit(self.read_memory(addr));
        }
        self.pc += 3;
    }

    // Comparisons

    /// Comapare memory with accumulator
    /// 
    /// Sets the necessary flags but does *not* increment PC
    fn cmp(&mut self, value: u8) {
        let res = self.accumulator - value;
        // TODO Set flags
    }

    fn cmp_imm(&mut self, disassemble: bool) {
        let imm = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} CMP #{}", self.pc, imm);
        } else {
            self.cmp(imm);
        }
        self.pc += 2;
    }

    fn cmp_zero_page(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} CMP ${}", self.pc, addr);
        } else {
            let val = self.read_memory(addr as u16);
            self.cmp(val);
        }
        self.pc += 2;
    }

    fn cmp_zero_page_x(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} CMP ${}, X", self.pc, addr);
        } else {
            let val = self.read_memory((addr + self.idx_x) as u16);
            self.cmp(val);
        }
        self.pc += 2;
    }

    fn cmp_absolute(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} CMP ${}", self.pc, addr);
        } else {
            let val = self.read_memory(addr);
            self.cmp(val);
        }
        self.pc += 3;
    }

    fn cmp_absolute_x(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} CMP ${}, X", self.pc, addr);
        } else {
            let val = self.read_memory(addr + self.idx_x as u16);
            self.cmp(val);
        }
        self.pc += 3;
    }

    fn cmp_absolute_y(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} CMP ${}, Y", self.pc, addr);
        } else {
            let val = self.read_memory(addr + self.idx_y as u16);
            self.cmp(val);
        }
        self.pc += 3;
    }

    fn cmp_indirect_x(&mut self, disassemble: bool) {
        // TODO IMPL
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} CMP (${}, X)", self.pc, addr);
        } else {
            let val = self.read_memory(addr as u16);
            self.cmp(val);
        }
        self.pc += 2;
    }

    fn cmp_indirect_y(&mut self, disassemble: bool) {
        // TODO IMPL
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} CMP (${}), Y", self.pc, addr);
        } else {
            let val = self.read_memory(addr as u16);
            self.cmp(val);
        }
        self.pc += 2;
    }

    /// Compare memory with index X
    /// 
    /// Sets the necessary flags but does *not* increment PC
    fn cpx(&mut self, value: u8) {
        let res = self.accumulator - value;
        // TODO Set flags
    }

    fn cpx_imm(&mut self, disassemble: bool) {
        let imm = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} CPX #{}", self.pc, imm);
        } else {
            self.cpx(imm);
        }
        self.pc += 2;
    }

    fn cpx_zero_page(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} CPX ${}", self.pc, addr);
        } else {
            let val = self.read_memory(addr as u16);
            self.cpx(val);
        }
        self.pc += 2;
    }

    fn cpx_absolute(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} CPX ${}", self.pc, addr);
        } else {
            let val = self.read_memory(addr as u16);
            self.cpx(val);
        }
        self.pc += 3;
    }

    /// Compare memory with index Y
    /// 
    /// Sets the necessary flags but does *not* increment PC
    fn cpy(&mut self, value: u8) {
        let res = self.accumulator - value;
        // TODO Set flags
    }

    fn cpy_imm(&mut self, disassemble: bool) {
        let imm = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} CPY #{}", self.pc, imm);
        } else {
            self.cpy(imm);
        }
        self.pc += 2;
    }

    fn cpy_zero_page(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} CPY ${}", self.pc, addr);
        } else {
            let val = self.read_memory(addr as u16);
            self.cpy(val);
        }
        self.pc += 2;
    }

    fn cpy_absolute(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} CPY ${}", self.pc, addr);
        } else {
            let val = self.read_memory(addr);
            self.cpy(val);
        }
        self.pc += 3;
    }

    // Stack Ops

    fn pha(&mut self, disassemble: bool) {
        if disassemble {
            println!("{} PHA", self.pc);
        }
        // TODO IMPL
        self.pc += 1;
    }

    fn php(&mut self, disassemble: bool) {
        if disassemble {
            println!("{} PHP", self.pc);
        }
        // TODO IMPL
        self.pc += 1;
    }

    fn pla(&mut self, disassemble: bool) {
        if disassemble {
            println!("{} PLA", self.pc);
        }
        // TODO IMPL
        self.pc += 1;
    }

    fn plp(&mut self, disassemble: bool) {
        if disassemble {
            println!("{} PLP", self.pc);
        }
        // TODO IMPL
        self.pc += 1;
    }

    // Shifts

    fn asl_a(&mut self, disassemble: bool) {
        if disassemble {
            println!("{} ASL A", self.pc);
        } else {
            self.assign_carry(self.accumulator & 0b10000000 > 0);
            self.accumulator = self.accumulator << 1;
        }
        self.pc += 1;
    }

    fn asl_zero_page(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} ASL ${}", self.pc, addr);
        } else {
            let effective = addr as u16;
            self.assign_carry(self.read_memory(effective) & 0b10000000 > 0);
            self.write_memory(effective, self.read_memory(effective) << 1);
        }
        self.pc += 2;
    }

    fn asl_zero_page_x(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} ASL ${}, X", self.pc, addr);
        } else {
            let effective = (addr + self.idx_x) as u16;
            self.assign_carry(self.read_memory(effective) & 0b10000000 > 0);
            self.write_memory(effective, self.read_memory(effective) << 1);
        }
        self.pc += 2;
    }

    fn asl_absolute(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} ASL ${}", self.pc, addr);
        } else {
            self.assign_carry(self.read_memory(addr) & 0b10000000 > 0);
            self.write_memory(addr, self.read_memory(addr) << 1);
        }
        self.pc += 3;
    }

    fn asl_absolute_x(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} ASL ${}, X", self.pc, addr);
        } else {
            let effective = addr + self.idx_x as u16;
            self.assign_carry(self.read_memory(effective) & 0b10000000 > 0);
            self.write_memory(effective, self.read_memory(effective) << 1);
        }
        self.pc += 3;
    }

    fn lsr_a(&mut self, disassemble: bool) {
        if disassemble {
            println!("{} LSR A", self.pc);
        } else {
            if self.accumulator & 1 > 0 {
                self.assign_carry(true);
            }
            self.accumulator = self.accumulator >> 1;
        }
        self.pc += 1;
    }

    fn lsr_zero_page(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} LSR ${}", self.pc, addr);
        } else {
            let effective = addr as u16;
            if self.read_memory(effective) & 1 > 0 {
                self.assign_carry(true);
            }
            self.write_memory(effective, self.read_memory(effective) >> 1);
        }
        self.pc += 2;
    }

    fn lsr_zero_page_x(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} LSR ${}, X", self.pc, addr);
        } else {
            let effective = (addr + self.idx_x) as u16;
            if self.read_memory(effective) & 1 > 0 {
                self.assign_carry(true);
            }
            self.write_memory(effective, self.read_memory(effective) >> 1);
        }
        self.pc += 2;
    }

    fn lsr_absolute(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} LSR ${}", self.pc, addr);
        } else {
            if self.read_memory(addr) & 1 > 0 {
                self.assign_carry(true);
            }
            self.write_memory(addr, self.read_memory(addr) >> 1);
        }
        self.pc += 3;
    }

    fn lsr_absolute_x(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} LSR ${}, X", self.pc, addr);
        } else {
            let effective = addr + self.idx_x as u16;
            if self.read_memory(effective) & 1 > 0 {
                self.assign_carry(true);
            }
            self.write_memory(effective, self.read_memory(effective) >> 1);
        }
        self.pc += 3;
    }

    fn rol_a(&mut self, disassemble: bool) {
        if disassemble {
            println!("{} ROL A", self.pc);
        } else {
            let should_set_carry = self.accumulator & 1 > 0;
            self.accumulator = self.accumulator << 1;
            if self.carry_flag() {
                self.accumulator = self.accumulator ^ 1;
            }
            if should_set_carry {
                self.assign_carry(true);
            }
        }
        self.pc += 1;
    }

    fn rol_zero_page(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} ROL ${}", self.pc, addr);
        } else {
            let effective = addr as u16;
            let should_set_carry = self.read_memory(effective) & 1 > 0;
            self.write_memory(effective, self.read_memory(effective) << 1);
            if self.carry_flag() {
                self.write_memory(effective, self.read_memory(effective) ^ 1);
            }
            if should_set_carry {
                self.assign_carry(true);
            }
        }
        self.pc += 2;
    }

    fn rol_zero_page_x(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} ROL ${}, X", self.pc, addr);
        } else {
            let effective = (addr + self.idx_x) as u16;
            let should_set_carry = self.read_memory(effective) & 1 > 0;
            self.write_memory(effective, self.read_memory(effective) << 1);
            if self.carry_flag() {
                self.write_memory(effective, self.read_memory(effective) ^ 1);
            }
            if should_set_carry {
                self.assign_carry(true);
            }
        }
        self.pc += 2;
    }

    fn rol_absolute(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} ROL ${}", self.pc, addr);
        } else {
            let should_set_carry = self.read_memory(addr) & 1 > 0;
            self.write_memory(addr, self.read_memory(addr) << 1);
            if self.carry_flag() {
                self.write_memory(addr, self.read_memory(addr) ^ 1);
            }
            if should_set_carry {
                self.assign_carry(true);
            }
        }
        self.pc += 3;
    }

    fn rol_absolute_x(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} ROL ${}, X", self.pc, addr);
        } else {
            let effective = addr + self.idx_x as u16;
            let should_set_carry = self.read_memory(effective as u16) & 1 > 0;
            self.write_memory(effective, self.read_memory(effective) << 1);
            if self.carry_flag() {
                self.write_memory(effective, self.read_memory(effective) ^ 1);
            }
            if should_set_carry {
                self.assign_carry(true);
            }
        }
        self.pc += 3;
    }

    fn ror_a(&mut self, disassemble: bool) {
        if disassemble {
            println!("{} ROR A", self.pc);
        } else {
            let should_set_carry = self.accumulator & 1 > 0;
            self.accumulator = self.accumulator >> 1;
            if self.carry_flag() {
                self.accumulator = self.accumulator ^ 0b10000000;
            }
            if should_set_carry {
                self.assign_carry(true);
            }
        }
        self.pc += 1;
    }

    fn ror_zero_page(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} ROR ${}", self.pc, addr);
        } else {
            let should_set_carry = self.read_memory(addr as u16) & 1 > 0;
            self.write_memory(addr as u16, self.read_memory(addr as u16) >> 1);
            if self.carry_flag() {
                self.write_memory(addr as u16, self.read_memory(addr as u16) ^ 0b10000000);
            }
            if should_set_carry {
                self.assign_carry(true);
            }
        }
        self.pc += 2;
    }

    fn ror_zero_page_x(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} ROR ${}, X", self.pc, addr);
        } else {
            let effective = (addr + self.idx_x) as u16;
            let should_set_carry = self.read_memory(effective) & 1 > 0;
            self.write_memory(effective, self.read_memory(effective) >> 1);
            if self.carry_flag() {
                self.write_memory(effective, self.read_memory(effective) ^ 0b10000000);
            }
            if should_set_carry {
                self.assign_carry(true);
            }
        }
        self.pc += 2;
    }

    fn ror_absolute(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} ROR ${}", self.pc, addr);
        } else {
            let effective = addr as u16;
            let should_set_carry = self.read_memory(effective) & 1 > 0;
            self.write_memory(effective, self.read_memory(effective) >> 1);
            if self.carry_flag() {
                self.write_memory(effective, self.read_memory(effective) ^ 0b10000000);
            }
            if should_set_carry {
                self.assign_carry(true);
            }
        }
        self.pc += 3;
    }

    fn ror_absolute_x(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} ROR ${}, X", self.pc, addr);
        } else {
            let effective = addr + self.idx_x as u16;
            let should_set_carry = self.read_memory(effective) & 1 > 0;
            self.write_memory(effective, self.read_memory(effective) >> 1);
            if self.carry_flag() {
                self.write_memory(effective, self.read_memory(effective) ^ 0b10000000);
            }
            if should_set_carry {
                self.assign_carry(true);
            }
        }
        self.pc += 3;
    }

    // Branches/Jumps
    fn branch(&mut self, relative_addr: u8) {
        // TODO IMPL
    }

    fn bcc(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} BCC ${}", self.pc, addr);
            self.pc += 2;
        } else if !self.carry_flag() {
            self.branch(addr);
        }
    }

    fn bcs(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} BCS ${}", self.pc, addr);
            self.pc += 2;
        } else if self.carry_flag() {
            self.branch(addr);
        }
    }

    fn beq(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} BEQ ${}", self.pc, addr);
            self.pc += 2;
        } else if self.zero_flag() {
            self.branch(addr);
        }
    }

    fn bmi(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} BMI ${}", self.pc, addr);
            self.pc += 2;
        } else if self.negative_flag() {
            self.branch(addr);
        }
    }

    fn bne(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} BNE ${}", self.pc, addr);
            self.pc += 2;
        } else if !self.zero_flag() {
            self.branch(addr);
        }
    }

    fn bpl(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} BPL ${}", self.pc, addr);
            self.pc += 2;
        } else if !self.negative_flag() {
            self.branch(addr);
        }
    }

    fn bvc(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} BVC ${}", self.pc, addr);
            self.pc += 2;
        } else if !self.overflow_flag() {
            self.branch(addr);
        }
    }

    fn bvs(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc + 1);
        if disassemble {
            println!("{} BVS ${}", self.pc, addr);
            self.pc += 2;
        } else if self.overflow_flag() {
            self.branch(addr);
        }
    }

    fn jmp(&mut self, addr: u16) {
        self.pc = addr;
    }

    fn jmp_absolute(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} JMP ${}", self.pc, addr);
            self.pc += 3;
        } else {
            self.jmp(addr);
        }
    }

    fn jmp_indirect(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let indirected = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} JMP (${})", self.pc, indirected);
            self.pc += 3;
        } else {
            let addr_low = self.read_memory(indirected);
            let addr_high = self.read_memory(indirected+1);
            let addr = double_byte_from_two_single_bytes(addr_low, addr_high);
            self.jmp(addr);
        }
    }

    fn jsr(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} JSR ${}", self.pc, addr);
            self.pc += 3;
        } else {
            // TODO IMPL (save PC+2 onto stack)
            self.jmp(addr);
        }
    }

    fn rti(&mut self, disassemble: bool) {
        if disassemble {
            println!("{} RTI", self.pc);
            self.pc += 1;
        } else {
            // TODO IMPL (Pull PSR and PC from stack)
        }
    }

    fn rts(&mut self, disassemble: bool) {
        if disassemble {
            println!("{} RTS", self.pc);
        } else {
            // TODO IMPL (Pull PC from stack - the increment by one belowthen puts PC where it should be)
        }
        self.pc += 1;
    }

    // Load/Store

    /// Loads the given value into the accumulator.
    /// 
    /// Sets flags but does *not* set PC.
    fn lda(&mut self, value: u8) {
        // TODO Set flags
        self.accumulator = value;
    }

    /// Loads the given value into the X index register.
    /// 
    /// Sets flags but does *not* set PC.
    fn ldx(&mut self, value: u8) {
        // TODO Set flags
        self.idx_x = value;
    }

    /// Loads the given value into the Y index register.
    /// 
    /// Sets flags but does *not* set PC.
    fn ldy(&mut self, value: u8) {
        // TODO Set flags
        self.idx_y = value;
    }

    /// Stores accumulator in addr
    /// 
    /// Does *not* increment PC.
    fn sta(&mut self, addr: u16) {
        self.write_memory(addr, self.accumulator);
    }

    /// Stores X index register in addr
    /// 
    /// Does *not* increment PC.
    fn stx(&mut self, addr: u16) {
        self.write_memory(addr, self.idx_x);
    }

    /// Stores Y index register in addr
    /// 
    /// Does *not* increment PC.
    fn sty(&mut self, addr: u16) {
        self.write_memory(addr, self.idx_y);
    }

    fn lda_imm(&mut self, disassemble: bool) {
        let imm = self.read_memory(self.pc+1);
        if disassemble {
            println!("{} LDA #{}", self.pc, imm);
        } else {
            self.lda(imm);
        }
        self.pc += 2;
    }

    fn lda_zero_page(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc+1);
        if disassemble {
            println!("{} LDA ${}", self.pc, addr);
        } else {
            let value = self.read_memory(addr as u16);
            self.lda(value);
        }
        self.pc += 2;
    }

    fn lda_zero_page_x(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc+1);
        if disassemble {
            println!("{} LDA ${}, X", self.pc, addr);
        } else {
            // TODO Check Impl
            let value = self.read_memory((addr + self.idx_x) as u16);
            self.lda(value);
        }
        self.pc += 2;
    }

    fn lda_absolute(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} LDA ${}", self.pc, addr);
        } else {
            self.lda(self.read_memory(addr));
        }
        self.pc += 3;
    }

    fn lda_absolute_x(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} LDA ${}, X", self.pc, addr);
        } else {
            self.lda(self.read_memory(addr + self.idx_x as u16));
        }
        self.pc += 3;
    }

    fn lda_absolute_y(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} LDA ${}, X", self.pc, addr);
        } else {
            self.lda(self.read_memory(addr + self.idx_y as u16));
        }
        self.pc += 3;
    }

    fn lda_indirect_x(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc+1);
        if disassemble {
            println!("{} LDA (${}, X)", self.pc, addr);
        } else {
            // TODO IMPL
        }
        self.pc += 2;
    }

    fn lda_indirect_y(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc+1);
        if disassemble {
            println!("{} LDA (${}), Y", self.pc, addr);
        } else {
            // TODO IMPL
        }
        self.pc += 2;
    }

    fn ldx_imm(&mut self, disassemble: bool) {
        let imm = self.read_memory(self.pc+1);
        if disassemble {
            println!("{} LDX #{}", self.pc, imm);
        } else {
            self.ldx(imm);
        }
        self.pc += 2;
    }

    fn ldx_zero_page(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc+1);
        if disassemble {
            println!("{} LDX ${}", self.pc, addr);
        } else {
            let value = self.read_memory(addr as u16);
            self.ldx(value);
        }
        self.pc += 2;
    }

    fn ldx_zero_page_y(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc+1);
        if disassemble {
            println!("{} LDX ${}, Y", self.pc, addr);
        } else {
            // TODO Check Impl
        let value = self.read_memory((addr + self.idx_y) as u16);
            self.ldx(value);
        }
        self.pc += 2;
    }

    fn ldx_absolute(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} LDX ${}", self.pc, addr);
        } else {
            self.ldx(self.read_memory(addr));
        }
        self.pc += 3;
    }

    fn ldx_absolute_y(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} LDX ${}, Y", self.pc, addr);
        } else {
            self.ldx(self.read_memory(addr + self.idx_y as u16));
        }
        self.pc += 3;
    }

    fn ldy_imm(&mut self, disassemble: bool) {
        let imm = self.read_memory(self.pc+1);
        if disassemble {
            println!("{} LDY #{}", self.pc, imm);
        } else {
            self.ldy(imm);
        }
        self.pc += 2;
    }

    fn ldy_zero_page(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc+1);
        if disassemble {
            println!("{} LDY ${}", self.pc, addr);
        } else {
            let value = self.read_memory(addr as u16);
            self.ldy(value);
        }
        self.pc += 2;
    }

    fn ldy_zero_page_x(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc+1);
        if disassemble {
            println!("{} LDY ${}, Y", self.pc, addr);
        } else {
            // TODO Check Impl
            let value = self.read_memory((addr + self.idx_x) as u16);
            self.ldy(value);
        }
        self.pc += 2;
    }

    fn ldy_absolute(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} LDY ${}", self.pc, addr);
        } else {
            self.ldy(self.read_memory(addr));
        }
        self.pc += 3;
    }

    fn ldy_absolute_x(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} LDY ${}, X", self.pc, addr);
        } else {
            self.ldy(self.read_memory(addr + self.idx_x as u16));
        }
        self.pc += 3;
    }

    fn sta_zero_page(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc+1);
        if disassemble {
            println!("{} STA ${}", self.pc, addr);
        } else {
            self.sta(addr as u16);
        }
        self.pc += 2;
    }

    fn sta_zero_page_x(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc+1);
        if disassemble {
            println!("{} STA ${}, X", self.pc, addr);
        } else {
            // TODO Check Impl
            self.sta((addr + self.idx_x) as u16);
        }
        self.pc += 2;
    }

    fn sta_absolute(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} STA ${}", self.pc, addr);
        } else {
            self.sta(addr);
        }
        self.pc += 3;
    }

    fn sta_absolute_x(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} STA ${}, X", self.pc, addr);
        } else {
            // TODO Check impl
            self.sta(addr + self.idx_x as u16);
        }
        self.pc += 3;
    }

    fn sta_absolute_y(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} STA ${}, X", self.pc, addr);
        } else {
            // TODO Check impl
            self.sta(addr + self.idx_y as u16);
        }
        self.pc += 3;
    }

    fn sta_indirect_x(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc+1);
        if disassemble {
            println!("{} STA (${}, X)", self.pc, addr);
        } else {
            // TODO IMPL
        }
        self.pc += 2;
    }

    fn sta_indirect_y(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc+1);
        if disassemble {
            println!("{} STA (${}), Y", self.pc, addr);
        } else {
            // TODO IMPL
        }
        self.pc += 2;
    }

    fn stx_zero_page(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc+1);
        if disassemble {
            println!("{} STX ${}", self.pc, addr);
        } else {
            self.stx(addr as u16);
        }
        self.pc += 2;
    }

    fn stx_zero_page_y(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc+1);
        if disassemble {
            println!("{} STX ${}, Y", self.pc, addr);
        } else {
            // TODO Check Impl
            self.stx((addr + self.idx_y) as u16);
        }
        self.pc += 2;
    }

    fn stx_absolute(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} STX ${}", self.pc, addr);
        } else {
            self.stx(addr);
        }
        self.pc += 3;
    }

    fn sty_zero_page(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc+1);
        if disassemble {
            println!("{} STY ${}", self.pc, addr);
        } else {
            self.sty(addr as u16);
        }
        self.pc += 2;
    }

    fn sty_zero_page_x(&mut self, disassemble: bool) {
        let addr = self.read_memory(self.pc+1);
        if disassemble {
            println!("{} STY ${}, X", self.pc, addr);
        } else {
            // TODO Check Impl
            self.sty((addr + self.idx_x) as u16);
        }
        self.pc += 2;
    }

    fn sty_absolute(&mut self, disassemble: bool) {
        let low = self.read_memory(self.pc + 1);
        let high = self.read_memory(self.pc + 2);
        let addr = double_byte_from_two_single_bytes(low, high);
        if disassemble {
            println!("{} STY ${}", self.pc, addr);
        } else {
            self.sty(addr);
        }
        self.pc += 3;
    }

    fn tax(&mut self, disassemble: bool) {
        if disassemble {
            println!("{} TAX", self.pc);
        } else {
            self.idx_x = self.accumulator;
        }
        self.pc += 1;
    }

    fn tay(&mut self, disassemble: bool) {
        if disassemble {
            println!("{} TAY", self.pc);
        } else {
            self.idx_y = self.accumulator;
        }
        self.pc += 1;
    }

    fn tya(&mut self, disassemble: bool) {
        if disassemble {
            println!("{} TYA", self.pc);
        } else {
            self.accumulator = self.idx_y;
        }
        self.pc += 1;
    }

    fn tsx(&mut self, disassemble: bool) {
        if disassemble {
            println!("{} TSX", self.pc);
        } else {
            self.idx_x = self.sp;
        }
        self.pc += 1;
    }

    fn txa(&mut self, disassemble: bool) {
        if disassemble {
            println!("{} TXA", self.pc);
        } else {
            self.accumulator = self.idx_x;
        }
        self.pc += 1;
    }

    fn txs(&mut self, disassemble: bool) {
        if disassemble {
            println!("{} TXS", self.pc);
        } else {
            self.sp = self.idx_x;
        }
        self.pc += 1;
    }

}

/// Concatenates two single-byte values into one 16-bit value. Usually used for constructing memory addresses.
///
/// Note the order of the low and high bytes.
fn double_byte_from_two_single_bytes(low_byte: u8, high_byte: u8) -> u16 {
    let mut result = low_byte as u16;
    result = result & (high_byte as u16) << 8;
    result
}