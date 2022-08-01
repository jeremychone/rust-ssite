use self::config::{RunnerConfig, SiteConfig};
use crate::Error;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::Command;
use strum_macros::EnumString;
use toml::Value;
use walkdir::{DirEntry, WalkDir};

mod config;

#[derive(Debug)]
pub struct Site {
	content_dir: PathBuf,
	dist_dir: PathBuf,
	root_dir: PathBuf,
	runners: Option<Vec<Runner>>,
}

#[derive(Debug, Clone)]
pub struct Runner {
	name: String,
	cwd: Option<String>,
	cmd: String,
	args: Option<Vec<String>>,
	watch_args: Option<Vec<String>>,
	run_on: HashSet<RunMode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumString)]
pub enum RunMode {
	Build,
	Dev,
}

// region:    --- Runer Impls
/// Makers
impl From<RunnerConfig> for Runner {
	fn from(val: RunnerConfig) -> Self {
		Runner {
			name: val.name,
			cwd: val.cwd,
			cmd: val.cmd,
			args: val.args,
			watch_args: val.watch_args,
			run_on: val.run_modes,
		}
	}
}

impl Runner {
	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn has_run_mode(&self, run_mode: &RunMode) -> bool {
		self.run_on.contains(&run_mode)
	}

	pub fn get_build_command(&self, root_dir: &Path) -> Command {
		let mut cmd = self.get_base_cmd(root_dir);
		if let Some(args) = self.args.as_ref() {
			cmd.args(args);
		}
		cmd
	}

	pub fn get_watch_command(&self, root_dir: &Path) -> Command {
		let mut cmd = self.get_base_cmd(root_dir);
		if let Some(args) = self.watch_args.as_ref().or_else(|| self.args.as_ref()) {
			cmd.args(args);
		}
		cmd
	}

	fn get_base_cmd(&self, root_dir: &Path) -> Command {
		let mut cmd = Command::new(&self.cmd);
		let cwd = self.cwd.as_ref().map(|p| root_dir.join(p)).unwrap_or(root_dir.to_owned());
		cmd.current_dir(cwd);
		cmd
	}
}

// endregion: --- Runer Impls

// region:    --- Site Impls

/// Makers
impl Site {
	pub fn from_dir(dir: &Path) -> Result<Self, Error> {
		let config = SiteConfig::from_dir(dir)?;
		let runners = config.runner_configs.map(|v| v.into_iter().map(|v| v.into()).collect());
		Ok(Site {
			root_dir: config.root_dir,
			content_dir: config.content_dir,
			dist_dir: config.dist_dir,
			runners,
		})
	}
}

impl Site {
	pub fn root_dir(&self) -> &Path {
		&self.root_dir
	}
	pub fn content_dir(&self) -> &Path {
		&self.content_dir
	}

	pub fn dist_dir(&self) -> &Path {
		&self.dist_dir
	}

	pub fn runners(&self) -> Option<&Vec<Runner>> {
		self.runners.as_ref()
	}

	/// Return the files entries of the content folder
	#[allow(unused)]
	pub fn dist_entries(&self) -> impl Iterator<Item = DirEntry> {
		WalkDir::new(self.dist_dir())
			.into_iter()
			.filter_map(|e| e.ok())
			.filter(|e| e.path().is_file())
	}

	/// Return the files entries of the content folder
	pub fn content_entries(&self) -> impl Iterator<Item = DirEntry> + '_ {
		WalkDir::new(self.content_dir())
			.into_iter()
			.filter_entry(|e| match e.path().canonicalize() {
				Ok(path) => self.valid_content_path(&path),
				Err(_) => false,
			})
			.filter_map(|e| e.ok())
			.filter(|e| e.path().is_file())
	}

	pub fn valid_content_path(&self, path: &Path) -> bool {
		!path.starts_with(self.dist_dir()) && !path.ends_with("ssite.toml")
	}
}

// endregion: --- Site Impls

// region:    --- Runner Impls

// endregion: --- Runner Impls

#[cfg(test)]
#[path = "../_tests/test_site_mod.rs"]
mod tests;
