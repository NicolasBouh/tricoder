use crate::{
    model::{Port, SubDomain},
    port_common::MOST_COMMON_PORTS_100,
};
use std::{
    net::{SocketAddr, TcpStream, ToSocketAddrs},
    time::Duration,
};

pub fn scan_ports(mut subdomain: SubDomain) -> SubDomain {
    subdomain.open_ports = MOST_COMMON_PORTS_100
        .into_iter()
        .map(|port| scan_port(&subdomain.domain, *port))
        .filter(|port| port.is_open)
        .collect();

    subdomain
}

fn scan_port(hostname: &str, port: u16) -> Port {
    let timeout = Duration::from_secs(3);
    let socket_addresses: Vec<SocketAddr> = format!("{}:{}", hostname, port)
        .to_socket_addrs()
        .expect("port scanner: Creating socket address")
        .collect();

    if socket_addresses.len() == 0 {
        return Port {
            port: port,
            is_open: false,
        };
    }

    let is_open = if let Ok(_) = TcpStream::connect_timeout(&socket_addresses[0], timeout) {
        true
    } else {
        false
    };

    Port { port, is_open }
}
