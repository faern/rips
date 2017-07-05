macro_rules! packet {
    ($name:ident, $mut_name:ident, $min_len:expr) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        pub struct $name<'a>(&'a [u8]);
        pub struct $mut_name<'a>(&'a mut [u8]);

        impl<'a> $name<'a> {
            /// Creates a new immutable packet based on the given backing slice. Returns `None` if
            /// the buffer is shorter than the minimum length of this packet.
            #[inline]
            pub fn new(data: &'a [u8]) -> Option<$name<'a>>
            {
                if data.len() >= $min_len {
                    Some($name(data))
                } else {
                    None
                }
            }

            /// Creates a new immutable packet based on the given backing slice without checking
            /// its length first. If the slice is too short, a subsequent read from a field might
            /// result in invalid memory access.
            #[inline]
            pub unsafe fn new_unchecked(data: &'a [u8]) -> $name<'a> {
                $name(data)
            }

            /// Returns the minimum number of bytes in this type of packet. Usually equal to the
            /// header size of this protocol.
            #[inline]
            pub fn min_len() -> usize {
                $min_len
            }

            /// Returns the number of bytes in this packet. This is simply the length of the byte
            /// slice backing the packet instance.
            #[inline]
            pub fn len(&self) -> usize {
                self.0.len()
            }

            /// Returns a reference to the slice backing this packet.
            #[inline]
            pub fn data(&self) -> &[u8] {
                self.0
            }

            /// Returns a slice to the part of the backing data that represents the header.
            /// This is simply everything up until `min_len()`.
            #[inline]
            pub fn header(&self) -> &[u8] {
                &self.0[..$min_len]
            }

            /// Returns a slice to the payload part of the backing data. This is simply everything
            /// after the header.
            #[inline]
            pub fn payload(&self) -> &[u8] {
                &self.0[$min_len..]
            }
        }

        impl<'a> $mut_name<'a> {
            /// Creates a new mutable packet based on the given backing slice. Returns `None` if
            /// the buffer is shorter than the minimal length of this packet.
            #[inline]
            pub fn new(data: &'a mut [u8]) -> Option<$mut_name<'a>>
            {
                if data.len() >= $min_len {
                    Some($mut_name(data))
                } else {
                    None
                }
            }

            /// Creates a new mutable packet based on the given backing slice without checking
            /// its length first. If the slice is too short, a subsequent read from a field might
            /// result in invalid memory access.
            #[inline]
            pub unsafe fn new_unchecked(data: &'a mut [u8]) -> $mut_name<'a> {
                $mut_name(data)
            }

            /// Returns an immutable version of the same packet and backed by the same byte slice.
            /// Used to access the getters.
            #[inline]
            pub fn as_immutable(&'a self) -> $name<'a> {
                $name(&self.0[..])
            }

            /// Returns the minimum number of bytes in this type of packet. Usually equal to the
            /// header size of this protocol.
            #[inline]
            pub fn min_len() -> usize {
                $min_len
            }

            /// Returns the number of bytes in this packet. This is simply the length of the byte
            /// slice backing the packet instance.
            #[inline]
            pub fn len(&self) -> usize {
                self.0.len()
            }

            /// Returns a mutable reference to the slice backing this packet.
            #[inline]
            pub fn data(&mut self) -> &mut [u8] {
                self.0
            }

            /// Returns a mutable slice to the part of the backing data that represents the header.
            /// This is simply everything up until `min_len()`.
            #[inline]
            pub fn header(&mut self) -> &mut [u8] {
                &mut self.0[..$min_len]
            }

            /// Returns a mutable slice to the payload part of the backing data. This is simply
            /// everything after the header.
            #[inline]
            pub fn payload(&mut self) -> &mut [u8] {
                &mut self.0[$min_len..]
            }
        }
    }
}

macro_rules! getters {
    ($pkg:ident
    $(
        $(#[$doc: meta])*
        pub fn $name:ident(&$selff:ident) -> $type:ty $body:block
    )*) => {
        impl<'a> $pkg<'a> {
            $($(#[$doc])*
            #[inline]
            pub fn $name(&$selff) -> $type {
                $body
            })*
        }
    }
}

macro_rules! setters {
    ($pkg:ident
    $(
        $(#[$doc: meta])*
        pub fn $name:ident(&mut $selff:ident, $arg:ident: $type:ty) $body:block
    )*) => {
        impl<'a> $pkg<'a> {
            $($(#[$doc])*
            #[inline]
            pub fn $name(&mut $selff, $arg: $type) {
                $body
            })*
        }
    }
}


macro_rules! read_offset {
    ($buff:expr, $offset:expr, $type:ty) => {{
        let ptr = &$buff[$offset];
        unsafe { *(ptr as *const _ as *const $type) }
    }};
    ($buff:expr, $offset:expr, $type:ident, from_be) => {{
        $type::from_be(read_offset!($buff, $offset, $type))
    }}
}

macro_rules! write_offset {
    ($buff:expr, $offset:expr, $value:expr, $type:ty) => {{
        let ptr = (&mut $buff[$offset]) as *mut _ as *mut $type;
        unsafe { *ptr = $value };
    }};
    ($buff:expr, $offset:expr, $value:expr, $type:ident, to_be) => {{
        write_offset!($buff, $offset, $type::to_be($value), $type)
    }}
}

/// Creates a test for the `$set_name` setter and `$name` getter of packet type
/// `$packet`. First calls `$set_name` with `$value`. Then makes sure the
/// `$name` getter returns `$value` again. Lastly it checks so that the only
/// bits in the backing buffer that are non-zero are at `$offset` and contain
/// `$expected`.
#[allow(unused_macros)]
macro_rules! setget_test {
    ($packet:ident,
     $name:ident,
     $set_name:ident,
     $value:expr,
     $offset:expr,
     $expected:expr) => {
        #[test]
        fn $name() {
            let mut backing_data = [0; 1024];
            {
                let mut testee = $packet::new(&mut backing_data).unwrap();
                testee.$set_name($value);
                // Check that the getter returns the same value
                assert_eq!($value, testee.as_immutable().$name());
            }
            // Check that only the intended bytes were affected
            let start = $offset;
            let end = $offset + $expected.len();
            assert!(backing_data[0..start].iter().all(|&v| v == 0x00));
            assert_eq!($expected, backing_data[start..end]);
            assert!(backing_data[end..].iter().all(|&v| v == 0x00));
        }
    }
}
