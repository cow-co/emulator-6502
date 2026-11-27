use crate::machine_layer::MachineMissileCommand;

pub fn run(rom_file: &str) {
    let mut machine = MachineMissileCommand::new();
    machine.load_rom(rom_file, 0);
    loop {
        machine.run_machine(true);
    }
}