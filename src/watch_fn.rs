use anyhow::Result;
use notify::Event;
use std::fs::copy;
use std::path::PathBuf;
use tokio::time::{Duration, sleep};

pub async fn watch_callback(dst_path: PathBuf, event: Event) -> Result<()> {
    if !event.kind.is_create() {
        return Ok(());
    }

    for path in event.paths {
        let Some(file_name) = path.file_name() else {
            continue;
        };

        let destination = dst_path.join(file_name);

        sleep(Duration::from_millis(500)).await;

        println!("Start to copy: {:?} -> {:?}", path, destination);

        copy(&path, &destination)?;

        println!("Copied {:?} -> {:?}", path, destination);
    }

    Ok(())
}
