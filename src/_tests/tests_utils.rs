use crate::utils::rebase_path;
use std::path::Path;

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
