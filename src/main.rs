#![allow(unused)] // silence unused warnings while exploring (to comment out)
use cmd::cmd_run;

mod cmd;
pub mod consts;
mod error;
mod gen;
mod site;
mod utils;

pub use error::Error;

#[tokio::main]
async fn main() {
	let d = s!("some");
	match cmd_run().await {
		Ok(_) => println!("✔ All good and well"),
		Err(e) => {
			println!("Error:\n  {}", e)
		}
	};
}

pub use std::format as f;

#[macro_export]
macro_rules! s {
	() => {
		String::new()
	};
	($x:expr $(,)?) => {
		ToString::to_string(&$x)
	};
}
