use clap::Parser;
use polkadot_sdk::sc_cli::{SubstrateCli, ChainSpec};
mod chain_spec;
mod service;

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,
    #[clap(flatten)]
    pub run: polkadot_sdk::sc_cli::RunCmd,
}

#[derive(Parser, Debug)]
pub enum Subcommand {
    PurgeChain(polkadot_sdk::sc_cli::PurgeChainCmd),
}

impl SubstrateCli for Cli {
    fn impl_name() -> String { "Moral Money Node".into() }
    fn impl_version() -> String { env!("CARGO_PKG_VERSION").into() }
    fn description() -> String { "Blockchain MMECO".into() }
    fn author() -> String { "Moral Money Team".into() }
    fn support_url() -> String { "https://github.com/nuno/mmeco".into() }
    fn copyright_start_year() -> i32 { 2026 }
    fn load_spec(&self, id: &str) -> Result<Box<dyn ChainSpec>, String> {
        Ok(Box::new(chain_spec::development_config()?))
    }
}

fn main() -> polkadot_sdk::sc_cli::Result<()> {
    let cli = Cli::parse();
    match &cli.subcommand {
        Some(Subcommand::PurgeChain(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.database))
        },
        None => {
            let runner = cli.create_runner(&cli.run)?;
            runner.run_node_until_exit(|config| async move {
                service::new_full(config).map_err(polkadot_sdk::sc_cli::Error::Service)
            })
        },
    }
}
