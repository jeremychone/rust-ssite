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

	#[error("Config error for the runner {0}. Cause: {1}")]
	RunnerConfigError(String, String),

	#[error("Invalid runner run_on value '{0}'. Must be 'Build' | 'Dev'")]
	RunnerConfigErrorRunOn(String),

	#[error("Value for property {0} was not found.")]
	TomlMissingValue(String),

	#[error(transparent)]
	IOError(#[from] std::io::Error),

	#[error(transparent)]
	AnyhowError(#[from] anyhow::Error),

	#[error(transparent)]
	NotifyError(#[from] notify::Error),

	#[error(transparent)]
	TomlError(#[from] toml::de::Error),

	#[error(transparent)]
	EnumParseError(#[from] strum::ParseError),
}
