use std::process;
mod dd_backup;

fn main() {
    if let Err(e) = dd_backup::run() {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
