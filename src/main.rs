use std::process;

use crate::logger::configure_logger;
mod logger;
mod run;

#[macro_use]
extern crate log;

fn main() {
    configure_logger();
    debug!("Application is starting");

    if let Err(e) = run::run() {
        error!("Application error: {}", e);

        process::exit(1);
    }
    debug!("Application ran successfully");
}
