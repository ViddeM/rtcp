use crate::layers::transport::tcp::tcb::{TCB, SendSequence, ReceiveSequence};
use crate::layers::transport::tcp::tcp::TCP;
use crate::layers::transport::tcp::tcp_error::TcpError;
use crate::layers::transport::tcp::states::tcp_state::TcpState;
use crate::layers::transport::tcp::control_bits::ControlBits;

/// Handle an incoming TCP segment when the connection is in the LISTEN STATE
/// Returns a Result containing either, a tuple containing
/// the new TCB for the connection as well as the TCP response; or a TcpError.
pub fn handle_listen_receive(tcb: &TCB, segment: &TCP) -> Result<(TCB, TCP), TcpError>{
    if !tcp.control_bits.syn {
        return Err(TcpError::UnexpectedConnection)
    }

    let new_tcb = TCB {
        local_port: tcb.local_port,
        remote_port: tcb.remote_port,
        send_sequence: SendSequence::new_send_sequence(
            segment.sequence_number,
        ),
        receive_sequence: ReceiveSequence {
            next: segment.sequence_number + 1, // SYN takes 1 segment number.
            window: segment.window,
            urgent_pointer: 0,
            initial_receive_sequence: segment.sequence_number,
        },
        state: TcpState::SynReceived
    };

    let response_segment = TCP {
        src_port: segment.dst_port,
        dst_port: segment.src_port,
        sequence_number: new_tcb.send_sequence.next,
        acknowledgement_number: new_tcb.receive_sequence.next,
        data_offset: 5,
        reserved: 0,
        control_bits: ControlBits::get_syn(),
        window: new_tcb.receive_sequence.window,
        checksum: 0,
        urgent_pointer: 0,
        options: vec![],
        data: vec![]
    };

    Ok((new_tcb, response_segment.add_checksum()))
}