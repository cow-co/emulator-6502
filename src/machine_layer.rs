use crate::emulator_layer;
pub struct Machine2600 {
    processor: emulator_layer::Emulator6502
}

impl Machine2600 {
    pub fn new() -> Self {
        Self {
            processor: emulator_layer::Emulator6502::new(),
        }
    }

    /// Loads the given file into memory at the location the machine requires (which is generally machine-dependent)
    pub fn load_rom(&self, rom_file: String) -> () {
        
    }

    pub fn run_machine(&self) {
        self.processor.run_emulation();
    }

    pub fn frame_buffer(&self) {
        
    }
}

