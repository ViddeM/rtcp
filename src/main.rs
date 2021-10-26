use std::collections::HashMap;

use crate::common::arithmetics::calculate_ones_complement_sum;
use crate::layers::ip_layer::ip_layer::IPLayerProtocol;
use crate::layers::transport_layer::tcp::states::state_change::TCPStateChange;
use crate::layers::transport_layer::tcp::tcb::TCB;
use crate::layers::transport_layer::tcp::tcp::TCP;
use crate::layers::transport_layer::tcp::tcp_ip_port_quad::TCPQuad;
use crate::layers::transport_layer::transport_layer::TransportLayer;
use crate::layers::tun_layer::tun_layer::TunLayer;
use std::io::Error;
use tun_tap::Iface;

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
                        TransportLayer::UDP(udp) => {
                            println!(
                                "UDP: {}\n(Calculated checksum: {:X}",
                                udp,
                                udp.calculate_checksum(
                                    &ipv4.source_address,
                                    &ipv4.destination_address
                                )
                                .expect("Shit happens")
                            );
                        }
                        TransportLayer::TCP(tcp) => {
                            let quad = TCPQuad {
                                src_ip: ipv4.source_address.clone(),
                                dst_ip: ipv4.destination_address.clone(),
                                src_port: tcp.src_port,
                                dst_port: tcp.dst_port,
                            };

                            let result = match connections
                                .entry(quad.clone())
                                .or_default()
                                .on_packet_received(tcp)
                            {
                                Ok(v) => v,
                                Err(e) => {
                                    eprintln!("Failed to handle packet: {}", e);
                                    continue;
                                }
                            };

                            let tcb: TCB;
                            let tcp_opt: Option<TCP>;

                            match result {
                                TCPStateChange::WithResponse(new_tcb, new_tcp) => {
                                    tcb = new_tcb;
                                    tcp_opt = Some(new_tcp);
                                }
                                TCPStateChange::NoResponse(new_tcb) => {
                                    tcb = new_tcb;
                                    tcp_opt = None;
                                }
                            }

                            println!("Now in state: {}", tcb.state);

                            // TODO: Remove
                            let data = tcb.receive_buffer.clone();
                            let len = data.len();
                            if len > 0 {
                                println!(
                                    "{} bytes of data in receive buffer: {}",
                                    len,
                                    String::from_utf8(data).unwrap(),
                                );
                            }

                            connections.insert(quad, tcb);

                            // Does this warrant a response?
                            if let Some(tcp) = tcp_opt {
                                let resp = match ipv4.generate_response(TransportLayer::TCP(tcp)) {
                                    Ok(val) => {
                                        TunLayer::generate_response(IPLayerProtocol::IPv4(val))
                                    }
                                    Err(e) => {
                                        eprintln!("failed to generate ipv4 response: {}", e);
                                        continue;
                                    }
                                };

                                let mut serialized = match resp.serialize() {
                                    Ok(val) => val,
                                    Err(e) => {
                                        eprintln!("Failed to serialize data for send: {}", e);
                                        continue;
                                    }
                                };

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
                        }
                        _ => {}
                    }
                }
                IPLayerProtocol::Other(_) => println!("Unsupported protocol: {}", tun_layer.proto),
            }
        }
    }
}
