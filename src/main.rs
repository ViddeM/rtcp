use std::collections::HashMap;

use tun_tap::Iface;
use crate::layers::transport_layer::tcp::tcb::TCB;
use crate::layers::transport_layer::tcp::tcp_ip_port_quad::TCPQuad;
use crate::layers::transport_layer::transport_layer::TransportLayer;
use crate::layers::tun_layer::tun_layer::{TunLayer};
use crate::layers::ip_layer::ip_layer::IPLayerProtocol;
use std::io::Error;

mod common;
mod layers;

fn main() {
    let mut connections: HashMap<TCPQuad, TCB> = HashMap::new();

    let nic = Iface::new("rtcp_tun0", tun_tap::Mode::Tun).expect("failed to setup tun_tap");
    let mut buf = [0u8; 1504];

    loop {
        // If n_bytes == 1504 we need to append more data before sending it onwards.
        let n_bytes = nic.recv(&mut buf[..]).expect("failed to receive");
        if let Some(tun_layer) = TunLayer::parse(&mut &buf[..n_bytes]) {
            // println!("Parsed: {}", tun_layer);
            match tun_layer.data {
                IPLayerProtocol::IPv4(ipv4) => {
                    println!("IPv4: {}", ipv4.to_short_string());
                    // println!("IPv4: {}", ipv4);
                    match &ipv4.data {
                        TransportLayer::TCP(tcp) => {
                            let quad = TCPQuad {
                                src_ip: ipv4.source_address.clone(),
                                dst_ip: ipv4.destination_address.clone(),
                                src_port: tcp.src_port,
                                dst_port: tcp.dst_port,
                            };

                            let (tcb, tcp) = match connections.entry(quad.clone())
                                .or_default().on_packet_received(tcp) {
                                Ok(v) => v,
                                Err(e) => {
                                    println!("Failed to handle packet: {}", e);
                                    continue;
                                }
                            };

                            connections.insert(
                                quad,
                                tcb,
                            );

                            let resp = match ipv4.generate_response(TransportLayer::TCP(tcp)) {
                                Ok(val) => TunLayer::generate_response(IPLayerProtocol::IPv4(val)),
                                Err(e) => {
                                    eprintln!("failed to generate ipv4 response: {}", e);
                                    continue;
                                }
                            };

                            let mut serialized = resp.serialize();
                            if let Some(v) = TunLayer::parse(&mut serialized.as_slice()) {
                                if let IPLayerProtocol::IPv4(ipv4) = v.data {
                                    println!("Responding with: {}", ipv4.to_short_string());
                                }
                            }

                            // println!("bytes to send: {:x?}", serialized);

                            match nic.send(serialized.as_slice()) {
                                Ok(v) => println!("successfully responded with {}b", v),
                                Err(e) => println!("failed to send response: {}", e),
                            }
                        }
                        _ => {}
                    }
                }
                IPLayerProtocol::Other(_) => println!("Unsupported protocol: {}", tun_layer.proto)
            }
        }
    }
}
