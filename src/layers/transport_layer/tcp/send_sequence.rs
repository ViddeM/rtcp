use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
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
    // Sequence number generation as described in: https://datatracker.ietf.org/doc/html/rfc793#page-27
    fn generate_initial_send_sequence_number() -> u32 {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time set to before UNIX EPOCH!")
            .as_micros();

        return (time / 4) as u32; // should update every 4 microseconds.
    }

    pub fn new_send_sequence(rcv_seq: u32) -> SendSequence {
        let iss = SendSequence::generate_initial_send_sequence_number();

        SendSequence {
            unacknowledged: iss,
            next: iss,
            window: 1024,  // Should follow some cool algorithm but hardcoded for now, should be fine™.🕶
            urgent_pointer: 0,
            last_window_update_sequence: rcv_seq,
            last_window_update_ack: rcv_seq,
            initial_send_sequence: iss
        }
    }
}

impl Default for SendSequence {
    fn default() -> Self {
        SendSequence {
            unacknowledged: 0,
            next: 0,
            window: 0,
            urgent_pointer: 0,
            last_window_update_sequence: 0,
            last_window_update_ack: 0,
            initial_send_sequence: 0
        }
    }
}