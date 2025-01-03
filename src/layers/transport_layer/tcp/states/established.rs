use crate::layers::transport_layer::tcp::control_bits::ControlBits;
use crate::layers::transport_layer::tcp::receive_sequence::ReceiveSequence;
use crate::layers::transport_layer::tcp::states::state_change::TCPStateChange;
use crate::layers::transport_layer::tcp::states::tcp_state::TcpState;
use crate::layers::transport_layer::tcp::tcb::TCB;
use crate::layers::transport_layer::tcp::tcp::TCP;

pub fn handle_established_receive(tcb: &TCB, segment: &TCP) -> eyre::Result<TCPStateChange> {
    if !segment.control_bits.ack {
        eyre::bail!("missing ack flag");
    }

    let mut new_buffer = tcb.receive_buffer.to_owned();
    let next_sequence_number;

    if segment.sequence_number == tcb.receive_sequence.next {
        new_buffer.extend_from_slice(segment.data.as_slice());
        next_sequence_number = segment
            .sequence_number
            .overflowing_add(segment.data.len() as u32)
            .0; // TODO: Handle if segment.data.len() > u32 max value
    } else if segment.sequence_number < tcb.receive_sequence.next
        && tcb.receive_sequence.next < segment.sequence_number + (segment.data.len() as u32)
    {
        let acked_data = tcb.receive_sequence.next - segment.sequence_number;
        let new_data = &segment.data[(acked_data as usize)..];
        new_buffer.extend_from_slice(new_data);
        next_sequence_number = tcb.receive_sequence.next + acked_data;
    } else {
        next_sequence_number = tcb.receive_sequence.next;
    }

    let new_tcb = TCB {
        local_port: tcb.local_port,
        remote_port: tcb.remote_port,
        send_sequence: tcb.send_sequence.clone(),
        receive_sequence: ReceiveSequence {
            next: next_sequence_number,
            window: segment.window,
            urgent_pointer: 0, // TODO: Implement
            initial_receive_sequence: tcb.receive_sequence.initial_receive_sequence,
        },
        state: TcpState::Established,
        send_buffer: tcb.send_buffer.to_owned(),
        receive_buffer: new_buffer,
    };

    let options = vec![];

    let new_tcp = TCP {
        src_port: new_tcb.local_port,
        dst_port: new_tcb.remote_port,
        sequence_number: new_tcb.send_sequence.next,
        acknowledgement_number: new_tcb.receive_sequence.next,
        data_offset: 5 + (options.len() as u8),
        reserved: 0,
        control_bits: ControlBits::get_ack(),
        window: new_tcb.receive_sequence.window,
        checksum: 0,
        urgent_pointer: 0,
        options,
        data: vec![],
    };

    Ok(TCPStateChange::WithResponse(new_tcb, new_tcp))
}
