use clap::{crate_version, Arg, Command};

pub fn cmd_app() -> Command<'static> {
	Command::new("ssite")
		.version(&crate_version!()[..])
		.arg(arg_root_dir())
		.subcommand(sub_dev())
}

fn sub_dev() -> Command<'static> {
	Command::new("dev").arg(arg_root_dir())
}

// region:    Common Args
fn arg_root_dir() -> Arg<'static> {
	Arg::new("root_dir")
		.short('d')
		.long("dir")
		.takes_value(true)
		.help("The root dir where the driving ssite.toml resides")
}

// endregion: Common Args
