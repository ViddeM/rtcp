// Closed is not represented as it represents the case where there is no state.
pub enum TcpState {
    Listen,
    SynSent,
    SynReceived,
    Established,
    FinWait1,
    FinWait2,
    CloseWait,
    CLosing,
    LastAck,
    TimeWait,
}
