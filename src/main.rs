use std::process;
mod dd_back_up;

fn main() {
    if let Err(e) = dd_back_up::run() {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
