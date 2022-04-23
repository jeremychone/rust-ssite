use crate::site::{S3Config, Site};
use crate::Error;
use s3::creds::Credentials;
use s3::{Bucket, Region};
use std::str::FromStr;

use super::s3_cred::extract_aws_cred_from_profile;

pub fn get_bucket_client(site: &Site) -> Result<Bucket, Error> {
	let s3_config = site.s3_config().ok_or_else(|| Error::InvalidS3Config)?;

	let S3Config {
		profile,
		bucket_name,
		bucket_root,
	} = s3_config;

	let cred = extract_aws_cred_from_profile(profile)?;

	// Region
	let region = Region::from_str(&cred.region)?;

	let bucket = Bucket::new_with_path_style(
		bucket_name,
		region,
		Credentials {
			access_key: Some(cred.id.to_string()),
			secret_key: Some(cred.secret.to_string()),
			security_token: None,
			session_token: None,
		},
	)?;

	Ok(bucket)
}
