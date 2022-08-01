use std::collections::HashSet;

use toml::Value;

use crate::Error;

enum RunMode {
	Dev,
	Build,
}

struct RunnerConfig {
	cmd: String,
	args: Option<Vec<String>>,
	watch_args: Option<Vec<String>>,
	run_on: HashSet<RunMode>,
}

impl RunnerConfig {
	fn from_value(toml: Value) -> Result<RunnerConfig, Error> {
		Ok(RunnerConfig {
			cmd: "FIXME".to_string(),
			args: None,
			watch_args: None,
			run_on: HashSet::new(),
		})
	}
}
