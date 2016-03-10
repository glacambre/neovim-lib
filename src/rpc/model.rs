
use rmp::decode::value::read_value;
use rmp::encode::value::write_value;
use rmp::Value;
use rmp::value::Integer;
use std::io;
use std::io::{Read, Write};
use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum RpcMessage {
    RpcRequest {
        msgid: u64,
        method: String,
        params: Vec<Value>,
    }, // 0
    RpcResponse {
        msgid: u64,
        error: Value,
        result: Value,
    }, // 1
    RpcNotification {
        method: String,
        params: Vec<Value>,
    }, // 2
}

macro_rules! try_str {
    ($exp:expr, $msg:expr) => (match $exp {
        Value::String(ref val) => val.to_owned(),
        _ => return Err(Box::new(io::Error::new(io::ErrorKind::Other, $msg)))
    })
}

macro_rules! try_int {
    ($exp:expr, $msg:expr) => (match $exp {
        Value::Integer(Integer::U64(val)) => val,
        _ => return Err(Box::new(io::Error::new(io::ErrorKind::Other, $msg)))
    })
}

macro_rules! try_arr {
    ($exp:expr, $msg:expr) => (match $exp {
        Value::Array(ref arr) => arr.to_owned(),
        _ => return Err(Box::new(io::Error::new(io::ErrorKind::Other, $msg)))
    })
}

macro_rules! rpc_args {
    ($($e:expr), *) => {{
        let mut vec = Vec::new();
        $(
            vec.push($e.into_val());
        )*
        vec.into_val()
    }}
}

pub fn decode<R: Read>(reader: &mut R) -> Result<RpcMessage, Box<Error>> {
    let arr = try_arr!(try!(read_value(reader)), "Rpc message must be array");
    match try_int!(arr[0], "Can't find message type") {
        0 => {
            let msgid = try_int!(arr[1], "msgid not found");
            let method = try_str!(arr[2], "method not found");
            let params = try_arr!(arr[3], "params not found");
            Ok(RpcMessage::RpcRequest {
                msgid: msgid,
                method: method,
                params: params,
            })
        }
        1 => {
            let msgid = try_int!(arr[1], "msgid not found");
            Ok(RpcMessage::RpcResponse {
                msgid: msgid,
                error: arr[2].to_owned(),
                result: arr[3].to_owned(),
            })
        }
        2 => {
            let method = try_str!(arr[1], "method not found");
            let params = try_arr!(arr[2], "params not found");
            Ok(RpcMessage::RpcNotification {
                method: method,
                params: params,
            })

        }
        _ => Err(Box::new(io::Error::new(io::ErrorKind::Other, "Not nown type"))),
    }
}

pub fn encode<W: Write>(writer: &mut W, msg: &RpcMessage) -> Result<(), Box<Error>> {
    match msg {
        &RpcMessage::RpcRequest{msgid, ref method, ref params} => {
            let val = rpc_args!(0, msgid, method.to_owned(), params.to_owned());
            try!(write_value(writer, &val));
        }
        &RpcMessage::RpcResponse{msgid, ref error, ref result} => {
            let val = rpc_args!(1, msgid, error.to_owned(), result.to_owned());
            try!(write_value(writer, &val));
        }
        &RpcMessage::RpcNotification{ref method, ref params} => {
            let val = rpc_args!(2, method.to_owned(), params.to_owned());
            try!(write_value(writer, &val));
        }
    };
    Ok(())
}

pub trait IntoVal<T> {
    fn into_val(self) -> T;
}

impl IntoVal<Value> for bool {
    fn into_val(self) -> Value {
        Value::Boolean(self)
    }
}

impl IntoVal<Value> for u64 {
    fn into_val(self) -> Value {
        Value::Integer(Integer::U64(self))
    }
}

impl IntoVal<Value> for String {
    fn into_val(self) -> Value {
        Value::String(self)
    }
}

impl IntoVal<Value> for Vec<Value> {
    fn into_val(self) -> Value {
        Value::Array(self)
    }
}

impl IntoVal<Value> for Value {
    fn into_val(self) -> Value {
        self
    }
}

#[cfg(test)]
mod test {
    use std::io::{Cursor, SeekFrom, Seek};
    use super::*;

    #[test]
    fn request_test() {
        let msg = RpcMessage::RpcRequest {
            msgid: 1,
            method: "test_method".to_owned(),
            params: vec![],
        };

        let mut buff = Cursor::new(vec![]);
        encode(&mut buff, &msg).unwrap();

        buff.seek(SeekFrom::Start(0)).unwrap();
        let msg_dest = decode(&mut buff).unwrap();
        assert_eq!(msg, msg_dest);
    }
}
