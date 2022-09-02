use std::fmt;

/// State of UDP connection.
/// Even though UDP is a stateless protocol, a socket can become connected,
/// meaning it only exchanges datagrams with another specific socket.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UdpState {
    Listen,
    Established,
}

impl fmt::Display for UdpState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UdpState::Listen => "LISTEN",
                UdpState::Established => "ESTABLISHED",
            }
        )
    }
}
