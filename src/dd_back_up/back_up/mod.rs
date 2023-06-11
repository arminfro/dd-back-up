use clap::Args;

#[derive(Args, Debug)]
pub struct BackUpArgs {}

pub fn run(back_up_args: &BackUpArgs) -> Result<(), String> {
    eprintln!("DEBUGPRINT[1]: mod.rs:8: back_up_args={:#?}", back_up_args);
    Ok(())
}
