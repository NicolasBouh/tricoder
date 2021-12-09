use futures::{stream, StreamExt};
use reqwest::Client;
use std::{
    time::{Duration, Instant},
};

use crate::{model::SubDomain, Error};
use crate::ports;
use crate::subdomains;

pub fn modules() {
    log::info!("Lauching modules");
}

pub fn scan(target: &str) -> Result<(), Error> {
    log::info!("Scanning on : {}", target);

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Building tokio's runtime");


    let http_timeout = Duration::from_secs(10);
    let http_client = Client::builder().timeout(http_timeout).build()?;

    let ports_concurrency = 200;
    let subdomains_concurrency = 100;
    let scan_start = Instant::now();

    runtime.block_on(async move {
        let subdomains: Vec<SubDomain> = match subdomains::enumerate(&http_client, target).await {
            Ok(s) => s,
            Err(err) => {
                log::error!("subdomains/{}: {}", target, err);
                Vec::<SubDomain>::new()
            }
        };
        
        let subdomains: Vec<SubDomain> =  stream::iter(subdomains.into_iter())
            .map(|subdomain| ports::scan_ports( ports_concurrency, subdomain))
            .buffer_unordered(subdomains_concurrency)
            .collect()
            .await;

        let scan_duration = scan_start.elapsed();
        println!("Scan completed in {:?}", scan_duration);

        for subdomain in &subdomains {
            println!("{}:", &subdomain.domain);
            for port in &subdomain.open_ports {
                println!("    {}: open", port.port);
            }

            println!("");
        }
    });

    let scan_duration = scan_start.elapsed();
    log::info!("Scan completed in {:?}", scan_duration);

    Ok(())
}
