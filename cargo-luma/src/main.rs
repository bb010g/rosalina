use clap::Parser;
use std::process::ExitCode;

mod args;
mod build;

fn main() -> anyhow::Result<ExitCode> {
    let args::Cargo::Luma(args) = args::Cargo::parse();
    match args.subcommand {
        args::CargoLumaSubcommand::Build(build_args) => build::handle_build(build_args),
    }
}
