use crate::layers::transport_layer::tcp::tcb::TCB;
use crate::layers::transport_layer::tcp::tcp::TCP;
use crate::layers::transport_layer::tcp::tcp_error::TcpError;
use crate::layers::transport_layer::tcp::send_sequence::SendSequence;

pub fn handle_syn_received_receive(tcb: &TCB, segment: &TCP) -> Result<(TCB, TCP), TcpError>{
    if !segment.control_bits.ack {
        eprintln!("Missing ack flag");
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

    let options = vec![];

    let response_segment = TCP {
        src_port: segment.dst_port,
        dst_port: segment.src_port,
        sequence_number: new_tcb.send_sequence.next,
        acknowledgement_number: new_tcb.receive_sequence.next,
        data_offset: 5 + (options.len() as u8), // + 5 for Hardcoded options.
        reserved: 0,
        control_bits: ControlBits::get_syn_ack(),
        window: new_tcb.receive_sequence.window,
        checksum: 0,
        urgent_pointer: 0,
        options,
        data: vec![]
    };

    Ok((new_tcb, response_segment))
}