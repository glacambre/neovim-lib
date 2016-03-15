use std::io::{Read, Write};
use std::marker::PhantomData;
use std::thread;
use std::thread::JoinHandle;
use std::collections::HashMap;
use std::sync::{mpsc, Mutex, Arc};

use rmp::Value;

use super::model;

type Queue = Arc<Mutex<HashMap<u64, mpsc::Sender<Result<Value, Value>>>>>;

pub struct Client<R: Read + Send + 'static, W: Write> {
    reader: PhantomData<R>,
    writer: W,
    dispatch_guard: JoinHandle<()>,
    queue: Queue,
    msgid_counter: u64,
}

impl<R: Read + Send + 'static, W: Write> Client<R, W> {
    pub fn new(reader: R, writer: W) -> Client<R, W> {
        let queue = Arc::new(Mutex::new(HashMap::new()));
        Client {
            reader: PhantomData,
            writer: writer,
            msgid_counter: 0,
            queue: queue.clone(),
            dispatch_guard: Self::dispatch_thread(queue.clone(), reader),
        }
    }

    pub fn call(&mut self, method: &str, args: &Vec<Value>) -> Result<Value, Value> {
        let msgid = self.msgid_counter;
        self.msgid_counter += 1;

        let req = model::RpcMessage::RpcRequest {
            msgid: msgid,
            method: method.to_owned(),
            params: args.clone(),
        };

        let (sender, receiver) = mpsc::channel();
        self.queue.lock().unwrap().insert(msgid, sender);

        model::encode(&mut self.writer, &req).expect("Error send message");
        receiver.recv().unwrap()
    }

    fn dispatch_thread(queue: Queue, mut reader: R) -> JoinHandle<()> {
        thread::spawn(move || {
            loop {
                let msg = model::decode(&mut reader).expect("Filed to decode message");
                println!("Get message {:?}", msg);
                match msg {
                    model::RpcMessage::RpcResponse{msgid, result, error} => {
                        let sender = queue.lock().unwrap().remove(&msgid).unwrap();
                        if error != Value::Nil {
                            sender.send(Err(error)).unwrap();
                        }
                        sender.send(Ok(result)).unwrap();
                    }
                    _ => println!("Unknown type"),
                };
            }
        })
    }
}
