// --- re-exports
pub use crate::error::Error;
pub use std::format as f;

// --- Generic Wrapper newtype pattern, mostly for external type to type From/TryFrom conversions
pub struct W<T>(pub T);

// --- Personal preference
macro_rules! s {
	() => {
		String::new()
	};
	($x:expr $(,)?) => {
		ToString::to_string(&$x)
	};
}
pub(crate) use s; // export macro for crate
