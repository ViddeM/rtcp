pub struct ReceiveSequence {
    pub next: u32,
    pub window: u16,
    pub urgent_pointer: u32, // TODO: Figure out datatype.
    pub initial_receive_sequence: u32,
}

impl Default for ReceiveSequence {
    fn default() -> Self {
        ReceiveSequence {
            next: 0,
            window: 0,
            urgent_pointer: 0,
            initial_receive_sequence: 0
        }
    }
}