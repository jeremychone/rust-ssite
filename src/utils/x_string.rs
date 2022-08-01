///! as_string  trait/implementations
///! ----
///
use std::ffi::OsStr;
use std::fs::DirEntry;
use std::path::PathBuf;

pub trait XString {
	fn x_string(&self) -> Option<String>;
}

pub trait XStr {
	fn x_str(&self) -> Option<&str>;
}

pub trait DispStr {
	/// Return the &str of the type or Option<T> or &'static "" if None
	fn disp_str(&self) -> &str;
}

// region:    --- OsStr
impl XString for OsStr {
	#[inline]
	fn x_string(&self) -> Option<String> {
		self.to_str().map(|s| s.to_string())
	}
}

impl XString for Option<&OsStr> {
	#[inline]
	fn x_string(&self) -> Option<String> {
		self.and_then(|s| s.to_str()).map(|s| s.to_string())
	}
}

impl XStr for Option<&OsStr> {
	#[inline]
	fn x_str(&self) -> Option<&str> {
		self.and_then(|s| s.to_str())
	}
}

// endregion: --- OsStr

// region:    --- String
impl XStr for Option<String> {
	fn x_str(&self) -> Option<&str> {
		self.as_ref().map(|v| v.as_str())
	}
}
// endregion: --- String

// region:    --- PathBuf
impl XString for PathBuf {
	#[inline]
	fn x_string(&self) -> Option<String> {
		self.to_str().map(|v| v.to_string())
	}
}

impl XString for Option<PathBuf> {
	#[inline]
	fn x_string(&self) -> Option<String> {
		match self {
			Some(path) => XString::x_string(path),
			None => None,
		}
	}
}

impl XStr for PathBuf {
	#[inline]
	fn x_str(&self) -> Option<&str> {
		self.to_str()
	}
}

impl XStr for Option<&PathBuf> {
	#[inline]
	fn x_str(&self) -> Option<&str> {
		match self {
			Some(path) => PathBuf::x_str(path),
			None => None,
		}
	}
}

impl DispStr for Option<PathBuf> {
	fn disp_str(&self) -> &str {
		self.as_ref().and_then(|p| p.to_str()).unwrap_or("")
	}
}

impl DispStr for Option<&PathBuf> {
	fn disp_str(&self) -> &str {
		self.and_then(|p| p.to_str()).unwrap_or("")
	}
}
// endregion: --- PathBuf

// region:    --- DirEntry
impl XString for DirEntry {
	#[inline]
	fn x_string(&self) -> Option<String> {
		self.path().to_str().map(|s| s.to_string())
	}
}

impl XString for Option<DirEntry> {
	#[inline]
	fn x_string(&self) -> Option<String> {
		self.as_ref().and_then(|v| DirEntry::x_string(v))
	}
}
// endregion: --- DirEntry
