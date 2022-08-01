use super::*;
use anyhow::Result;

#[test]
fn site_test_site_from_dir() -> Result<()> {
	let site = Site::from_dir(Path::new(".tests-data/site-a"))?;
	println!("->> site {:?}", site);
	Ok(())
}
