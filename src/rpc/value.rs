use rmp;
use std::io::Read;
use rmp::Marker;
use rmp::decode::*;
use std::str::{Utf8Error, from_utf8};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Integer {
    /// Every non-negative integer is treated as u64, even if it fits in i64.
    U64(u64),
    /// Every negative integer is treated as i64.
    I64(i64),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Float {
    F32(f32),
    F64(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// Nil represents nil.
    Nil,
    /// Boolean represents true or false.
    Boolean(bool),
    /// Integer represents an integer.
    Integer(Integer),
    /// Float represents a floating point number.
    Float(Float),
    /// String extending Raw type represents a UTF-8 string.
    String(String),
    /// Binary extending Raw type represents a byte array.
    Binary(Vec<u8>),
    /// Array represents a sequence of objects.
    Array(Vec<Value>),
    /// Map represents key-value pairs of objects.
    Map(Vec<(Value, Value)>),
    /// Extended implements Extension interface: represents a tuple of type information and a byte
    /// array where type information is an integer whose meaning is defined by applications.
    Ext(i8, Vec<u8>),
}

/// Implements human-readable value formatting.
impl ::std::fmt::Display for Value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Value::Nil => write!(f, "nil"),
            Value::Boolean(val) => write!(f, "{}", val),
            Value::Integer(Integer::U64(val)) => write!(f, "{}", val),
            Value::Integer(Integer::I64(val)) => write!(f, "{}", val),
            Value::Float(Float::F32(val)) => write!(f, "{}", val),
            Value::Float(Float::F64(val)) => write!(f, "{}", val),
            Value::String(ref val) => write!(f, "\"{}\"", val),
            Value::Binary(ref val) => write!(f, "{:?}", val),
            Value::Array(ref vec) => {
                let res = vec.iter()
                             .map(|val| format!("{}", val))
                             .collect::<Vec<String>>()
                             .join(", ");

                write!(f, "[{}]", res)
            }
            Value::Map(ref vec) => {
                try!(write!(f, "{{"));

                match vec.iter().take(1).next() {
                    Some(&(ref k, ref v)) => {
                        try!(write!(f, "{}: {}", k, v));
                    }
                    None => {
                        try!(write!(f, ""));
                    }
                }

                for &(ref k, ref v) in vec.iter().skip(1) {
                    try!(write!(f, ", {}: {}", k, v));
                }

                write!(f, "}}")
            }
            Value::Ext(ty, ref data) => write!(f, "[{}, {:?}]", ty, data),
        }
    }
}

fn read_str_data<'r, R>(rd: &mut R,
                        len: usize,
                        buf: &'r mut [u8])
                        -> Result<&'r str, DecodeStringError<'r>>
    where R: Read
{
    debug_assert_eq!(len, buf.len());

    // Trying to copy exact `len` bytes.
    match rd.read_exact(buf) {
        Ok(()) => {
            match from_utf8(buf) {
                Ok(decoded) => Ok(decoded),
                Err(err) => Err(DecodeStringError::InvalidUtf8(buf, err)),
            }
        }
        Err(err) => Err(DecodeStringError::InvalidDataRead(From::from(err))),
    }
}

fn read_array<R>(rd: &mut R, len: usize) -> Result<Vec<Value>, ValueReadError>
    where R: Read
{
    let mut vec = Vec::with_capacity(len);

    for _ in 0..len {
        let val = try!(read_value(rd));
        vec.push(val);
    }
    Ok(vec)
}

fn read_map<R>(rd: &mut R, len: usize) -> Result<Vec<(Value, Value)>, ValueReadError>
    where R: Read
{
    let mut map = Vec::with_capacity(len);

    for _ in 0..len {
        let key = try!(read_value(rd));
        let value = try!(read_value(rd));

        map.push((key, value));
    }

    Ok(map)
}

fn read_bin_data<R>(rd: &mut R, len: usize) -> Result<Vec<u8>, ValueReadError>
    where R: Read
{
    let mut vec = Vec::with_capacity(len);
    try!(rd.read_exact(&mut vec).map_err(|e| ValueReadError::InvalidDataRead(e)));
    Ok(vec)
}

fn read_ext_body<R>(rd: &mut R, len: usize) -> Result<(i8, Vec<u8>), ValueReadError>
    where R: Read
{
    let ty = try!(read_data_i8(rd));
    let vec = try!(read_bin_data(rd, len));
    Ok((ty, vec))
}

/// Attempts to read bytes from the given reader and interpret them as a `Value`.
///
/// # Errors
///
/// This function will return `Error` on any I/O error while either reading or decoding a `Value`.
/// All instances of `ErrorKind::Interrupted` are handled by this function and the underlying
/// operation is retried.
pub fn read_value<R>(rd: &mut R) -> Result<Value, ValueReadError>
    where R: Read
{
    let val = match try!(read_marker(rd)) {
        Marker::Null => Value::Nil,
        Marker::True => Value::Boolean(true),
        Marker::False => Value::Boolean(false),
        Marker::FixPos(val) => Value::Integer(Integer::U64(val as u64)),
        Marker::FixNeg(val) => Value::Integer(Integer::I64(val as i64)),
        Marker::U8 => Value::Integer(Integer::U64(try!(read_data_u8(rd)) as u64)),
        Marker::U16 => Value::Integer(Integer::U64(try!(read_data_u16(rd)) as u64)),
        Marker::U32 => Value::Integer(Integer::U64(try!(read_data_u32(rd)) as u64)),
        Marker::U64 => Value::Integer(Integer::U64(try!(read_data_u64(rd)))),
        Marker::I8 => Value::Integer(Integer::I64(try!(read_data_i8(rd)) as i64)),
        Marker::I16 => Value::Integer(Integer::I64(try!(read_data_i16(rd)) as i64)),
        Marker::I32 => Value::Integer(Integer::I64(try!(read_data_i32(rd)) as i64)),
        Marker::I64 => Value::Integer(Integer::I64(try!(read_data_i64(rd)))),
        Marker::F32 => Value::Float(Float::F32(try!(read_data_f32(rd)))),
        Marker::F64 => Value::Float(Float::F64(try!(read_data_f64(rd)))),
        Marker::FixStr(len) => {
            let len = len as usize;
            let mut res: Vec<u8> = Vec::with_capacity(len);
            let res = try!(read_str_data(rd, len, &mut res)).to_owned();
            Value::String(res)
        }
        Marker::Str8 => {
            let len = try!(read_data_u8(rd)) as usize;
            let mut res: Vec<u8> = Vec::with_capacity(len);
            let res = try!(read_str_data(rd, len, &mut res)).to_owned();
            Value::String(res)
        }
        Marker::Str16 => {
            let len = try!(read_data_u16(rd)) as usize;
            let mut res: Vec<u8> = Vec::with_capacity(len);
            let res = try!(read_str_data(rd, len, &mut res)).to_owned();
            Value::String(res)
        }
        Marker::Str32 => {
            let len = try!(read_data_u32(rd)) as usize;
            let mut res: Vec<u8> = Vec::with_capacity(len);
            let res = try!(read_str_data(rd, len, &mut res)).to_owned();
            Value::String(res)
        }
        Marker::FixArray(len) => {
            let len = len as usize;
            let vec = try!(read_array(rd, len));
            Value::Array(vec)
        }
        Marker::Array16 => {
            let len = try!(read_data_u16(rd)) as usize;
            let vec = try!(read_array(rd, len));
            Value::Array(vec)
        }
        Marker::Array32 => {
            let len = try!(read_data_u32(rd)) as usize;
            let vec = try!(read_array(rd, len));
            Value::Array(vec)
        }
        Marker::FixMap(len) => {
            let len = len as usize;
            let map = try!(read_map(rd, len));
            Value::Map(map)
        }
        Marker::Map16 => {
            let len = try!(read_data_u16(rd)) as usize;
            let map = try!(read_map(rd, len));
            Value::Map(map)
        }
        Marker::Map32 => {
            let len = try!(read_data_u32(rd)) as usize;
            let map = try!(read_map(rd, len));
            Value::Map(map)
        }
        Marker::Bin8 => {
            let len = try!(read_data_u8(rd)) as usize;
            let vec = try!(read_bin_data(rd, len));
            Value::Binary(vec)
        }
        Marker::Bin16 => {
            let len = try!(read_data_u16(rd)) as usize;
            let vec = try!(read_bin_data(rd, len));
            Value::Binary(vec)
        }
        Marker::Bin32 => {
            let len = try!(read_data_u32(rd)) as usize;
            let vec = try!(read_bin_data(rd, len));
            Value::Binary(vec)
        }
        Marker::FixExt1 => {
            let len = 1 as usize;
            let (ty, vec) = try!(read_ext_body(rd, len));
            Value::Ext(ty, vec)
        }
        Marker::FixExt2 => {
            let len = 2 as usize;
            let (ty, vec) = try!(read_ext_body(rd, len));
            Value::Ext(ty, vec)
        }
        Marker::FixExt4 => {
            let len = 4 as usize;
            let (ty, vec) = try!(read_ext_body(rd, len));
            Value::Ext(ty, vec)
        }
        Marker::FixExt8 => {
            let len = 8 as usize;
            let (ty, vec) = try!(read_ext_body(rd, len));
            Value::Ext(ty, vec)
        }
        Marker::FixExt16 => {
            let len = 16 as usize;
            let (ty, vec) = try!(read_ext_body(rd, len));
            Value::Ext(ty, vec)
        }
        Marker::Ext8 => {
            let len = try!(read_data_u8(rd)) as usize;
            let (ty, vec) = try!(read_ext_body(rd, len));
            Value::Ext(ty, vec)
        }
        Marker::Ext16 => {
            let len = try!(read_data_u16(rd)) as usize;
            let (ty, vec) = try!(read_ext_body(rd, len));
            Value::Ext(ty, vec)
        }
        Marker::Ext32 => {
            let len = try!(read_data_u32(rd)) as usize;
            let (ty, vec) = try!(read_ext_body(rd, len));
            Value::Ext(ty, vec)
        }
        Marker::Reserved => return Err(ValueReadError::InvalidMarkerRead(Marker::Reserved)),
    };

    Ok(val)
}
