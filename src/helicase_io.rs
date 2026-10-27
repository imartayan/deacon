use anyhow::Result;
use helicase::config::*;
use helicase::{FastxParser, ParallelFastxParser};
use paraseq::parallel::ParallelProcessor;
use std::path::Path;

/// Headers and qualities, needed to write filtered reads
pub(crate) const FILTER_CONFIG: Config = ParserOptions::default().compute_quality().config();
/// Sequences only
pub(crate) const SEQ_CONFIG: Config = ParserOptions::default().ignore_headers().config();

pub(crate) fn process_parallel<const CONFIG: Config, P>(
    path: &Path,
    processor: &mut P,
    threads: usize,
) -> Result<()>
where
    P: for<'b, 'c> ParallelProcessor<&'b FastxParser<'c, CONFIG>> + Sync,
{
    ParallelFastxParser::<CONFIG>::from_file(path)?
        .threads(threads)
        .process_parallel(processor)?;
    Ok(())
}
