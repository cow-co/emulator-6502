use crate::machine_layer;

pub fn run() {
    let machine = machine_layer::Machine2600::new();
    loop {
        machine.run_machine();
    }
}