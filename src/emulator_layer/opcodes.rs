
pub struct Opcode {
    // TODO Maybe have addressing-type as a field here? Would that actually help at all though?
    op_bytes: u8,
    op_cycles: u8,      // Important in allowing us to run the emulator at the correct speed
    mnemonic: str,   // For disassembling
}