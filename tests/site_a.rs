#![allow(unused)]
use crate::utils::exec;
use anyhow::Result;

mod utils;

const CWD_DIR: &str = "tests-data/site-a";

#[test]
fn site_a_simple() -> Result<()> {
	// let cwd = Path::new(CWD_DIR);

	// exec("cargo", &["run", "--release", "--", "-d", CWD_DIR])?;

	assert_eq!(2 + 2, 4);

	Ok(())
}
