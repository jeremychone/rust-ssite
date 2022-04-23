use dirs::home_dir;
use regex::Regex;

use crate::Error;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;

#[derive(Debug)]
pub struct AwsCred {
	pub id: String,
	pub secret: String,
	pub region: String,
}

pub fn extract_aws_cred_from_profile(profile: &str) -> Result<AwsCred, Error> {
	let region = extract_region_from_aws_config(profile)?;
	let id_secret = extract_id_secret_from_aws_config(profile)?;

	if let (Some(region), Some((id, secret))) = (region, id_secret) {
		Ok(AwsCred { region, id, secret })
	} else {
		Err(Error::InvalidS3Config)
	}
}

fn extract_id_secret_from_aws_config(profile: &str) -> Result<Option<(String, String)>, std::io::Error> {
	// read the content
	let content = read_aws_credentials()?;
	let data = parse_aws_regex_block(&format!(r"\[{}\][\r\n]+([^\[]+)", profile), &content);
	let id = data.get("aws_access_key_id").map(|v| v.to_owned());
	let secret = data.get("aws_secret_access_key").map(|v| v.to_owned());

	if let (Some(id), Some(secret)) = (id, secret) {
		Ok(Some((id, secret)))
	} else {
		Ok(None)
	}
}

fn extract_region_from_aws_config(profile: &str) -> Result<Option<String>, std::io::Error> {
	// read the content
	let content = read_aws_config()?;
	let data = parse_aws_regex_block(&format!(r"\[profile\W{}\][\r\n]+([^\[]+)", profile), &content);
	let region = data.get("region").map(|v| v.to_owned());

	Ok(region)
}

fn parse_aws_regex_block(rgx_str: &str, content: &str) -> HashMap<String, String> {
	let re = Regex::new(&rgx_str).unwrap();
	let caps = re.captures(&content).unwrap();
	let block = caps.get(1).map_or("", |m| m.as_str()).to_owned();
	parse_aws_block(&block)
}

fn parse_aws_block(block: &str) -> HashMap<String, String> {
	let mut data = HashMap::new();

	for line in block.lines() {
		let mut parts = line.splitn(2, "=").map(|s| s.trim());
		let name = parts.next().map(|s| s.trim().to_owned());
		let value = parts.next().map(|s| s.trim().to_owned());
		if let (Some(name), Some(value)) = (name, value) {
			data.insert(name, value);
		}
	}

	data
}

fn read_aws_credentials() -> Result<String, std::io::Error> {
	let aws_config = home_dir().expect("no home").join("./.aws/credentials");
	let content = fs::read_to_string(aws_config)?;
	Ok(content)
}

fn read_aws_config() -> Result<String, std::io::Error> {
	let aws_config = home_dir().expect("no home").join("./.aws/config");
	let content = fs::read_to_string(aws_config)?;
	Ok(content)
}

//// Test assuming some local setup
#[cfg(test)]
mod tests_jc_only {
	use super::*;

	// #[test]
	fn _cred_from_profile() {
		assert!(extract_aws_cred_from_profile("jc-user").is_ok())
	}
}
