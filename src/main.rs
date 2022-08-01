// #![allow(unused)] // silence unused warnings while exploring (to comment out)

use cmd::cmd_run;

mod cmd;
pub mod consts;
mod error;
mod gen;
mod site;
mod utils;
mod xts;

pub use error::Error;

#[tokio::main(flavor = "current_thread")]
async fn main() {
	match cmd_run().await {
		Ok(_) => println!("✔ All good and well"),
		Err(e) => {
			println!("Error:\n  {}", e)
		}
	};
}
