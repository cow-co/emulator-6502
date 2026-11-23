use crate::emulator_layer::cpu_state::Emulator6502;

pub mod cpu_state;
mod instructions;



pub fn run_emulation(state: &mut Emulator6502, with_disassembly: bool) {
    let instr_byte = state.memory[self.pc as usize];
    (self.instructions[instr_byte as usize].op_func)(self, with_disassembly);
}