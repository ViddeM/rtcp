use tun_tap::Iface;

use crate::layers::tun_layer::tun_layer::{parse_tun_layer};

mod common;
mod layers;

fn main() {
    let nic = Iface::new("rtcp_tun0", tun_tap::Mode::Tun).expect("failed to setup tun_tap");
    let mut buf = [0u8; 1504];

    loop {
        let n_bytes = nic.recv(&mut buf[..]).expect("failed to receive");
        if let Some(tun_layer) = parse_tun_layer(&mut &buf[..n_bytes]) {
            // match tun_layer.proto {
            //     Protocol::IPv4 => {
            //         if let Some(mut ip_layer) = parse_ip_layer(&mut &*tun_layer.data.as_mut_slice()) {
            //             eprintln!("{}\n{}", tun_layer, ip_layer)
            //         }
            //     }
            //     proto => eprintln!("Unsupported protocol: {}", proto),
            // }
            println!("Parsed: {}", tun_layer)
        }
    }
}
