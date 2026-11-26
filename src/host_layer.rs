use crate::machine_layer::MachineMissileCommand;

pub fn run() {
    let mut machine = MachineMissileCommand::new();
    machine.load_rom("cpu_diag.bin", 0);
    loop {
        machine.run_machine(true);
    }
}