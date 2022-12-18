use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "cargo", bin_name = "cargo")]
pub enum Cargo {
    Luma(CargoLuma),
}

#[derive(Args, Debug)]
#[clap(author, version, about)]
pub struct CargoLuma {
    #[clap(subcommand)]
    pub subcommand: CargoLumaSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum CargoLumaSubcommand {
    Build(CargoLumaBuild),
}

#[derive(Args, Debug)]
pub struct CargoLumaBuild {
    /// Build artifacts in release mode, with optimizations
    #[clap(short, long)]
    pub release: bool,
    /// Comma separated list of features to activate
    #[clap(short = 'F', long, value_delimiter = ',')]
    pub features: Vec<String>,
}
