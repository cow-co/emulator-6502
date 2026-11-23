use crate::machine_layer::machine_state::Machine2600;

pub fn run() {
    let machine = Machine2600::new();
    loop {
        machine.run_machine();
    }
}