use crate::{
    model::{Port, SubDomain}, common_ports::MOST_COMMON_PORTS_100
};
use std::{
    net::{SocketAddr, ToSocketAddrs},
    time::Duration,
};
use futures::{stream, StreamExt};
use tokio::{net::TcpStream};

pub async fn scan_ports(concurrency: usize, subdomain: SubDomain) -> SubDomain {
    println!(" Starting to scan subdomain : {}", subdomain.domain);

    let mut ret = subdomain.clone();

    /*let scan_result: Vec<SubDomain> = stream::iter(MOST_COMMON_PORTS_100.into_iter())
        .filter(async |port| {
            let port = scan_port(&subdomain.domain, port).await;
            return port.is_open;

        })
        .buffer_unordered(MOST_COMMON_PORTS_100.len())
        .collect()
        .await;*/

    let scan_result: Vec<Port> = stream::iter(MOST_COMMON_PORTS_100.into_iter())
        .map(|port| scan_port(&subdomain.domain, *port))
        .buffer_unordered(concurrency)
        .collect()
        .await;

    let open_ports: Vec<Port> = scan_result.into_iter()
    .filter(|port| port.is_open)
    .collect();

    ret.open_ports = open_ports;

    ret
}

// Concurrent stream method 3: using channels
/*pub async fn scan_ports(concurrency: usize, subdomain: SubDomain) -> SubDomain {
    println!(" Starting to scan subdomain : {}", subdomain.domain);

    let mut ret = subdomain.clone();

    // Concurrent stream method 3: using channels
    let (input_tx, input_rx) = mpsc::channel(concurrency);
    let (output_tx, output_rx) = mpsc::channel(concurrency);

    let input_rx_stream = tokio_stream::wrappers::ReceiverStream::new(input_rx);
    input_rx_stream
        .for_each_concurrent(concurrency, |port| {
            let subdomain = subdomain.clone();
            let output_tx = output_tx.clone();
            async move {
                let port = scan_port(&subdomain.domain, port).await;
                if port.is_open {
                    let _ = output_tx.send(port).await;
                }
            }
        })
        .await;

    // close channel
    drop(output_tx);

    let output_rx_stream = tokio_stream::wrappers::ReceiverStream::new(output_rx);
    ret.open_ports = output_rx_stream.collect().await;

    ret
}*/

async fn scan_port(hostname: &str, port: u16) -> Port {
    println!("scanning port {} :{}", hostname, port);

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

    let is_open = match tokio::time::timeout(timeout, TcpStream::connect(&socket_addresses[0])).await {
        Ok(Ok(_)) => true,
        _ => false,
    };

    Port { port, is_open }
}
