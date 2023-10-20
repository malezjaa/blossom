use std::{fs, io};
use std::io::ErrorKind;
use std::path::Path;

pub fn copy_dir_contents(src: &Path, dst: &Path) -> std::io::Result<()> {
    if src.is_dir() {
        if !dst.exists() {
            fs::create_dir(dst)?;
        }

        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let dst_path = dst.join(file_name);

            if entry.file_type()?.is_dir() {
                copy_dir_contents(&path, &dst_path)?;
            } else {
                fs::copy(&path, &dst_path)?;
            }
        }
    }

    Ok(())
}

#[cfg(unix)]
pub fn symlink_dir(original: &Path, link: &Path) -> io::Result<()> {
    os::unix::fs::symlink(original, link)
}

#[cfg(windows)]
pub fn symlink_dir(original: &Path, link: &Path) -> io::Result<()> {
    junction::create(original, link)
}

pub fn symlink_pkg(symlink_target: &Path, symlink_path: &Path) {
    if let Err(error) = symlink_dir(symlink_target, symlink_path) {
        match error.kind() {
            ErrorKind::AlreadyExists => {}
            _ => panic!(
                "Failed to create symlink at {symlink_path:?} to {symlink_target:?}: {error}"
            )
        }
    }
}
