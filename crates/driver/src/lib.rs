#![doc = include_str!("../README.md")]
#![doc(issue_tracker_base_url = "https://github.com/anton-rs/hilo/issues/")]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(not(test), no_std)]

extern crate alloc;

mod blobs;
pub use blobs::{
    BlobSidecarProvider, OnlineBlobProvider, OnlineBlobProviderBuilder,
    OnlineBlobProviderWithFallback,
};

mod beacon_client;
pub use beacon_client::{
    APIConfigResponse, APIGenesisResponse, BeaconClient, OnlineBeaconClient, ReducedConfigData,
    ReducedGenesisData,
};

mod pipeline;
pub use pipeline::{
    HiloAttributesBuilder, HiloAttributesQueue, HiloDataProvider, HiloDerivationPipeline,
};

mod chain_provider;
pub use chain_provider::{reth_to_alloy_tx, InMemoryChainProvider};

mod blob_provider;
pub use blob_provider::{DurableBlobProvider, InnerBlobProvider, LayeredBlobProvider};

mod l2_chain_provider;
pub use l2_chain_provider::InMemoryL2ChainProvider;
