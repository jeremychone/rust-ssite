#![allow(unused)] // silence test utils

use anyhow::Result;
use std::path::Path;
use std::process::Command;

pub fn exec(cmd: &str, args: &[&str]) -> Result<()> {
	let mut proc = Command::new(cmd);
	// proc.current_dir(cwd);
	proc.args(args);

	proc.spawn()?.wait()?;

	Ok(())
}
