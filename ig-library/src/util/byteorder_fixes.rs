// Fixes a few issues with the "byteorder" crate that stem from poor design

use std::io::{Cursor, ErrorKind, Read};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use crate::core::fs::Endian;
use paste::paste;

// Endian is ignored here so it needs a custom implementation
#[inline]
pub fn read_u8(cursor: &mut Cursor<Vec<u8>>, _endian: &Endian) -> std::io::Result<u8> {
    cursor.read_u8()
}

macro_rules! define_read {
    ($type:ty) => {
        paste! {
            #[inline]
            pub fn [<read_ $type>](cursor: &mut Cursor<Vec<u8>>, endian: &Endian) -> std::io::Result<$type> {
                match endian {
                    Endian::Little => cursor.[<read_ $type>]::<LittleEndian>(),
                    Endian::Big => cursor.[<read_ $type>]::<BigEndian>(),
                    Endian::Unknown => Err(std::io::Error::new(
                        ErrorKind::InvalidInput,
                        "Endianness not set",
                    )),
                }
            }
        }
    };
}

pub fn read_struct_array_u8<'a>(
    cursor: &mut Cursor<Vec<u8>>,
    _endian: &Endian,
    count: usize,
) -> std::io::Result<Vec<u8>> {
    let mut buf = vec![0u8; count];
    cursor.read_exact(&mut buf)?; // advance the position internally. Strange performance issue here
    Ok(buf)
}


macro_rules! define_read_struct_array {
    ($($typ:ident),*) => {
        $(
            paste::paste! {
                pub fn [<read_struct_array_ $typ>](
                    reader: &mut Cursor<Vec<u8>>,
                    endian: &Endian,
                    count: usize,
                ) -> std::io::Result<Vec<$typ>> {
                    let mut vec = Vec::with_capacity(count);
                    for _ in 0..count {
                        vec.push([<read_ $typ>](reader, endian)?);
                    }
                    Ok(vec)
                }
            }
        )*
    };
}

define_read!(u16);
define_read!(u32);
define_read!(u64);
define_read_struct_array!(u16, u32, u64);

pub fn read_string(cursor: &mut Cursor<Vec<u8>>) -> std::io::Result<String> {
    let mut buf = Vec::new();

    loop {
        let mut byte = [0u8];
        if cursor.read_exact(&mut byte).is_err() {
            return Err(std::io::Error::new(ErrorKind::UnexpectedEof, "EOF before null terminator"));
        }

        if byte[0] == 0 {
            break;
        }

        buf.push(byte[0]);
    }

    // Convert to UTF-8 string
    String::from_utf8(buf)
        .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))
}