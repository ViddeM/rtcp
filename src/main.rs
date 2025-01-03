use std::collections::HashMap;

use crate::layers::ip_layer::ip_layer::IPLayerProtocol;
use crate::layers::transport_layer::tcp::states::state_change::TCPStateChange;
use crate::layers::transport_layer::tcp::tcb::TCB;
use crate::layers::transport_layer::tcp::tcp_ip_port_quad::TCPQuad;
use crate::layers::transport_layer::transport_layer::TransportLayer;
use crate::layers::tun_layer::tun_layer::TunLayer;
use colored::Colorize;
use common::proto::Proto;
use eyre::Context;
use layers::ip_layer::IPAddress;
use tun_tap::Iface;

mod common;
mod layers;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let mut connections: HashMap<TCPQuad, TCB> = HashMap::new();

    let nic =
        Iface::new("rtcp_tun0", tun_tap::Mode::Tun).wrap_err("failed to setup tun interface")?;

    let mut buf = [0u8; 1504];
    loop {
        // If n_bytes == 1504 we need to append more data before sending it onwards.
        let n_bytes = nic
            .recv(&mut buf[..])
            .wrap_err("failed to receive packet")?;

        let tun_layer =
            TunLayer::parse(&mut &buf[..n_bytes]).wrap_err("failed to parse tun layer")?;

        if let Some(resp) =
            handle_tun_layer(tun_layer, &mut connections).wrap_err("failed parsing ip layer")?
        {
            send_response(&nic, resp).wrap_err("failed to send response")?;
        }
    }
}

fn handle_tun_layer(
    tun_layer: TunLayer,
    connections: &mut HashMap<TCPQuad, TCB>,
) -> eyre::Result<Option<TunLayer>> {
    let response: Option<IPLayerProtocol> = match tun_layer.data {
        IPLayerProtocol::IPv6(ipv6) => {
            println!("{}", ipv6.to_short_string());
            match handle_transport_layer(
                &ipv6.data,
                connections,
                ipv6.source_address.clone().into(),
                ipv6.destination_address.clone().into(),
            )
            .wrap_err("handling ipv6 packaet")?
            {
                Some(response) => {
                    let response = ipv6
                        .generate_response(response)
                        .wrap_err("failed generating an ipv6 response")?;
                    Some(response.into())
                }
                None => None,
            }
        }
        IPLayerProtocol::IPv4(ipv4) => {
            println!("{}", ipv4.to_short_string());

            match handle_transport_layer(
                &ipv4.data,
                connections,
                ipv4.source_address.clone().into(),
                ipv4.destination_address.clone().into(),
            )
            .wrap_err("handling ipv4 packet")?
            {
                Some(response) => {
                    let response = ipv4
                        .generate_response(response)
                        .wrap_err("failed generating an ipv4 response")?;

                    Some(response.into())
                }
                None => None,
            }
        }
        IPLayerProtocol::Other(_) => {
            println!(
                "Unsupported protocol: {}",
                tun_layer.proto.to_string().red()
            );
            None
        }
    };

    Ok(match response {
        Some(resp) => Some(TunLayer::generate_response(resp)),
        None => None,
    })
}

fn handle_transport_layer(
    data: &TransportLayer,
    connections: &mut HashMap<TCPQuad, TCB>,
    source_address: IPAddress,
    destination_address: IPAddress,
) -> eyre::Result<Option<TransportLayer>> {
    match data {
        TransportLayer::UDP(_udp) => {}
        TransportLayer::TCP(tcp) => {
            let quad = TCPQuad {
                src_ip: source_address.clone(),
                dst_ip: destination_address.clone(),
                src_port: tcp.src_port,
                dst_port: tcp.dst_port,
            };

            /*
                        let result = connections
                            .entry(quad.clone())
                            .or_default()
                            .on_packet_received(&tcp)
                            .wrap_err("receiving TCP package")?;
            */

            let result = match connections
                .entry(quad.clone())
                .or_default()
                .on_packet_received(&tcp)
                .wrap_err("receiving TCP package")
            {
                Ok(r) => r,
                Err(err) => {
                    eprintln!("{}", err);
                    return Ok(None);
                }
            };

            let (tcb, tcp_opt) = match result {
                TCPStateChange::WithResponse(new_tcb, new_tcp) => (new_tcb, Some(new_tcp)),
                TCPStateChange::NoResponse(new_tcb) => (new_tcb, None),
            };

            println!("\tnow in state: {}", tcb.state.to_string().yellow());

            connections.insert(quad, tcb);

            // Does this warrant a response?
            if let Some(tcp_response) = tcp_opt {
                return Ok(Some(TransportLayer::TCP(tcp_response)));
            }
        }
        _ => {}
    }

    Ok(None)
}

fn send_response(nic: &Iface, response: TunLayer) -> eyre::Result<()> {
    let mut serialized = response
        .serialize()
        .wrap_err("failed serializing tun_layer response")?;

    // Ensure that we can parse the serialized response.
    match TunLayer::parse(&mut serialized.as_slice()) {
        Ok(resp) => println!("\tresponding with {}", resp.to_short_string()),
        Err(err) => eyre::bail!("Failed to parse response, this means we can generate responses we ourselves cannot parse (BAD!), err: {err}"),
    }

    match nic.send(serialized.as_slice()) {
        Ok(v) => println!("successfully responded with {}b", v),
        Err(e) => println!("failed to send response: {}", e),
    }

    Ok(())
}
