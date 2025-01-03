use crate::layers::transport_layer::tcp::receive_sequence::ReceiveSequence;
use crate::layers::transport_layer::tcp::states::state_change::TCPStateChange;
use crate::layers::transport_layer::tcp::states::tcp_state::TcpState;
use crate::layers::transport_layer::tcp::tcb::TCB;
use crate::layers::transport_layer::tcp::tcp::TCP;

pub fn handle_syn_received_receive(tcb: &TCB, segment: &TCP) -> eyre::Result<TCPStateChange> {
    if !segment.control_bits.ack {
        eyre::bail!("missing ack flag");
    }

    // TODO: handle if they sent data with the ack.

    let new_tcb = TCB {
        local_port: tcb.local_port,
        remote_port: tcb.remote_port,
        send_sequence: tcb.send_sequence.clone(),
        receive_sequence: ReceiveSequence {
            next: segment.sequence_number, // SYN takes 1 segment number.
            window: segment.window,
            urgent_pointer: 0, // TODO: Implement
            initial_receive_sequence: tcb.receive_sequence.initial_receive_sequence,
        },
        state: TcpState::Established,
        send_buffer: tcb.send_buffer.to_owned(),
        receive_buffer: tcb.receive_buffer.to_owned(),
    };

    Ok(TCPStateChange::NoResponse(new_tcb))
}
