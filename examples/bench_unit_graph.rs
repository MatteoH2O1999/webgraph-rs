/*
 * SPDX-FileCopyrightText: 2023 Inria
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use anyhow::Result;
use clap::Parser;
use dsi_progress_logger::*;
use lender::*;
use std::hint::black_box;
use webgraph::prelude::*;
use dsi_bitstream::prelude::*;
#[derive(Parser, Debug)]
#[command(about = "Breadth-first visits a graph.", long_about = None)]
struct Args {
    /// The basename of the graph.
    basename: String,
}

fn bench_impl<E: Endianness + 'static>(args: Args) -> Result<()> 
where
    for<'a> BufBitReader<E, MemWordReader<u32, &'a [u32]>>: ZetaRead<E> + DeltaRead<E> + GammaRead<E> + BitSeek
{
    let graph = webgraph::graph::bvgraph::load::<E>(&args.basename)?;
    let unit = UnitLabelGraph(&graph);
    let labelled = Zip(
        webgraph::graph::bvgraph::load::<E>(&args.basename)?,
        webgraph::graph::bvgraph::load::<E>(&args.basename)?,
    );
    for _ in 0..10 {
        let mut pl = ProgressLogger::default();
        pl.start("Standard graph lender...");
        let mut iter = graph.iter();
        while let Some((x, s)) = iter.next() {
            black_box(x);
            for i in s {
                black_box(i);
            }
        }
        pl.done_with_count(graph.num_nodes());

        pl.start("Unit graph lender...");
        let mut iter = unit.iter();
        while let Some((x, s)) = iter.next() {
            black_box(x);
            for i in s {
                black_box(i);
            }
        }
        pl.done_with_count(unit.num_nodes());

        let mut pl = ProgressLogger::default();
        pl.start("Standard graph successors...");
        for x in 0..graph.num_nodes() {
            black_box(x);
            for i in graph.successors(x) {
                black_box(i);
            }
        }
        pl.done_with_count(graph.num_nodes());

        pl.start("Unit graph successors...");
        for x in 0..unit.num_nodes() {
            black_box(x);
            for i in unit.successors(x) {
                black_box(i);
            }
        }
        pl.done_with_count(unit.num_nodes());

        pl.start("Zipped-projected graph successors...");
        for x in 0..unit.num_nodes() {
            black_box(x);
            for (i, _) in labelled.successors(x) {
                black_box(i);
            }
        }
        pl.done_with_count(unit.num_nodes());
    }

    Ok(())
}

pub fn main() -> Result<()> {
    let args = Args::parse();

    stderrlog::new()
        .verbosity(2)
        .timestamp(stderrlog::Timestamp::Second)
        .init()?;

    match get_endianess(&args.basename)?.as_str() {
        #[cfg(any(feature = "be_bins", not(any(feature = "be_bins", feature = "le_bins"))))]
        BE::NAME => bench_impl::<BE>(args),
        #[cfg(any(feature = "le_bins", not(any(feature = "be_bins", feature = "le_bins"))))]
        LE::NAME => bench_impl::<LE>(args),
        _ => panic!("Unknown endianness"),
    }
}
