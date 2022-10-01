use clap::{crate_version, Arg, Command};

pub fn cmd_app() -> Command {
	Command::new("ssite")
		.version(&crate_version!()[..])
		.arg(arg_root_dir())
		.subcommand(sub_dev())
}

fn sub_dev() -> Command {
	Command::new("dev").arg(arg_root_dir())
}

// region:    Common Args
fn arg_root_dir() -> Arg {
	Arg::new("root_dir")
		.short('d')
		.long("dir")
		.num_args(1)
		.help("The root dir where the driving ssite.toml resides")
}

// endregion: Common Args
