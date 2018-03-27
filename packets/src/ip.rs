#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Protocol(pub u8);

impl Protocol {
    #[inline]
    pub fn value(&self) -> u8 {
        self.0
    }
}

pub mod protocols {
    use super::Protocol;

    pub const ICMP: Protocol = Protocol(1);
    pub const TCP: Protocol = Protocol(6);
    pub const UDP: Protocol = Protocol(17);
}
