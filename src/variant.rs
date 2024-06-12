use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum AmmVariant {
    UniswapV2,
    UniswapV3,
}
