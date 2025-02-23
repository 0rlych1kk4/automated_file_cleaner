use log::{info, error};
use std::env;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

const DAYS_THRESHOLD: u64 = 30; // Files older than this will be deleted
const DRY_RUN: bool = true; // Set to false to enable actual deletion

fn main() {
    env_logger::init();
    
    let target_directory = "./target_directory"; // Change this to your folder
    
    match clean_directory(target_directory) {
        Ok(_) => info!("File cleanup completed successfully."),
        Err(e) => error!("Failed to clean directory: {}", e),
    }
}

fn clean_directory(directory: &str) -> std::io::Result<()> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let threshold = now - (DAYS_THRESHOLD * 24 * 60 * 60);

    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let metadata = fs::metadata(&path)?;
            if let Ok(modified) = metadata.modified() {
                if let Ok(mod_time) = modified.duration_since(UNIX_EPOCH) {
                    if mod_time.as_secs() < threshold {
                        delete_file(&path);
                    }
                }
            }
        }
    }
    Ok(())
}

fn delete_file(path: &Path) {
    if DRY_RUN {
        info!("[DRY RUN] Would delete: {}", path.display());
    } else {
        match fs::remove_file(path) {
            Ok(_) => info!("Deleted: {}", path.display()),
            Err(e) => error!("Failed to delete {}: {}", path.display(), e),
        }
    }
}
