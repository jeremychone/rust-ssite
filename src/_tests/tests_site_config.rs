use crate::prelude::*;
use crate::site::config::tests::_test_infra::TESTS_DATA_DIR;
use crate::site::config::{RunMode, RunnerConfig, SiteConfig};
use anyhow::Result;
use std::collections::HashSet;
use std::path::Path;
use toml::Value;

mod _test_infra;

#[test]
fn site_config_test_from_toml() -> Result<()> {
	// --- Fixtures
	let toml = r#"
		[source]
		content_dir = "content/"
		dist_dir = "_site"
		
		[runner.pcss]
    cmd = "echo"
    args = ["pcss", "args"]
    watch_args = ["pcss", "watch"]

		[runner.rollup]
    cmd = "rollup"
    args = ["rollup", "-c"]
    watch_args = ["rollup", "-w"]  		
  "#;
	let toml: Value = toml::from_str(&toml)?;

	// --- Exec
	let root_dir = Path::new(TESTS_DATA_DIR).to_path_buf();
	let site_config = SiteConfig::from_value(root_dir, toml)?;

	// --- Checks
	assert_eq!(TESTS_DATA_DIR, site_config.root_dir.to_string_lossy());
	let content_dir = site_config.content_dir.to_string_lossy();
	// Note: Works because string is assume to be ascii 1 byte each
	let end_with = f!("{}/content", &TESTS_DATA_DIR[1..]);
	assert!(content_dir.ends_with(&end_with));

	let r_configs = site_config.runner_configs.as_ref().unwrap();
	let runner = r_configs.get(0).unwrap();
	assert_eq!("pcss", runner.name);
	assert_eq!("echo", runner.cmd);
	assert_eq!(&vec!["pcss", "args"], runner.args.as_ref().unwrap());

	Ok(())
}

#[test]
fn site_config_test_runner_config_parse_simple() -> Result<()> {
	let toml = r#"
    cmd = "echo"
    args = ["pcss", "args"]
    watch_args = ["pcss", "watch"]  
  "#;
	let toml: Value = toml::from_str(&toml)?;

	let runner_config = RunnerConfig::from_value("test_runner", &toml)?;

	assert_eq!("echo", runner_config.cmd);
	assert_eq!(&vec!["pcss", "args"], runner_config.args.as_ref().unwrap());
	assert_eq!(&vec!["pcss", "watch"], runner_config.watch_args.as_ref().unwrap());
	assert_eq!(&RunMode::Build, runner_config.run_modes.iter().next().unwrap());

	Ok(())
}

#[test]
fn site_config_test_runner_config_parse_with_run_on() -> Result<()> {
	let toml = r#"
    cmd = "echo"
    args = ["pcss", "args"]
    watch_args = ["pcss", "watch"]  
    run_on = ["Dev", "Build"]
  "#;
	let toml: Value = toml::from_str(&toml)?;

	let runner_config = RunnerConfig::from_value("test_runner", &toml)?;

	assert_eq!("echo", runner_config.cmd);
	assert_eq!(&vec!["pcss", "args"], runner_config.args.as_ref().unwrap());
	assert_eq!(&vec!["pcss", "watch"], runner_config.watch_args.as_ref().unwrap());
	assert_eq!(&HashSet::from_iter(vec![RunMode::Dev, RunMode::Build]), &runner_config.run_modes);

	Ok(())
}
