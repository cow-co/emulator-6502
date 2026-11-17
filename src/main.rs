mod host_layer;
mod machine_layer;
mod emulator_layer;

fn main() {
    println!("Running emulation...");
    host_layer::run();
}
