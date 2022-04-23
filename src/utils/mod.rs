use crate::Error;
use pathdiff::diff_paths;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

pub mod toml;

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
mod tests {
	use super::*;

	#[test]
	fn test_rebase_path() -> Result<(), Box<dyn std::error::Error>> {
		// FIXTURE
		let src_base_dir = Path::new("/src_base_dir");
		let src_file = src_base_dir.join("some/file.txt");
		let dst_base_dir = Path::new("dest_dir/");

		// ACTION
		let dst_file = rebase_path(src_base_dir, &src_file, dst_base_dir);

		// CHECK
		assert_eq!(Some(Path::new("dest_dir/some/file.txt").to_path_buf()), dst_file);

		Ok(())
	}
}
