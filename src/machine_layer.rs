use crate::emulator_layer::CPU6502;
use std::fs::read;

pub struct MachineMissileCommand {
    processor: CPU6502,
    rom_end_addr: usize,
}

impl MachineMissileCommand {
    pub fn new() -> Self {
        Self {
            processor: CPU6502::new(),
            rom_end_addr: 0,
        }
    }

    /// Loads the given file into memory at the location the machine requires (which is generally machine-dependent)
    pub fn load_rom(&mut self, rom_file: &str, start_addr: u16) -> () {
        println!("Loading ROM...");
        let bytes = read(rom_file).expect("ROM File could not be read!");
        self.rom_end_addr = self.processor.bulk_write_memory(start_addr, &bytes);
        self.processor.set_execution_address(start_addr);
        println!("{} bytes of ROM loaded, starting at address {}...", bytes.len(), start_addr);
    }

    pub fn run_machine(&mut self, with_disassembly: bool) {
        println!("Running processor until address {}...", self.rom_end_addr);
        while !self.processor.execution_complete() {
            self.processor.run_instruction(with_disassembly);
        }
    }

    pub fn frame_buffer(&self) {
        
    }
}