use crate::publish::s3_cred::extract_aws_cred_from_profile;
use crate::publish::s3_utils::get_bucket_client;
use crate::site::{S3Config, Site};
use crate::Error;
use pathdiff::diff_paths;
use s3::serde_types::Object;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::Path;

mod s3_cred;
mod s3_utils;

pub async fn publish(site: &Site) -> Result<(), Error> {
	let bucket = get_bucket_client(site)?;
	let S3Config { bucket_root, .. } = site.s3_config().ok_or(Error::InvalidS3Config)?;

	let mut content_key_set: HashSet<String> = HashSet::new();

	// - LIST what is present
	println!("---- LIST WHAT IS PRESENT ");
	let results = bucket.list(bucket_root.to_owned(), None).await?;
	for result in results {
		for item in result.contents {
			println!("s3 key: {}", item.key);
		}
	}
	println!("---- /LIST WHAT IS PRESENT ");

	// - UPLOAD updated data
	println!("\n---- UPLOAD DATA ");
	for entry in site.content_entries() {
		let file = entry.path();
		// TODO - should exists a more elegant way to do the same
		if let Some(key) = diff_paths(file, site.content_dir())
			.map(|p| p.to_str().map(|s| s.to_string()))
			.flatten()
		{
			println!("uploading: {}", key);

			let mime_type = mime_guess::from_path(file).first_or_octet_stream().to_string();

			let mut file_obj = File::open(&file)?;
			let mut buffer = Vec::new();
			file_obj.read_to_end(&mut buffer)?;
			bucket.put_object_with_content_type(&key, &buffer, &mime_type).await?;

			content_key_set.insert(key);
		}
	}
	println!("---- /UPLOAD DATA ");

	// - CLEAN S3
	println!("\n---- CLEAN S3 ");
	let results = bucket.list(bucket_root.to_owned(), None).await?;
	for result in results {
		for Object { key, .. } in result.contents {
			if !content_key_set.contains(&key) {
				println!("deleting {}", key);
				bucket.delete_object(key).await?;
			}
		}
	}
	println!("---- /CLEAN S3 ");

	Ok(())
}
