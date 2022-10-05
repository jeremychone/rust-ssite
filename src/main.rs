#![allow(unused)] // silence unused warnings while exploring (to comment out)
use crate::prelude::*;
use cmd::cmd_run;

mod cmd;
mod consts;
mod error;
mod gen;
mod prelude;
mod site;
mod utils;

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
