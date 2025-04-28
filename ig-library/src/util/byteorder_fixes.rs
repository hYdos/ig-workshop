// Fixes a few issues with the "byteorder" crate that stem from poor design

use crate::core::ig_core_platform::IG_CORE_PLATFORM;
use crate::core::ig_fs::Endian;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use paste::paste;
use std::io::{Cursor, ErrorKind, Read};
use std::slice::from_raw_parts;

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

pub fn read_ptr(
    cursor: &mut Cursor<Vec<u8>>,
    platform: &IG_CORE_PLATFORM,
    endian: &Endian,
) -> std::io::Result<u64> {
    if platform.is_64bit() {
        read_u64(cursor, endian)
    } else {
        read_u32(cursor, endian).map(|t| t as u64)
    }
}

pub fn read_string(cursor: &mut Cursor<Vec<u8>>) -> std::io::Result<String> {
    let mut buf = Vec::new();

    loop {
        let mut byte = [0u8];
        if cursor.read_exact(&mut byte).is_err() {
            return Err(std::io::Error::new(
                ErrorKind::UnexpectedEof,
                "EOF before null terminator",
            ));
        }

        if byte[0] == 0 {
            break;
        }

        buf.push(byte[0]);
    }

    // Convert to UTF-8 string
    String::from_utf8(buf).map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))
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

// Custom implementation separate to macro doubles the speed
pub fn read_struct_array_u8<'a>(
    cursor: &mut Cursor<Vec<u8>>,
    _endian: &Endian,
    count: usize,
) -> std::io::Result<Vec<u8>> {
    let mut buf = vec![0u8; count];
    cursor.read_exact(&mut buf)?; // advance the position internally. Strange performance issue here
    Ok(buf)
}

/// Returns a slice from a [Cursor<Vec<u8>>] while also advancing the position.
/// When not modifying the data, this is a safe way of referencing data as needed,
/// and is faster than [read_struct_array_u8] due to the copy-less nature of a reference over an owned [Vec<u8>]
pub fn read_struct_array_u8_ref<'a>(
    cursor: &mut Cursor<Vec<u8>>,
    _endian: &Endian,
    count: usize,
) -> std::io::Result<&'a [u8]> {
    let start = cursor.position() as usize;
    let ptr = cursor.get_ref().as_ptr();
    cursor.set_position((start + count) as u64);

    // SAFETY: ptr was from a Vec<u8> with at least `start+count` bytes,
    // the Vec isn’t reallocated by set_position(),
    // and we honor the lifetime 'a tied to &mut Cursor.
    let slice = unsafe { from_raw_parts(ptr.add(start), count) };
    Ok(slice)
}

define_read!(u16);
define_read!(i16);
define_read!(u32);
define_read!(i32);
define_read!(u64);
define_read!(i64);
define_read_struct_array!(u16, u32, u64);
