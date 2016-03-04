use std::io::{Read, Write};
use std::marker::PhantomData;
use std::thread;
use std::thread::JoinHandle;

use rmp::Value;

use super::model;

pub struct Client <R: Read + Send + 'static, W: Write> {
    reader: PhantomData<R>,
    writer: W,
    dispatch_guard: JoinHandle<()>,
    msgid_counter: u64,
}

impl <R: Read + Send + 'static, W: Write> Client <R, W> {
    pub fn new(reader: R, writer: W) -> Client<R, W> {
        Client { 
            reader: PhantomData,
            writer: writer,
            msgid_counter: 0,
            dispatch_guard: Self::dispatch_thread(reader),
        }
    }

    pub fn call(&mut self, method: &str, args: &Vec<Value>) {
        let req = model::RpcMessage::RpcRequest {
            msgid: self.msgid_counter,
            method: method.to_owned(),
            params: args.clone(),
        };

        self.msgid_counter += 1;

        model::encode(&mut self.writer, &req).expect("Error send message");
    }

    fn dispatch_thread(mut reader: R) -> JoinHandle<()> {
        thread::spawn(move || {
            loop {
                let msg = model::decode(&mut reader).expect("Filed to decode message");
                println!("Get message {:?}", msg);
            }
        })
    }
}
