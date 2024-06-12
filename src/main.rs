use std::{path::PathBuf, sync::Arc};

use alloy::providers::{Provider, ProviderBuilder};
use amms::amm::factory::Factory;
use amms::{amm::AMM, sync::checkpoint::construct_checkpoint};
use clap::Parser;
use log::{error, info};
use url::Url;

use crate::spec::CheckpointSpecification;

mod spec;
mod variant;

const DEFAULT_RPC_URL: &str = "https://eth.merkle.io";
const DEFAULT_OUTPUT_PATH: &str = ".cfmms-checkpoint.json";

#[derive(Parser)]
struct Opts {
    #[clap(short, long)]
    rpc: Option<Url>,
    r#in: PathBuf,
    out: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    pretty_env_logger::init_timed();
    let opts: Opts = Opts::parse();

    let provider = Arc::new(
        ProviderBuilder::new().on_http(
            opts.rpc.unwrap_or(
                DEFAULT_RPC_URL
                    .parse::<Url>()
                    .expect("Invalid hardcoded RPC URL"),
            ),
        ),
    );

    /* seems repetitive but minimises network requests! */
    let spec = CheckpointSpecification::load(opts.r#in)?;
    let factories_and_pools = match spec.fetch(provider.clone()).await {
        Ok(t) => {
            info!("Retrieved all {} pools", t.len());
            t
        }
        Err(e) => {
            error!("Failed to retrieve all pools: {:?}", e);
            return Err(e);
        }
    };
    let (factories, pools): (Vec<Factory>, Vec<AMM>) = (
        factories_and_pools
            .iter()
            .map(|(factory, _)| factory)
            .cloned()
            .collect(),
        factories_and_pools
            .iter()
            .map(|(_, pool)| pool)
            .cloned()
            .collect(),
    );

    construct_checkpoint(
        factories,
        &pools,
        provider.get_block_number().await?,
        opts.out.unwrap_or(DEFAULT_OUTPUT_PATH.into()),
    )?;
    Ok(())
}
