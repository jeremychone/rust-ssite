use crate::Error;
use toml::Value;

pub fn toml_as_string(root: &Value, arr: &[&str]) -> Result<String, Error> {
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
