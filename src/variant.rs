//! Types for handling types of AMMs
use serde::{Deserialize, Serialize};

/// Represents a type of AMM pool
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum AmmVariant {
    UniswapV2,
    UniswapV3,
}
