mod watch_fn;
use anyhow::{Ok, Result};
use std::path::Path;
use watch_fn::watch_callback;
use watch_folder_lib::run_watch;

#[tokio::main]
async fn main() -> Result<()> {
    let src = "dir_src";
    let dst = "dir_dst";

    let src_path = Path::new(src);
    let dst_path = Path::new(dst);

    run_watch(src_path, dst_path, watch_callback).await?;

    Ok(())
}
