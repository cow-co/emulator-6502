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
    pub fn run_machine(&self) {
        self.processor.run_emulation();
    }
}

