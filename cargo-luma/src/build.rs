use crate::args;
use std::process::ExitCode;

pub fn handle_build(_args: args::CargoLumaBuild) -> anyhow::Result<ExitCode> {
    println!("Hello, World!");

    Ok(ExitCode::SUCCESS)
}
