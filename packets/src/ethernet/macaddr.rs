use std::fmt;
use std::str::FromStr;
use std::error::Error;

/// The broadcast MAC address. Used to broadcast to the local network.
pub static BROADCAST_MAC: MacAddr = MacAddr([0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);

#[derive(Debug, Eq, PartialEq)]
pub struct MacAddrLengthError;

impl fmt::Display for MacAddrLengthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

impl Error for MacAddrLengthError {
    fn description(&self) -> &str {
        "Given data not six bytes long"
    }
}

/// A MAC address. Six bytes representing a link layer network address.
#[derive(Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct MacAddr(pub [u8; 6]);

impl MacAddr {
    /// Constructs a `MacAddr` from a slice of bytes.
    /// Will fail if the given slice is not 6 bytes long.
    pub fn try_from_slice(slice: &[u8]) -> Result<MacAddr, MacAddrLengthError> {
        if slice.len() == 6 {
            Ok(Self::from_slice(slice))
        } else {
            Err(MacAddrLengthError)
        }
    }

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

#[derive(Debug, Eq, PartialEq)]
pub struct MacAddrParseError(String);

impl fmt::Display for MacAddrParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid MAC string: {}", self.0)
    }
}

impl Error for MacAddrParseError {
    fn description(&self) -> &str {
        "Invalid MAC address string"
    }
}

impl FromStr for MacAddr {
    type Err = MacAddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use std::num::ParseIntError;

        let bytes: Result<Vec<u8>, ParseIntError> =
            s.split(":").map(|s| u8::from_str_radix(s, 16)).collect();
        match bytes {
            Ok(ref bytes) if bytes.len() == 6 => Ok(Self::from_slice(&bytes)),
            _ => Err(MacAddrParseError(s.to_owned())),
        }
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

    #[test]
    fn from_str() {
        let result = MacAddr::from_str("01:02:ff:ac:13:37");
        assert_eq!(result, Ok(MacAddr([0x01, 0x02, 0xff, 0xac, 0x13, 0x37])));
    }
}
