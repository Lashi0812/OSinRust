use std::{fs, io::Write, os::unix::fs::MetadataExt, u64};

const ONE_MEG: u32 = 1 << 20;
fn main() -> std::io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("/tmp/foo")?;

    let mut prior_block: Option<u64> = None;

    for _i in 0..ONE_MEG {
        file.write(b".")?;
        let meta = file.metadata()?;
        if meta.blocks() != prior_block.unwrap_or(0) {
            println!(
                "Size: {} blocks :{}  on disk: {}",
                meta.size(),
                meta.blocks(),
                meta.blocks() * 512
            );
            prior_block = Some(meta.blocks());
        }
    }

    Ok(())
}
