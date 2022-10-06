use super::FileProcessor;
use crate::gen::processor::tests::_test_infra::TESTS_DATA_DIR;
use crate::site::Site;
use std::fs;
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
fn test_processor_get_frames_count() -> anyhow::Result<()> {
	for (path, count) in CONTENT_WITH_FRAMES_COUNT {
		let site = Site::from_dir(Path::new(TESTS_DATA_DIR))?;
		let src = site.content_dir().join(path);
		let fp = FileProcessor::from_src_file(&site, src).unwrap();

		let frames = fp.get_frames(&site)?;

		assert_eq!(frames.len(), *count, "{path}");
	}

	Ok(())
}

#[test]
fn test_processor_process_page() -> anyhow::Result<()> {
	let site = Site::from_dir(Path::new(TESTS_DATA_DIR))?;

	const FILE: &str = "sub-frame/content-2.md";
	let src = site.content_dir().join(FILE);

	let fp = FileProcessor::from_src_file(&site, src).unwrap();

	let dst = fp.process(&site)?.unwrap();
	let content = fs::read_to_string(dst)?;

	assert!(
		content.contains("Wrapped from root _frame.html"),
		"Wrapped from root _frame.html"
	);

	assert!(
		content.contains("Wrapped from sub-frame/_frame.html"),
		"Wrapped from sub-frame/_frame.html"
	);

	assert!(
		content.contains("Wrapped from content-2_frame.md"),
		"Wrapped from content-2_frame.md"
	);

	assert!(
		content.contains("<p>from sub-frame/content-2.md</p>"),
		"<p>from sub-frame/content-2.md</p>"
	);

	Ok(())
}
