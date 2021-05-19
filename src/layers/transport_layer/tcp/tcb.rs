use crate::layers::transport_layer::tcp::tcp::TCP;
use crate::layers::transport_layer::tcp::states::tcp_state::TcpState;
use crate::layers::transport_layer::tcp::states::listen::handle_listen_receive;
use crate::layers::transport_layer::tcp::tcp_error::TcpError;
use crate::layers::transport_layer::tcp::send_sequence::SendSequence;
use crate::layers::transport_layer::tcp::receive_sequence::ReceiveSequence;
use crate::layers::transport_layer::tcp::states::syn_received::handle_syn_received_receive;
use crate::layers::transport_layer::tcp::states::state_change::TCPStateChange;
use crate::layers::transport_layer::tcp::states::established::handle_established_receive;

// As specified in https://datatracker.ietf.org/doc/html/rfc793#section-3.2
pub struct TCB {
    pub local_port: u16, // Socket number?
    pub remote_port: u16, // Socket number?
    // TODO: Should contain ''The security and precedence of the connection''
    // TODO: Should contain pointer to retransmit queue.
    pub send_sequence: SendSequence,
    pub receive_sequence: ReceiveSequence,
    pub state: TcpState,
    pub send_buffer: Vec<u8>,
    pub receive_buffer: Vec<u8>,
}

impl TCB {
    pub fn on_packet_received(&self, tcp: &TCP) -> Result<TCPStateChange, TcpError> {
        match &self.state {
            TcpState::Listen => handle_listen_receive(self, tcp),
            TcpState::SynReceived => handle_syn_received_receive(self, tcp),
            TcpState::Established => handle_established_receive(self, tcp),
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
            state: TcpState::Listen,
            send_buffer: vec![],
            receive_buffer: vec![]
        }
    }
}
