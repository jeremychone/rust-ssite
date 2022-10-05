use super::safer_remove_file_and_empty_parent;
use crate::consts::{FRAME, INCLUDE_CONTENT};
use crate::prelude::*;
use crate::site::Site;
use crate::utils::{lower_case, rebase_path, DispStr};
use crate::utils::{XStr, XString};
use aho_corasick::AhoCorasick;
use comrak::{markdown_to_html, ComrakOptions, ComrakRenderOptions};
use pathdiff::diff_paths;
use std::fs::{self, create_dir_all, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

const DOC_TYPE: &str = "<!DOCTYPE html>";

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

#[derive(Debug)]
pub struct FileProcessor {
	src_file: PathBuf,
	src_type: SrcType,
	dist_file: PathBuf,
}

/// Constructors & Getters/Setters
impl FileProcessor {
	pub fn from_src_file(site: &Site, src_file: PathBuf) -> Option<Self> {
		let src_type = SrcType::from_path(&src_file);
		// let dist_file = match src_type {
		// 	SrcType::Frame => None,
		// 	_ => Some(get_dist_file(site, &src_type, &src_file)),
		// };
		match get_dist_file(site, &src_type, &src_file) {
			Some(dist_file) => Some(FileProcessor {
				src_file,
				src_type,
				dist_file,
			}),
			None => None,
		}
	}

	pub fn is_for_html_render(&self) -> bool {
		self.src_type.is_for_html_render()
	}

	pub fn root_rel_dist_file(&self, site: &Site) -> Option<PathBuf> {
		diff_paths(&self.dist_file, site.root_dir())
	}

	pub fn root_rel_src_file(&self, site: &Site) -> Option<PathBuf> {
		diff_paths(&self.src_file, site.root_dir())
	}
}

/// Processors
impl FileProcessor {
	pub fn process(&self, site: &Site) -> Result<Option<PathBuf>> {
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
			println!(
				"- process:    {:<40} >>    {}",
				self.root_rel_src_file(site).disp_str(),
				self.root_rel_dist_file(site).disp_str()
			);

			// --- debug
			// let mm = mime_guess::from_path(&self.src_file);
			// let frames = self.get_frames(site);
			// println!(
			// 	"->> src_file: {}\n     srctype: {:?}\n    dst_file: {}\n        mime: {}",
			// 	self.src_file.display(),
			// 	self.src_type,
			// 	self.dist_file.display(),
			// 	mm.first_or_octet_stream()
			// );
			// println!("      frames: {frames:?}");

			Ok(Some(self.dist_file.to_owned()))
		}
	}

	/// Render the content as string.
	/// Return None if the content does not need rendering (can be copied directly)
	fn render_content(&self, site: &Site) -> Result<Option<String>> {
		if !self.is_for_html_render() {
			return Ok(None);
		}
		let frames = self.get_frames(site)?;

		// if not to render
		let mut src_content = fs::read_to_string(&self.src_file)?;

		// if it is markdown
		match self.src_type {
			SrcType::Markdown | SrcType::ReadmeMarkdown => {
				let render_opts = ComrakRenderOptions {
					unsafe_: true,
					..Default::default()
				};

				let opts = ComrakOptions {
					render: render_opts,
					..Default::default()
				};
				// unsafe_
				src_content = markdown_to_html(&src_content, &opts)
			}
			_ => (),
		}

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

	fn get_frames(&self, site: &Site) -> Result<Vec<PathBuf>> {
		let mut path = self.src_file.to_path_buf();

		let mut frames: Vec<PathBuf> = Vec::new();

		// if this file is a doctype html, then, no frames.
		if is_doctype_html(&path)? {
			return Ok(frames);
		}

		while let Some(dir) = path.parent() {
			let frame = dir.join(FRAME);
			if frame.is_file() {
				frames.push(frame.to_owned());
				// if this frame is a doctype html, then, it's the last.
				if is_doctype_html(&path)? {
					break;
				}
			}
			// if the dir is the content_dir, then, this the end of line.
			if dir == site.content_dir() {
				break;
			}
			path = dir.to_path_buf();
		}

		Ok(frames)
	}
}

// region:    --- Utils

fn get_dist_file(site: &Site, src_type: &SrcType, src_file: &Path) -> Option<PathBuf> {
	// if not a file, return None.
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

fn is_doctype_html(file: &Path) -> Result<bool> {
	if file.is_file() {
		let file = File::open(file)?;
		let reader = BufReader::new(file);
		if let Some(Ok(first_line)) = reader.lines().next() {
			if first_line.trim() == DOC_TYPE {
				return Ok(true);
			}
		}
	}

	Ok(false)
}

// endregion: --- Utils

// region:    --- Tests
#[cfg(test)]
#[path = "../_tests/tests_processor.rs"]
mod tests;
// endregion: --- Tests
