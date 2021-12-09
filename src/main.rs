use clap::{App, SubCommand, Arg};
use futures::{stream, StreamExt};
use reqwest::Client;
use std::{
    env,
    time::{Duration, Instant},
};

mod error;
mod model;
mod ports;
mod common_ports;
mod subdomains;
use model::SubDomain;
pub use error::Error;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env::set_var("RUST_LOG", "info,trust_dns_proto=error");
    env_logger::init();

    let cli = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .subcommand(SubCommand::with_name("modules")
            .about("List add modules")
        )
        .subcommand(SubCommand::with_name("scan")
            .about("Scan an target")
            .arg(
                Arg::with_name("target")
                    .help("The domain name to scan")
                    .required(true)
                    .index(1)
            ),
        )
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .setting(clap::AppSettings::VersionlessSubcommands)
        .get_matches();


    if let Some(_) = cli.subcommand_matches("modules") {
        log::info!("Lauching modules");
    } else if let Some(matches) = cli.subcommand_matches("scan") {
        // we can safely unwrap as the argument is required
        let target = matches.value_of("target").unwrap();

        log::info!("Starting scan on : {}", target);

        scan(&target).await?;

        log::info!("Finishing scan on : {}", target);
    }

    Ok(())
}

async fn scan(target: &str) -> Result<(), anyhow::Error> {
    let http_timeout = Duration::from_secs(10);
    let http_client = Client::builder().timeout(http_timeout).build()?;

    let ports_concurrency = 200;
    let subdomains_concurrency = 100;
    let scan_start = Instant::now();

    let subdomains = subdomains::enumerate(&http_client, target).await?;

    let scan_result: Vec<SubDomain> =  stream::iter(subdomains.into_iter())
        .map(|subdomain| ports::scan_ports( ports_concurrency, subdomain))
        .buffer_unordered(subdomains_concurrency)
        .collect()
        .await;

    // Concurrent stream method 2: Using an Arc<Mutex<T>>
    // let res: Arc<Mutex<Vec<Subdomain>>> = Arc::new(Mutex::new(Vec::new()));

    // stream::iter(subdomains.into_iter())
    //     .for_each_concurrent(subdomains_concurrency, |subdomain| {
    //         let res = res.clone();
    //         async move {
    //             let subdomain = ports::scan_ports(ports_concurrency, subdomain).await;
    //             res.lock().await.push(subdomain)
    //         }
    //     })
    //     .await;

    let scan_duration = scan_start.elapsed();
    println!("Scan completed in {:?}", scan_duration);

    for subdomain in scan_result {
        println!("{}:", &subdomain.domain);
        for port in &subdomain.open_ports {
            println!("    {}: open", port.port);
        }

        println!("");
    }

    Ok(())
}
