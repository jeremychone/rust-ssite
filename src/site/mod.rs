use self::config::SiteConfig;
use crate::Error;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

mod config;

pub struct Site {
	config: SiteConfig,
}

/// Factory implementation
impl Site {
	pub fn from_dir(dir: &Path) -> Result<Self, Error> {
		let config = SiteConfig::from_dir(dir)?;
		Ok(Site { config })
	}
}

/// Factory implementation
impl Site {
	pub fn content_dir(&self) -> &Path {
		self.config.content_dir()
	}

	pub fn dist_dir(&self) -> &Path {
		self.config.dist_dir()
	}

	/// Return the files entries of the content folder
	#[allow(unused)]
	pub fn dist_entries(&self) -> impl Iterator<Item = DirEntry> {
		WalkDir::new(self.config.dist_dir())
			.into_iter()
			.filter_map(|e| e.ok())
			.filter(|e| e.path().is_file())
	}

	/// Return the files entries of the content folder
	pub fn content_entries(&self) -> impl Iterator<Item = DirEntry> + '_ {
		WalkDir::new(self.config.content_dir())
			.into_iter()
			.filter_entry(|e| match e.path().canonicalize() {
				Ok(path) => self.valid_content_path(&path),
				Err(_) => false,
			})
			.filter_map(|e| e.ok())
			.filter(|e| e.path().is_file())
	}

	pub fn valid_content_path(&self, path: &Path) -> bool {
		!path.starts_with(self.config.dist_dir()) && !path.ends_with("ssite.toml")
	}
}
