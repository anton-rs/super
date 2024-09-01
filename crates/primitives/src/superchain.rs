//! Superchain types.

use alloc::{string::String, vec::Vec};
use alloy_primitives::Address;

use crate::ChainConfig;
use crate::HardForkConfiguration;

/// A superchain configuration.
#[derive(Debug, Clone, Default, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
pub struct Superchain {
    /// Superchain identifier, without capitalization or display changes.
    pub name: String,
    /// Superchain configuration file contents.
    pub config: SuperchainConfig,
    /// Chain IDs of chains that are part of this superchain.
    pub chains: Vec<ChainConfig>,
}

/// A superchain configuration file format
#[derive(Debug, Clone, Default, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
pub struct SuperchainConfig {
    /// Superchain name (e.g. "Mainnet")
    #[cfg_attr(feature = "serde", serde(rename = "Name"))]
    pub name: String,
    /// Superchain L1 anchor information
    #[cfg_attr(feature = "serde", serde(rename = "L1"))]
    pub l1: SuperchainL1Info,
    /// Optional addresses for the superchain-wide default protocol versions contract.
    #[cfg_attr(feature = "serde", serde(rename = "ProtocolVersionsAddr"))]
    pub protocol_versions_addr: Option<Address>,
    /// Optional address for the superchain-wide default superchain config contract.
    #[cfg_attr(feature = "serde", serde(rename = "SuperchainConfigAddr"))]
    pub superchain_config_addr: Option<Address>,
    /// Hardfork Configuration. These values may be overridden by individual chains.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub hardfork_defaults: HardForkConfiguration,
}

/// Superchain L1 anchor information
#[derive(Debug, Clone, Default, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
pub struct SuperchainL1Info {
    /// L1 chain ID
    #[cfg_attr(feature = "serde", serde(rename = "ChainID"))]
    pub chain_id: u64,
    /// L1 chain public RPC endpoint
    #[cfg_attr(feature = "serde", serde(rename = "PublicRPC"))]
    pub public_rpc: String,
    /// L1 chain explorer RPC endpoint
    #[cfg_attr(feature = "serde", serde(rename = "Explorer"))]
    pub explorer: String,
}

/// Level of integration with the superchain.
#[derive(Debug, Clone, Default, Hash, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)
)]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
#[repr(u8)]
pub enum SuperchainLevel {
    /// Frontier chains are chains with customizations beyond the
    /// standard OP Stack configuration and are considered "advanced".
    Frontier = 0,
    /// Standard chains don't have any customizations beyond the
    /// standard OP Stack configuration and are considered "vanilla".
    #[default]
    Standard = 1,
}
