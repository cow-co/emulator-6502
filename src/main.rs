mod host_layer;
mod machine_layer;
mod emulator_layer;

use std::env;

fn main() {
    let cli_args: Vec<String> = env::args().collect();
    println!("Running emulation...");
    if cli_args.len() > 1 {
        let file: &str = cli_args[1].as_str();
        host_layer::run(file);
    } else {
        println!("You should enter a path to a ROM file!");
    }
}
