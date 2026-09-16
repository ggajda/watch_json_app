use anyhow::Result;
use notify::Event;
use std::path::Path;

pub async fn watch_callback(dst_path: &Path, event: Event) -> Result<()> {
    println!("Copied!");

    Ok(())
}
