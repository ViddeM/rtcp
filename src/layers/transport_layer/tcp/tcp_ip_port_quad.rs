use crate::layers::ip_layer::IPAddress;

#[derive(Clone, Debug, Hash)]
pub struct TCPQuad {
    pub src_ip: IPAddress,
    pub dst_ip: IPAddress,
    pub src_port: u16,
    pub dst_port: u16,
}

impl PartialEq for TCPQuad {
    fn eq(&self, other: &Self) -> bool {
        return self.src_port == other.src_port
            && self.dst_port == other.dst_port
            && self.src_ip == other.src_ip
            && self.dst_ip == other.dst_ip;
    }
}

impl Eq for TCPQuad {}
