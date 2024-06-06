/*
 * SPDX-FileCopyrightText: 2024 Matteo Dell'Acqua
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use super::utils::*;
use crate::{
    graphs::{random::ErdosRenyi, BVComp},
    prelude::SequentialLabeling,
};
use anyhow::{ensure, Result};
use clap::{ArgMatches, Args, Command, FromArgMatches};
use dsi_bitstream::prelude::*;
use log::info;
use std::path::PathBuf;

pub const COMMAND_NAME: &str = "generate";

#[derive(Args, Debug)]
#[command(about = "Generate a Erdös-Rényi random graph", long_about = None)]
struct CliArgs {
    /// The basename where to save the generated graph
    basename: PathBuf,

    /// The number of nodes to generate
    num_nodes: usize,

    /// The probability of an edge between any two nodes. Must be between 0 and 1
    arc_probability: f64,

    #[clap(flatten)]
    ca: CompressArgs,

    /// The seed for the psudorandom number generator
    seed: Option<u64>,
}

pub fn cli(command: Command) -> Command {
    command.subcommand(CliArgs::augment_args(Command::new(COMMAND_NAME)))
}

pub fn main(submatches: &ArgMatches) -> Result<()> {
    let args = CliArgs::from_arg_matches(submatches)?;
    let seed = args.seed.unwrap_or(0);
    let endianess = args.ca.endianness.clone().unwrap_or(BE::NAME.into());
    ensure!(
        args.arc_probability <= 1.0 && args.arc_probability >= 0.0,
        "ARC_PROBABILITY should be between 0 and 1. Got {}",
        args.arc_probability
    );
    info!("Generating Erdös-Rényi with {} nodes, a probability of an edge between two nodes {} and random seed {}", args.num_nodes, args.arc_probability, seed);

    let graph = ErdosRenyi::new(args.num_nodes, args.arc_probability, seed);

    if endianess == BE::NAME {
        BVComp::single_thread::<BE, _>(
            args.basename,
            graph.iter(),
            args.ca.into(),
            false,
            Some(graph.num_nodes()),
        )?;
    } else {
        BVComp::single_thread::<LE, _>(
            args.basename,
            graph.iter(),
            args.ca.into(),
            false,
            Some(graph.num_nodes()),
        )?;
    }
    Ok(())
}
