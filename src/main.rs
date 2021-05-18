use tun_tap::Iface;

use crate::layers::tun_layer::tun_layer::{parse_tun_layer, Protocol};
use crate::layers::ip_layer::ip_layer::{IPLayerProtocol, IPv4};
use std::collections::HashMap;
use crate::layers::transport::tcp::tcp_ip_port_quad::TCPQuad;
use crate::layers::transport::tcp::tcb::TCB;
use crate::layers::transport::transport_layer::TransportLayer;
use crate::layers::ip_layer::ip_address::IPAddress;
use crate::layers::transport::tcp::tcp::TCP;
use crate::layers::transport::tcp::tcp_error::TcpError;

mod common;
mod layers;

fn main() {
    let mut connections: HashMap<TCPQuad, TCB> = HashMap::new();

    let nic = Iface::new("rtcp_tun0", tun_tap::Mode::Tun).expect("failed to setup tun_tap");
    let mut buf = [0u8; 1504];

    loop {
        // If n_bytes == 1504 we need to append more data before sending it onwards.
        let n_bytes = nic.recv(&mut buf[..]).expect("failed to receive");
        if let Some(tun_layer) = parse_tun_layer(&mut &buf[..n_bytes]) {
            // println!("Parsed: {}", tun_layer);
            match tun_layer.data {
                IPLayerProtocol::IPv4(ipv4) => {
                    println!("IPv4: {}", ipv4.to_short_string());
                    match ipv4.data {
                        TransportLayer::TCP(tcp) => {
                            let conn = match connections.entry(TCPQuad {
                                src_ip: ipv4.source_address,
                                dst_ip: ipv4.destination_address,
                                src_port: tcp.src_port,
                                dst_port: tcp.dst_port,
                            }).or_default().on_packet_received(tcp) {
                                Ok(v) => v,
                                Err(e) => {
                                    println!("Failed to handle packet: {}", e);
                                    continue;
                                }
                            };


                        }
                        _ => {}
                    }
                }
                IPLayerProtocol::Other(_) => println!("Unsupported protocol: {}", tun_layer.proto)
            }
        }
    }
}

fn generate_tcp_response(tcp: TCP, quad: TCPQuad) -> IPv4 {

}