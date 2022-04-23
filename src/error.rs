#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Missing config file 'ssite.toml' in root dir {0} ")]
	MissingConfigFile(String),

	#[error("Missing content dir {0} ")]
	MissingContentDir(String),

	#[error("Missing config property {0}")]
	MissingConfigProperty(String),

	#[error("Invalid or missing S3 config")]
	InvalidS3Config,

	#[error("Site root path {0} is not a valid directory path. Provide valid path with -d 'some/valid/dir/path'")]
	SiteDirMissing(String),

	#[error(transparent)]
	IOError(#[from] std::io::Error),

	#[error(transparent)]
	AnyhowError(#[from] anyhow::Error),

	#[error(transparent)]
	NotifyError(#[from] notify::Error),

	#[error(transparent)]
	TomlError(#[from] toml::de::Error),
}
