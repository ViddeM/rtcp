use crate::layers::transport_layer::tcp::tcp::TCP;
use crate::layers::transport_layer::tcp::states::tcp_state::TcpState;
use crate::layers::transport_layer::tcp::states::listen::handle_listen_receive;
use crate::layers::transport_layer::tcp::tcp_error::TcpError;
use crate::layers::transport_layer::tcp::send_sequence::SendSequence;
use crate::layers::transport_layer::tcp::receive_sequence::ReceiveSequence;

// As specified in https://datatracker.ietf.org/doc/html/rfc793#section-3.2
pub struct TCB {
    pub local_port: u16, // Socket number?
    pub remote_port: u16, // Socket number?
    // TODO: Should contain ''The security and precedence of the connection''
    // TODO: Should contain pointers to users send and receive buffers.
    // TODO: Should contain pointer to retransmit queue.
    pub send_sequence: SendSequence,
    pub receive_sequence: ReceiveSequence,
    pub state: TcpState,
}

impl TCB {
    pub fn on_packet_received(&self, tcp: &TCP) -> Result<(TCB, TCP), TcpError> {
        match &self.state {
            TcpState::Listen => {
                handle_listen_receive(self, tcp)
            }
            state => Err(TcpError::NotSupported(state.to_string())),
        }
    }
}

impl Default for TCB {
    fn default() -> Self {
        TCB {
            local_port: 0,
            remote_port: 0,
            send_sequence: SendSequence::default(),
            receive_sequence: ReceiveSequence::default(),
            state: TcpState::Listen
        }
        
    }
}
