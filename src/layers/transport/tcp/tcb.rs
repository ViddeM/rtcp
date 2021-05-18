use crate::layers::transport::tcp::tcp::TCP;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::layers::transport::tcp::states::tcp_state::TcpState;
use crate::layers::transport::tcp::states::listen::handle_listen_receive;
use crate::layers::transport::tcp::tcp_error::TcpError;

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
    pub fn on_packet_received(&self, tcp: TCP) -> Result<(TCB, TCP), TcpError> {
        match self.state {
            TcpState::Listen => {
                handle_listen_receive(self, &tcp)
            }
            TcpState::SynSent => todo!("Not implemented"),
            TcpState::SynReceived => todo!("Not implemented"),
            TcpState::Established => todo!("Not implemented"),
            TcpState::FinWait1 => todo!("Not implemented"),
            TcpState::FinWait2 => todo!("Not implemented"),
            TcpState::CloseWait => todo!("Not implemented"),
            TcpState::CLosing => todo!("Not implemented"),
            TcpState::LastAck => todo!("Not implemented"),
            TcpState::TimeWait => todo!("Not implemented"),
        }
    }
}

pub struct SendSequence {
    pub unacknowledged: u32,
    pub next: u32,
    pub window: u16,
    pub urgent_pointer: u32, // TODO: Figure out what type this one should have.
    pub last_window_update_sequence: u32, // WL1
    pub last_window_update_ack: u32, // WL2
    pub initial_send_sequence: u32,
}

impl SendSequence {
    fn generate_initial_send_sequence_number() -> u32 {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time set to before UNIX EPOCH!")
            .as_micros();

        // Sequence number generation as described in: https://datatracker.ietf.org/doc/html/rfc793#page-27
        return (time.overflowing_div(4) // Should be fine with overflow, should update every 4 microseconds.
            .0 % u32::MAX) as u32;
    }

    pub fn new_send_sequence(rcv_seq: u32) -> SendSequence {
        let iss = SendSequence::generate_initial_send_sequence_number();

        SendSequence {
            unacknowledged: iss,
            next: iss,
            window: 1024,  // Should follow some cool algorithm but hardcoded for now, should be fine™.
            urgent_pointer: 0,
            last_window_update_sequence: rcv_seq,
            last_window_update_ack: rcv_seq,
            initial_send_sequence: iss
        }
    }
}

pub struct ReceiveSequence {
    pub next: u32,
    pub window: u16,
    pub urgent_pointer: u32, // TODO: Figure out datatype.
    pub initial_receive_sequence: u32,
}
