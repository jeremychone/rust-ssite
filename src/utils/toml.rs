use crate::prelude::*;
use std::collections::HashSet;
use toml::Value;

// region:    --- Another Approach (one trait for all types)
pub trait DeepGet {
	fn deep_get<'v>(&'v self, arr: &[&str]) -> Option<&'v Value>;
	fn deep_string(&self, arr: &[&str]) -> Result<String>;
	fn deep_str<'v>(&'v self, arr: &[&str]) -> Result<&'v str>;
	fn deep_vec_string(&self, arr: &[&str]) -> Result<Vec<String>>;
}

impl DeepGet for Value {
	fn deep_get<'v>(&'v self, arr: &[&str]) -> Option<&'v Value> {
		let mut value: &Value = self;

		for name in arr {
			value = match value.get(name) {
				Some(v) => v,
				None => return None,
			}
		}

		Some(value)
	}

	fn deep_string(&self, arr: &[&str]) -> Result<String> {
		match self.deep_get(arr).and_then(|v| v.as_str()) {
			Some(str) => Ok(str.to_string()),
			None => Err(Error::TomlMissingValue(arr.join(".").to_string())),
		}
	}

	fn deep_vec_string(&self, arr: &[&str]) -> Result<Vec<String>> {
		match self.deep_get(arr).and_then(|v| v.as_array()) {
			Some(v_arr) => {
				// FIXME: Should return error cannot be as_str()
				let v = v_arr
					.into_iter()
					.map(|v| v.as_str().map(|v| v.to_string()).unwrap_or("".to_string()))
					.collect();
				Ok(v)
			}
			None => Err(Error::TomlMissingValue(arr.join(".").to_string())),
		}
	}

	fn deep_str<'v>(&'v self, arr: &[&str]) -> Result<&'v str> {
		self
			.deep_get(arr)
			.and_then(|v| v.as_str())
			.ok_or_else(|| Error::TomlMissingValue(arr.join(".").to_string()))
	}
}
// endregion: --- Another Approach (one trait for all types)

// region:    --- Old Utilities
pub fn toml_as_string(root: &Value, arr: &[&str]) -> Result<String> {
	toml_as_option_string(&root, arr).ok_or_else(|| Error::MissingConfigProperty(arr.join(".").to_string()))
}

pub fn toml_as_option_string(root: &Value, arr: &[&str]) -> Option<String> {
	let value = toml_as_option_value(root, arr)?;
	match value.as_str() {
		Some(str) => Some(str.to_owned()),
		None => None,
	}
}

pub fn toml_as_option_value<'v>(root: &'v Value, arr: &[&str]) -> Option<&'v Value> {
	let mut value: &Value = root;

	for name in arr {
		value = match value.get(name) {
			Some(v) => v,
			None => return None,
		}
	}

	Some(value)
}
// endregion: --- Old Utilities
