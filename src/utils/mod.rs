use crate::Error;
use pathdiff::diff_paths;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

pub mod toml;

mod x_string;

// re-export

pub use self::x_string::*;

pub fn lower_case(ostr: Option<&OsStr>) -> Option<String> {
	ostr.and_then(|s| s.to_str()).and_then(|s| Some(s.to_lowercase()))
}

pub fn assert_valid_dir(path: &Path) -> Result<(), Error> {
	if !path.exists() || !path.is_dir() {
		Err(Error::SiteDirMissing(path.to_string_lossy().to_string()))
	} else {
		Ok(())
	}
}

pub fn rebase_path(src_base_dir: &Path, src_file: &Path, dst_base_dir: &Path) -> Option<PathBuf> {
	if let Some(diff) = diff_paths(src_file, src_base_dir) {
		let dp = dst_base_dir.join(diff);
		Some(dp)
	} else {
		None
	}
}

#[cfg(test)]
#[path = "../_tests/tests_utils.rs"]
mod tests;
