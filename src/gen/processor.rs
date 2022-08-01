use toml::to_string;

use crate::consts::{FRAME, INCLUDE_CONTENT};
use crate::site::Site;
use crate::utils::{lower_case, rebase_path};
use crate::xts::{XStr, XString};
use crate::Error;
use aho_corasick::AhoCorasick;
use std::ffi::OsStr;
use std::format as f;
use std::fs::{self, create_dir_all};
use std::path::{Path, PathBuf};

use super::safer_remove_file_and_empty_parent;

#[derive(Debug)]
enum SrcType {
	Frame,
	ReadmeMarkdown,
	Markdown,
	IndexHtml,
	Html,
	Other,
}

impl SrcType {
	fn from_path(path: &Path) -> Self {
		let (name, ext) = (lower_case(path.file_name()), lower_case(path.extension()));

		match (name.x_str(), ext.x_str()) {
			// TODO - if no name, probably should be SrcType::Unknown (so that we do nothing)
			(Some("readme.md") | Some("README.md"), _) => SrcType::ReadmeMarkdown,
			(Some("index.html"), _) => SrcType::IndexHtml,
			(Some("_frame.html"), _) => SrcType::Frame,
			(Some(_), Some("html")) => SrcType::Html,
			(Some(_), Some("md")) => SrcType::Markdown,
			(None, _) => SrcType::Other,
			(_, _) => SrcType::Other,
		}
	}

	fn is_for_html_render(&self) -> bool {
		match self {
			SrcType::ReadmeMarkdown => true,
			SrcType::Markdown => true,
			SrcType::IndexHtml => true,
			SrcType::Html => true,
			SrcType::Frame => false,
			SrcType::Other => false,
		}
	}
}

pub struct FileProcessor {
	src_file: PathBuf,
	src_type: SrcType,
	dist_file: PathBuf,
}

/// Constructor(s)
impl FileProcessor {
	pub fn from_src_file(site: &Site, src_file: PathBuf) -> Option<Self> {
		let src_type = SrcType::from_path(&src_file);
		match get_dist_file(site, &src_type, &src_file) {
			Some(dist_file) => Some(FileProcessor {
				src_file,
				src_type,
				dist_file,
			}),
			None => None,
		}
	}
}

/// Processors
impl FileProcessor {
	pub fn process(&self, site: &Site) -> Result<Option<PathBuf>, Error> {
		// if the src file does not exist, then, we clean the dist file
		if !self.src_file.exists() {
			safer_remove_file_and_empty_parent(&self.dist_file)?;
			Ok(None)
		}
		// otherwise, we process (e.g., process and copy the src file)
		else {
			// --- create the parent dist dir if needed
			if let Some(dst_dir) = self.dist_file.parent() {
				if !dst_dir.exists() {
					create_dir_all(dst_dir)?;
				}
			}

			// --- get the frames
			// Call render, and if there is some content, we use the content
			// Otherwise, just copy the file
			match self.render_content(site) {
				Ok(Some(content)) => {
					fs::write(&self.dist_file, content)?;
				}
				Ok(None) => {
					fs::copy(&self.src_file, &self.dist_file)?;
				}
				Err(ex) => println!("Error while rendering file {:?}", ex),
			}

			// --- debug
			let mm = mime_guess::from_path(&self.src_file);
			let frames = self.get_frames(site);
			println!(
				"->> src_file: {}\n     srctype: {:?}\n    dst_file: {}\n        mime: {}",
				self.src_file.display(),
				self.src_type,
				self.dist_file.display(),
				mm.first_or_octet_stream()
			);
			println!("      frames: {frames:?}");

			Ok(Some(self.dist_file.to_owned()))
		}
	}

	/// Render the content as string.
	/// Return None if the content does not need rendering (can be copied directly)
	fn render_content(&self, site: &Site) -> Result<Option<String>, Error> {
		if !self.src_type.is_for_html_render() {
			return Ok(None);
		}
		let frames = self.get_frames(site);

		// if not to render
		let src_content = fs::read_to_string(&self.src_file)?;

		// TODO: Process content with handlebars

		if frames.len() == 0 {
			Ok(Some(src_content))
		} else {
			let patterns = &[INCLUDE_CONTENT];
			let mut content = src_content;

			for frame in frames.iter() {
				let frame_content = fs::read_to_string(frame)?;
				let ac = AhoCorasick::new(patterns);
				let res = ac.replace_all_bytes(frame_content.as_bytes(), &[&content]);
				let rendered = std::str::from_utf8(&res).unwrap();
				content = rendered.to_string();
			}

			Ok(Some(content))
		}
	}

	fn get_frames(&self, site: &Site) -> Vec<PathBuf> {
		let mut path = self.src_file.to_path_buf();

		let mut frames: Vec<PathBuf> = Vec::new();

		while let Some(dir) = path.parent() {
			let frame = dir.join(FRAME);
			if frame.is_file() {
				frames.push(frame.to_owned())
			}
			if dir == site.content_dir() {
				break;
			}
			path = dir.to_path_buf();
		}

		frames
	}
}

// region:    --- Utils

fn get_dist_file(site: &Site, src_type: &SrcType, src_file: &Path) -> Option<PathBuf> {
	// if not a file, return None
	if !src_file.is_file() {
		return None;
	}

	if let SrcType::Frame = src_type {
		return None;
	}

	let (content_dir, site_dist_dir) = (site.content_dir(), site.dist_dir());

	// for now, just return the result of rebase path
	if let Some(mut dist_file) = rebase_path(content_dir, src_file, site_dist_dir) {
		let new_file_name = match src_type {
			SrcType::ReadmeMarkdown => Some("index.html".to_owned()),
			SrcType::Markdown | SrcType::Html => dist_file.file_stem().x_string(),
			_ => None,
		};

		match new_file_name {
			Some(new_file_name) => {
				dist_file.set_file_name(new_file_name);
				Some(dist_file)
			}
			None => Some(dist_file),
		}
	} else {
		None
	}
}

// endregion: --- Utils
