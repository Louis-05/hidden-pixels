use std::path::{Path, PathBuf};

pub fn gen_name(
    folder: impl AsRef<Path>,
    base_name: impl AsRef<str>,
    added_name: impl AsRef<str>,
) -> PathBuf {
    let mut file_name = String::new();
    file_name.push_str(added_name.as_ref());
    file_name.push_str(base_name.as_ref());
    folder.as_ref().join(file_name)
}
