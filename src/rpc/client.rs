use std::io::{Read, Write};
use std::marker::PhantomData;
use std::thread;
use std::thread::JoinHandle;
use super::model;

pub struct Client <R: Read + Send + 'static, W: Write> {
    reader: PhantomData<R>,
    writer: W,
    dispatch_guard: JoinHandle<()>,
}

impl <R: Read + Send + 'static, W: Write> Client <R, W> {
    pub fn new(reader: R, writer: W) -> Client<R, W> {
        Client { 
            reader: PhantomData,
            writer: writer,
            dispatch_guard: Self::dispatch_thread(reader),
        }
    }

    fn dispatch_thread(mut reader: R) -> JoinHandle<()> {
        thread::spawn(move || {
            loop {
                model::decode(&mut reader).unwrap();
            }
        })
    }
}
