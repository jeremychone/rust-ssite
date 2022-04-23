use toml::to_string;

use crate::site::Site;
use crate::utils::{lower_case, rebase_path};
use crate::Error;
use std::ffi::OsStr;
use std::fs::{self, create_dir_all};
use std::path::{Path, PathBuf};

use super::safer_remove_file_and_empty_parent;

enum SrcType {
	ReadmeMarkdown,
	Markdown,
	IndexHtml,
	Html,
	Other,
}

impl SrcType {
	fn from_path(path: &Path) -> Self {
		let (name, ext) = (lower_case(path.file_name()), lower_case(path.extension()));

		match (name.as_ref().map(|v| v.as_str()), ext.as_ref().map(|v| v.as_str())) {
			// TODO - if no name, probably should be SrcType::Unknown (so that we do nothing)
			(None, _) => SrcType::Other,
			(Some("readme.md"), _) => SrcType::ReadmeMarkdown,
			(Some("index.html"), _) => SrcType::IndexHtml,
			(Some(_), Some("html")) => SrcType::Html,
			(_, _) => SrcType::Other,
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
			let mm = mime_guess::from_path(&self.src_file);
			println!(
				"--- src_file: {}\n    dst_file: {}\n        mime: {}",
				self.src_file.display(),
				self.dist_file.display(),
				mm.first_or_octet_stream()
			);

			if let Some(dst_dir) = self.dist_file.parent() {
				if !dst_dir.exists() {
					create_dir_all(dst_dir)?;
				}
			}

			fs::copy(&self.src_file, &self.dist_file)?;
			Ok(Some(self.dist_file.to_owned()))
		}
	}
}

// region:    --- Utils
fn get_dist_file(site: &Site, src_type: &SrcType, src_file: &Path) -> Option<PathBuf> {
	// if not a file, return None
	if !src_file.is_file() {
		return None;
	}

	let (content_dir, site_dir) = (site.content_dir(), site.dist_dir());
	// for now, just return the result of rebase path
	let dist_file = rebase_path(content_dir, src_file, site_dir);

	match src_type {
		SrcType::ReadmeMarkdown => todo!(),
		SrcType::Markdown => todo!(),
		SrcType::IndexHtml => todo!(),
		SrcType::Html => todo!(),
		SrcType::Other => todo!(),
	}
}

// endregion: --- Utils
