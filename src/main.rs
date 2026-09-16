mod watch_fn;
use std::{fs::create_dir_all, path::Path};
use watch_fn::watch_callback;
use watch_folder_lib::run_watch;

fn main() {
    let src = "src_dir";
    let dst = "dst_dir";

    create_dir_all(src).expect("Failed to create source folder");
    create_dir_all(dst).expect("Failed to create destination folder");

    let src_path = Path::new(src);
    let dst_path = Path::new(dst);

    run_watch(src_path, dst_path, watch_callback).await?;
}
