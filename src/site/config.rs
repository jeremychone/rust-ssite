use crate::utils::assert_valid_dir;
use crate::utils::toml::toml_as_string;
use crate::Error;
use std::fs::{create_dir_all, read_to_string};
use std::path::{Path, PathBuf};
use toml::Value;

const CONFIG_FILE_NAME: &str = "ssite.toml";

pub struct SiteConfig {
	content_dir: PathBuf,
	dist_dir: PathBuf,
}

impl SiteConfig {
	pub fn content_dir(&self) -> &Path {
		self.content_dir.as_ref()
	}

	pub fn dist_dir(&self) -> &Path {
		self.dist_dir.as_ref()
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

		// TODO: Implement processors
		let processors = toml.get("processors").and_then(|v| v.as_table()).expect("no processors");
		for (name, proc_info) in processors {
			println!("{name}: {proc_info:?}");
		}

		// get content dir (exception if not exist)
		let content_dir = toml_as_string(&toml, &["source", "content_dir"])?;
		let content_dir = root_dir.join(Path::new(&content_dir)).canonicalize()?;
		assert_valid_dir(&content_dir)?;

		let dist_dir = toml_as_string(&toml, &["source", "dist_dir"])?;
		let dist_dir = root_dir.join(Path::new(&dist_dir));
		if !dist_dir.exists() {
			// TODO: Handle error
			let _ = create_dir_all(&dist_dir);
		}
		let dist_dir = dist_dir.canonicalize()?;

		// let profile = toml_as_string(&toml, &["publish", "bucket_cred_profile"])?;
		// let bucket_name = toml_as_string(&toml, &["publish", "bucket_name"])?;
		// let bucket_root = toml_as_option_string(&toml, &["publish", "bucket_root"]).unwrap_or_else(|| "".to_string());

		Ok(SiteConfig { content_dir, dist_dir })
	}
}
