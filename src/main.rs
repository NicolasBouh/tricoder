use clap::{App, SubCommand, Arg};
use std::{
    env,
};
use anyhow::Result;

mod cli;
mod error;
mod model;
mod modules;
mod ports;
mod common_ports;
mod subdomains;
pub use error::Error;

fn main() -> Result<()> {
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
        cli::modules();
    } else if let Some(matches) = cli.subcommand_matches("scan") {
        // we can safely unwrap as the argument is required
        let target = matches.value_of("target").unwrap();

        cli::scan(&target)?;
    }

    Ok(())
}
