use crate::utils::assert_valid_dir;
use crate::utils::toml::{toml_as_option_string, toml_as_string};
use crate::Error;
use std::fs::{create_dir, create_dir_all, read_to_string};
use std::path::{Path, PathBuf};
use toml::Value;
use walkdir::{DirEntry, WalkDir};

const CONTENT_DIR_NAME: &str = "content";
const SITE_DIR_NAME: &str = "_site";
const CONFIG_FILE_NAME: &str = "ssite.toml";

pub struct S3Config {
	pub profile: String,
	pub bucket_name: String,
	pub bucket_root: String,
}

pub(super) struct SiteConfig {
	content_dir: PathBuf,
	dist_dir: PathBuf,
	s3_config: Option<S3Config>,
}

impl SiteConfig {
	pub fn content_dir(&self) -> &Path {
		self.content_dir.as_ref()
	}

	pub fn dist_dir(&self) -> &Path {
		self.dist_dir.as_ref()
	}

	pub fn s3_config(&self) -> Option<&S3Config> {
		self.s3_config.as_ref()
	}
}

impl SiteConfig {
	pub(super) fn from_dir(root_dir: &Path) -> Result<SiteConfig, Error> {
		// load S3 info
		let config_file = root_dir.join(CONFIG_FILE_NAME);
		if !config_file.exists() {
			return Err(Error::MissingConfigFile(root_dir.display().to_string()));
		}
		let toml = read_to_string(config_file)?;
		let toml: Value = toml::from_str(&toml)?;

		// get content dir (exception if not exist)
		let content_dir = toml_as_string(&toml, &["source", "content_dir"])?;
		let content_dir = root_dir.join(Path::new(&content_dir)).canonicalize()?;
		assert_valid_dir(&content_dir)?;

		let dist_dir = toml_as_string(&toml, &["source", "dist_dir"])?;
		let dist_dir = root_dir.join(Path::new(&dist_dir));
		if !dist_dir.exists() {
			create_dir_all(&dist_dir);
		}
		let dist_dir = dist_dir.canonicalize()?;

		let profile = toml_as_string(&toml, &["publish", "bucket_cred_profile"])?;
		let bucket_name = toml_as_string(&toml, &["publish", "bucket_name"])?;
		let bucket_root = toml_as_option_string(&toml, &["publish", "bucket_root"]).unwrap_or_else(|| "".to_string());

		// FIXME - for now harcode
		let s3_config = Some(S3Config {
			profile,
			bucket_name,
			bucket_root,
		});

		Ok(SiteConfig {
			content_dir,
			dist_dir,
			s3_config,
		})
	}
}
