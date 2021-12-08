mod error;
mod subdomain;
mod model;
pub use error::Error;

fn main() -> Result<(), anyhow::Error> {
    println!("Hello, world!");

    Ok(())
}
