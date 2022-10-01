use crate::cmd::app::cmd_app;
use crate::gen::gen;
use crate::site::{RunMode, Site};
use crate::utils::assert_valid_dir;
use crate::{s, Error};
use clap::ArgMatches;
use std::env;
use std::path::Path;
use std::process::Command;
use std::time::Duration;
use tokio::time::sleep;

mod app;

pub async fn cmd_run() -> Result<(), Error> {
	let argm = cmd_app().get_matches();

	// get the dir from the root command or sub command
	let dir = argm.get_one::<String>("root_dir").or_else(|| {
		if let Some((_, sub)) = &argm.subcommand() {
			sub.get_one::<String>("root_dir")
		} else {
			None
		}
	});

	let dir = dir
		.map(|d| Path::new(d).to_path_buf())
		.unwrap_or_else(|| env::current_dir().unwrap().to_path_buf());

	assert_valid_dir(&dir)?;

	// execute the sub command
	match argm.subcommand() {
		Some(("dev", sub_cmd)) => exec_dev(&dir, &sub_cmd).await?,
		_ => {
			cmd_app().print_long_help()?;
			println!("\n");
		}
	}

	Ok(())
}

async fn exec_dev(dir: &Path, _argm: &ArgMatches) -> Result<(), Error> {
	let site = Site::from_dir(dir)?;

	// if we have runners we execute them
	if let Some(runners) = site.runners() {
		// --- First the the runners for Build
		for runner in runners.iter().filter(|r| r.has_run_mode(&RunMode::Build)) {
			println!("Build - Run runner '{}'", runner.name());
			let mut cmd = runner.get_build_command(dir);
			cmd.spawn()?.wait();
		}

		// --- Then the dev
		for runner in runners.iter().filter(|r| r.has_run_mode(&RunMode::Dev)) {
			let mut cmd = runner.get_watch_command(dir);
			let name = s!(runner.name());
			tokio::spawn(async move {
				println!("Watch Start - runner: '{name}'");
				match cmd.spawn().map(|mut p| p.wait()) {
					Ok(_) => println!("Watch End - runner: '{name}'"),
					Err(_) => println!("Watch ERROR - runner: '{name}'"),
				}
			});
		}
	}
	gen(&site, true).await
}
