use super::FileProcessor;
use crate::gen::processor::tests::_test_infra::TESTS_DATA_DIR;
use crate::site::Site;
use std::path::Path;

mod _test_infra;

const CONTENT_WITH_FRAMES_COUNT: &[(&str, usize); 9] = &[
	("sub-frame/content-1.html", 2),
	("sub-frame/content-2.md", 3),      // has a page frame, so, 3.
	("sub-frame/full-content.html", 0), // is doctype html, so, 0.
	("sub-dir/index.html", 1),
	("sub-dir/full-other-content.html", 0),
	("sub-dir/content.html", 2), // has a page frame and the root _frame
	("hello.md", 1),
	("hello2.html", 1),
	("full.html", 0),
];

#[test]
fn test_get_frames_count() -> anyhow::Result<()> {
	for (path, count) in CONTENT_WITH_FRAMES_COUNT {
		let site = Site::from_dir(Path::new(TESTS_DATA_DIR))?;
		let src = site.content_dir().join(path);
		let fp = FileProcessor::from_src_file(&site, src).unwrap();

		let frames = fp.get_frames(&site)?;

		assert_eq!(frames.len(), *count, "{path}");
	}

	Ok(())
}
