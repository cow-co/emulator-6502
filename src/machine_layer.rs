use crate::emulator_layer::CPU6502;
use std::fs::read;

pub struct MachineMissileCommand {
    processor: CPU6502
}

impl MachineMissileCommand {
    pub fn new() -> Self {
        Self {
            processor: CPU6502::new(),
        }
    }

    /// Loads the given file into memory at the location the machine requires (which is generally machine-dependent)
    pub fn load_rom(&mut self, rom_file: &str, start_addr: u16) -> () {
        let bytes = read(rom_file).expect("ROM File could not be read!");
        let mut idx: u16 = 0;
        for b in bytes.iter() {
            self.processor.write_memory(start_addr+idx, *b);
            idx += 1;
        }
        self.processor.set_execution_address(start_addr);
    }

    pub fn run_machine(&mut self, with_disassembly: bool) {
        self.processor.run_instruction(with_disassembly);
    }

    pub fn frame_buffer(&self) {
        
    }
}