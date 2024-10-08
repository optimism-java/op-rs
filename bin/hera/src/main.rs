//! Hera OP Stack Rollup node

#![doc = include_str!("../README.md")]
#![doc(issue_tracker_base_url = "https://github.com/paradigmxyz/op-rs/issues/")]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

use eyre::Result;

fn main() -> Result<()> {
    rollup::init_telemetry_stack(8090)?;

    tracing::info!("Hera OP Stack Rollup node");

    Ok(())
}
