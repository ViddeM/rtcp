use crate::layers::transport_layer::tcp::control_bits::ControlBits;
use crate::layers::transport_layer::tcp::receive_sequence::ReceiveSequence;
use crate::layers::transport_layer::tcp::send_sequence::SendSequence;
use crate::layers::transport_layer::tcp::states::state_change::TCPStateChange;
use crate::layers::transport_layer::tcp::states::tcp_state::TcpState;
use crate::layers::transport_layer::tcp::tcb::TCB;
use crate::layers::transport_layer::tcp::tcp::TCP;

// const OPTIONS_DATA: [u8; 20] = [0x02, 0x04, 0xff, 0xd7,
//     0x04, 0x02, 0x08, 0x0a, 0xa5, 0xa3, 0x30, 0xed,
//     0xa5, 0xa3, 0x30, 0xed, 0x01, 0x03, 0x03, 0x07];

/// Handle an incoming TCP segment when the connection is in the LISTEN STATE
/// Returns a Result containing either, a tuple containing
/// the new TCB for the connection as well as the TCP response; or a TcpError.
pub fn handle_listen_receive(tcb: &TCB, segment: &TCP) -> eyre::Result<TCPStateChange> {
    if !segment.control_bits.syn {
        eyre::bail!("unexpected connection");
    }

    let mut send_sequence = SendSequence::new_send_sequence(segment.sequence_number);
    let sequence_number = send_sequence.next;

    let new_tcb = TCB {
        local_port: segment.dst_port,
        remote_port: segment.src_port,
        send_sequence: {
            send_sequence.next += 1;
            send_sequence
        },
        receive_sequence: ReceiveSequence {
            next: segment.sequence_number + 1, // SYN takes 1 segment number.
            window: segment.window,
            urgent_pointer: 0,
            initial_receive_sequence: segment.sequence_number,
        },
        state: TcpState::SynReceived,
        send_buffer: tcb.send_buffer.to_owned(),
        receive_buffer: tcb.receive_buffer.to_owned(),
    };

    let options = vec![];

    let response_segment = TCP {
        src_port: new_tcb.local_port,
        dst_port: new_tcb.remote_port,
        sequence_number,
        acknowledgement_number: new_tcb.receive_sequence.next,
        data_offset: 5 + (options.len() as u8), // + 5 for Hardcoded options.
        reserved: 0,
        control_bits: ControlBits::get_syn_ack(),
        window: new_tcb.receive_sequence.window,
        checksum: 0,
        urgent_pointer: 0,
        options,
        data: vec![],
    };

    Ok(TCPStateChange::WithResponse(new_tcb, response_segment))
}
