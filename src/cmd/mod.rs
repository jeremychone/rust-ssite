use crate::cmd::app::cmd_app;
use crate::gen::gen;
use crate::site::Site;
use crate::utils::assert_valid_dir;
use crate::Error;
use clap::ArgMatches;
use std::env;
use std::path::Path;

mod app;

pub async fn cmd_run() -> Result<(), Error> {
	let argm = cmd_app().get_matches();

	// get the dir from the root command or sub command
	let dir = argm.value_of("root_dir").or_else(|| {
		if let Some((_, sub)) = &argm.subcommand() {
			sub.value_of("root_dir")
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
	gen(&site, true).await
}
