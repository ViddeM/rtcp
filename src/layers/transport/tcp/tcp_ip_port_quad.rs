use crate::layers::ip_layer::ip_address::IPAddress;

pub struct TCPQuad {
    pub src_ip: IPAddress,
    pub dst_ip: IPAddress,
    pub src_port: u16,
    pub dst_port: u16,
}
