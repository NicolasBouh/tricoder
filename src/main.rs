mod error;
mod model;
mod port;
mod port_common;
mod subdomain;
use std::{env, time::Duration};

pub use error::Error;

use rayon::prelude::*;
use reqwest::{blocking::Client, redirect};

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(Error::CliUsage.into());
    }

    let target = args[1].as_str();

    let http_timeout = Duration::from_secs(5);
    let http_client = Client::builder()
        .redirect(redirect::Policy::limited(4))
        .timeout(http_timeout)
        .build()?;

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(256)
        .build()
        .unwrap();

    pool.install(|| {
        let scan_result = subdomain::enumerate(&http_client, target);
    });

    Ok(())
}
