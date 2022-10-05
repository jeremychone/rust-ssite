use crate::utils::assert_valid_dir;
use crate::utils::toml::{toml_as_string, DeepGet};
use crate::{f, s, Error};
use std::collections::HashSet;
use std::fs::{create_dir_all, read_to_string};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use strum_macros::EnumString;
use toml::Value;

use super::{RunMode, Runner};

const CONFIG_FILE_NAME: &str = "ssite.toml";

#[derive(Debug)]
pub struct SiteConfig {
	pub root_dir: PathBuf,
	pub content_dir: PathBuf,
	pub dist_dir: PathBuf,
	pub runner_configs: Option<Vec<RunnerConfig>>,
}

impl SiteConfig {
	pub(super) fn from_dir(root_dir: &Path) -> Result<SiteConfig, Error> {
		let root_dir = root_dir.canonicalize()?;

		// load S3 info
		let config_file = root_dir.join(CONFIG_FILE_NAME);
		if !config_file.exists() {
			return Err(Error::MissingConfigFile(root_dir.display().to_string()));
		}
		let toml = read_to_string(config_file)?;
		let toml: Value = toml::from_str(&toml)?;

		Self::from_value(root_dir, toml)
	}

	fn from_value(root_dir: PathBuf, toml: Value) -> Result<SiteConfig, Error> {
		// Parse the eventual runners
		let runner_configs = match toml.get("runner").and_then(|v| v.as_table()) {
			Some(runners) => Some(
				runners
					.into_iter()
					.map(|(name, props)| RunnerConfig::from_value(name, &props))
					.collect::<Result<Vec<_>, _>>()?,
			),
			None => None,
		};

		// get content dir (exception if not exist)
		let content_dir = toml.deep_string(&["source", "content_dir"])?;
		let content_dir = root_dir.join(Path::new(&content_dir)).canonicalize()?;
		assert_valid_dir(&content_dir)?;

		let dist_dir = toml.deep_string(&["source", "dist_dir"])?;
		let dist_dir = root_dir.join(Path::new(&dist_dir));
		if !dist_dir.exists() {
			// TODO: Handle error
			let _ = create_dir_all(&dist_dir).expect(&f!("Cannot create dir {dist_dir:?}"));
		}
		let dist_dir = dist_dir.canonicalize()?;

		Ok(SiteConfig {
			root_dir: root_dir.to_path_buf(),
			content_dir,
			dist_dir,
			runner_configs,
		})
	}
}

// region:    --- RunnerConfig

#[derive(Debug)]
pub struct RunnerConfig {
	pub name: String,
	pub cwd: Option<String>,
	pub cmd: String,
	pub args: Option<Vec<String>>,
	pub watch_args: Option<Vec<String>>,
	pub run_modes: HashSet<RunMode>,
}

impl RunnerConfig {
	pub fn from_value(runner_name: &str, toml: &Value) -> Result<RunnerConfig, Error> {
		let watch_args = toml.deep_vec_string(&["watch_args"]).ok();
		let args = toml.deep_vec_string(&["args"]).ok();

		// region:    --- set the run_modes
		// If we have a `run_on` property, then, it take precedence
		// Otherwise, Build by default, and Dev if we have some watch_args

		let run_on_string = toml.deep_vec_string(&["run_on"]).ok();
		let run_modes = run_on_string.map(|v| {
			v.into_iter()
				.map(|s| RunMode::from_str(&s).map_err(|_| Error::RunnerConfigErrorRunOn(s!(s))))
				.collect::<Result<HashSet<_>, _>>()
		});

		let run_modes = match run_modes {
			Some(run_modes) => run_modes?,
			None => {
				let mut run_modes = HashSet::from_iter(vec![RunMode::Build]);
				if watch_args.is_some() {
					run_modes.insert(RunMode::Dev);
				}
				run_modes
			}
		};
		// let mrun_on: Vec<RunMode> = Vec::new();

		// let default_run_modes =

		Ok(RunnerConfig {
			name: runner_name.to_string(),
			cwd: toml.deep_string(&["cwd"]).ok(),
			cmd: toml.deep_string(&["cmd"])?,
			args: toml.deep_vec_string(&["args"]).ok(),
			watch_args,
			run_modes,
		})
	}
}

// endregion: --- RunnerConfig

#[cfg(test)]
#[path = "../_tests/tests_site_config.rs"]
mod tests;
