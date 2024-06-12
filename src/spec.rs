//! Logic and types for checkpoint specification
use std::{path::Path, sync::Arc};

use alloy::primitives::{Address, BlockNumber};
use alloy::providers::ReqwestProvider;
use amms::amm::{
    factory::Factory,
    uniswap_v2::{factory::UniswapV2Factory, UniswapV2Pool},
    uniswap_v3::{factory::UniswapV3Factory, UniswapV3Pool},
    AMM,
};
use csv::Reader;
use futures::future::join_all;
use serde::{Deserialize, Serialize};

use crate::variant::AmmVariant;

/// Default fee for AMM pools
pub const DEFAULT_FEE: u32 = 300;

/// Represents a single row of CSV
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SpecificationEntry {
    pub variant: AmmVariant,
    pub factory: Address,
    pub factory_created: BlockNumber,
    pub pool: Address,
}

impl SpecificationEntry {
    /// Retrieves both the associated AMM factory and pool for this [`SpecificationEntry`]
    async fn fetch(
        &self,
        provider: Arc<ReqwestProvider>,
    ) -> eyre::Result<(Factory, AMM)> {
        Ok(match self.variant {
            AmmVariant::UniswapV2 => (
                Factory::UniswapV2Factory(UniswapV2Factory::new(
                    self.factory,
                    self.factory_created,
                    DEFAULT_FEE,
                )),
                AMM::UniswapV2Pool(
                    UniswapV2Pool::new_from_address(
                        self.pool,
                        DEFAULT_FEE,
                        provider.clone(),
                    )
                    .await?,
                ),
            ),
            AmmVariant::UniswapV3 => (
                Factory::UniswapV3Factory(UniswapV3Factory::new(
                    self.factory,
                    self.factory_created,
                )),
                AMM::UniswapV3Pool(
                    UniswapV3Pool::new_from_address(
                        self.pool,
                        DEFAULT_FEE.into(),
                        provider.clone(),
                    )
                    .await?,
                ),
            ),
        })
    }
}

/// Represents a sequence of CSV rows
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CheckpointSpecification(pub Vec<SpecificationEntry>);

impl CheckpointSpecification {
    /// Reads a [`CheckpointSpecification`] from disk
    pub fn load<P>(path: P) -> eyre::Result<Self>
    where
        P: AsRef<Path>,
    {
        Ok(Self(
            Reader::from_path(path)?
                .deserialize()
                .collect::<Result<Vec<SpecificationEntry>, csv::Error>>()?,
        ))
    }

    /// Retrieves the entire set of AMM factories and pools specified in this
    /// [`CheckpointSpecification`]
    pub async fn fetch(
        &self,
        provider: Arc<ReqwestProvider>,
    ) -> eyre::Result<Vec<(Factory, AMM)>> {
        join_all(self.0.iter().map(|x| x.fetch(provider.clone())))
            .await
            .into_iter()
            .collect::<eyre::Result<Vec<(Factory, AMM)>>>()
    }
}
