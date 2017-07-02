use std::fmt;

/// The broadcast MAC address. Used to broadcast to the local network.
pub static BROADCAST_MAC: MacAddr = MacAddr([0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);

/// A MAC address. Six bytes representing a link layer network address.
#[derive(Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct MacAddr(pub [u8; 6]);

impl MacAddr {
    /// Constructs a `MacAddr` from a slice of bytes.
    /// # Panics
    /// This function panics if the given slice is not 6 bytes long.
    pub fn from_slice(slice: &[u8]) -> MacAddr {
        let mut mac = MacAddr::default();
        <[u8; 6] as AsMut<[u8]>>::as_mut(&mut mac.0).copy_from_slice(slice);
        mac
    }

    /// Constructs a `MacAddr` from six individual bytes.
    pub fn from_bytes(b0: u8, b1: u8, b2: u8, b3: u8, b4: u8, b5: u8) -> MacAddr {
        MacAddr([b0, b1, b2, b3, b4, b5])
    }

    /// Creates a `MacAddr` with the broadcast address consisting of all one
    /// bits.
    pub fn broadcast() -> MacAddr {
        BROADCAST_MAC
    }
}

impl AsRef<[u8]> for MacAddr {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl AsRef<[u8; 6]> for MacAddr {
    fn as_ref(&self) -> &[u8; 6] {
        &self.0
    }
}

impl From<[u8; 6]> for MacAddr {
    fn from(data: [u8; 6]) -> Self {
        MacAddr(data)
    }
}

impl fmt::Display for MacAddr {
    /// Format the MAC address with each byte in hexadecimal form and separated
    /// by colons
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
            self.0[4],
            self.0[5]
        )
    }
}

impl fmt::Debug for MacAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self as &fmt::Display).fmt(f)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_slice() {
        let data = [5, 6, 7, 8, 9, 10];
        let mac = MacAddr::from_slice(&data[..]);
        assert_eq!(data, mac.as_ref());
    }

    #[test]
    #[should_panic]
    fn from_short_slice() {
        MacAddr::from_slice(&[1, 2, 3, 4, 5]);
    }

    #[test]
    #[should_panic]
    fn from_long_slice() {
        MacAddr::from_slice(&[1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn from_bytes() {
        let mac = MacAddr::from_bytes(2, 3, 4, 5, 6, 7);
        assert_eq!([2, 3, 4, 5, 6, 7], mac.as_ref());
    }

    #[test]
    fn as_ref() {
        let data = [5, 6, 7, 8, 9, 10];
        let mac = MacAddr(data);
        assert_eq!(data, mac.as_ref());
    }

    #[test]
    fn display() {
        let mac = MacAddr([255, 6, 7, 8, 9, 10]);
        assert_eq!("ff:06:07:08:09:0a", mac.to_string());
    }
}
