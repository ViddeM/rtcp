use tun_tap::Iface;

use crate::layers::tun_layer::tun_layer::{parse_tun_layer, Protocol};
use crate::layers::ip_layer::ip_layer::IPLayerProtocol;

mod common;
mod layers;

fn main() {
    let nic = Iface::new("rtcp_tun0", tun_tap::Mode::Tun).expect("failed to setup tun_tap");
    let mut buf = [0u8; 1504];

    loop {
        let n_bytes = nic.recv(&mut buf[..]).expect("failed to receive");
        if let Some(tun_layer) = parse_tun_layer(&mut &buf[..n_bytes]) {
            println!("Parsed: {}", tun_layer);
            // match tun_layer.data {
            //     IPLayerProtocol::IPv4(v) => println!("IPv4: {}", v.to_short_string()),
            //     IPLayerProtocol::Other(_) => println!("Unsupported protocol: {}", tun_layer.proto)
            // }
        }
    }
}
