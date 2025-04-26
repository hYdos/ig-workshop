use std::{
    env, fs,
    io::{self, ErrorKind},
    path::{Path, PathBuf},
};

fn main() {
    // Re-run if anything in ArkCore changes:
    println!("cargo:rerun-if-changed=ArkCore");

    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let profile = env::var("PROFILE").unwrap(); // "debug" or "release"
    let target_dir = manifest.join("target").join(&profile).join("ArkCore");
    let src_dir = manifest.join("ArkCore");

    // Start the recursive copy
    if let Err(e) = copy_dir_all(&src_dir, &target_dir) {
        println!(
            "cargo:warning=Failed to copy ArkCore → {}: {}",
            target_dir.display(),
            e
        );
    }
}

/// Recursively copy a directory tree, creating directories as needed,
/// and stripping read-only on the destination if necessary.
fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<()> {
    // Create the destination directory (and parents) if needed
    match fs::create_dir_all(dst) {
        Ok(()) => {}
        Err(e) if e.kind() == ErrorKind::PermissionDenied => {
            println!(
                "cargo:warning=Permission denied creating {} — trying to remove read-only flag",
                dst.display()
            );
            // Try to remove read-only attribute on existing directory
            let mut perms = fs::metadata(dst)?.permissions();
            perms.set_readonly(false);
            fs::set_permissions(dst, perms)?;
            fs::create_dir_all(dst)?; // retry
        }
        Err(e) => return Err(e),
    }

    for entry in fs::read_dir(src)? {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                println!(
                    "cargo:warning=Couldn’t read entry in {}: {}",
                    src.display(),
                    e
                );
                continue;
            }
        };
        let path = entry.path();
        let dest = dst.join(entry.file_name());

        if path.is_dir() {
            // Recurse into subdirectory
            copy_dir_all(&path, &dest)?;
        } else {
            // It’s a file — copy it, handling PermissionDenied by clearing read-only
            match fs::copy(&path, &dest) {
                Ok(_) => {}
                Err(e) if e.kind() == ErrorKind::PermissionDenied => {
                    println!(
                        "cargo:warning=Permission denied copying {} → {}; clearing read-only and retrying",
                        path.display(),
                        dest.display()
                    );
                    // Remove readonly on dest if it exists
                    if let Ok(mut perms) = fs::metadata(&dest).map(|m| m.permissions()) {
                        perms.set_readonly(false);
                        let _ = fs::set_permissions(&dest, perms);
                    }
                    fs::copy(&path, &dest)?; // retry
                }
                Err(e) => {
                    println!(
                        "cargo:warning=Failed copying {} → {}: {}",
                        path.display(),
                        dest.display(),
                        e
                    );
                }
            }
        }
    }
    Ok(())
}
