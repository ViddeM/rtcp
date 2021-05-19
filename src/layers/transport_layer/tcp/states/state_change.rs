use crate::layers::transport_layer::tcp::tcb::TCB;
use crate::layers::transport_layer::tcp::tcp::TCP;

pub enum TCPStateChange {
    WithResponse(TCB, TCP),
    NoResponse(TCB),
}