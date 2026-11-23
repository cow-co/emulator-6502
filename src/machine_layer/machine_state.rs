use crate::emulator_layer::cpu_state::Emulator6502;
pub struct Machine2600 {
    processor: Emulator6502
}

impl Machine2600 {
    pub fn new() -> Self {
        Self {
            processor: Emulator6502::new(),
        }
    }

    /// Loads the given file into memory at the location the machine requires (which is generally machine-dependent)
    pub fn load_rom(&self, rom_file: String) -> () {
        
    }

    pub fn run_machine(&self, with_disassembly: bool) {
        self.processor.run_emulation(with_disassembly);
    }

    pub fn frame_buffer(&self) {
        
    }
}