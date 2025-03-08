use std::{
    path::Path,
    env,
    fs,
};

/// Copy files from source to destination recursively.
pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn is_dev() -> bool {
    let Some(build_env) = env::args().nth(1) else {
	panic!("Build environment not specified. <dev|production>");
    };

    if !(build_env == "dev" || build_env == "production") {
	panic!("Invalid build environment '{}'. <dev|production>", build_env);
    }

    build_env == "dev"
}
