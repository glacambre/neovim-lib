use std::io::{Read, Write};
use std::thread;
use std::thread::JoinHandle;
use std::collections::HashMap;
use std::sync::{mpsc, Mutex, Arc};

use rmp::Value;

use super::model;

type Queue = Arc<Mutex<HashMap<u64, mpsc::Sender<Result<Value, Value>>>>>;

pub struct Client<R: Read + Send + 'static, W: Write> {
    reader: Option<R>,
    writer: W,
    dispatch_guard: Option<JoinHandle<()>>,
    queue: Queue,
    msgid_counter: u64,
}

impl<R: Read + Send + 'static, W: Write> Client<R, W> {

    pub fn start_event_loop_cb<F: Fn(&str, Vec<Value>) + Send + 'static>(&mut self, cb: F) {
        self.dispatch_guard = Some(Self::dispatch_thread(self.queue.clone(), self.reader.take().unwrap(), cb))
    }

    pub fn start_event_loop(&mut self) {
        self.dispatch_guard = Some(Self::dispatch_thread(self.queue.clone(), self.reader.take().unwrap(), |_, _| ()))
    }

    pub fn new(reader: R, writer: W) -> Self {
        let queue = Arc::new(Mutex::new(HashMap::new()));
        Client {
            reader: Some(reader),
            writer: writer,
            msgid_counter: 0,
            queue: queue.clone(),
            dispatch_guard: None,
        }
    }

    pub fn call(&mut self, method: &str, args: &Vec<Value>) -> Result<Value, Value> {
        if self.dispatch_guard.is_none() {
            return Err(Value::String("Event loop not started".to_owned()));
        }

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

    fn dispatch_thread<F: Fn(&str, Vec<Value>) + Send + 'static>(queue: Queue, mut reader: R, cb: F) -> JoinHandle<()> {
        thread::spawn(move || {
            loop {
                let msg = model::decode(&mut reader).expect("Filed to decode message");
                debug!("Get message {:?}", msg);
                match msg {
                    model::RpcMessage::RpcResponse{msgid, result, error} => {
                        let sender = queue.lock().unwrap().remove(&msgid).unwrap();
                        if error != Value::Nil {
                            sender.send(Err(error)).unwrap();
                        }
                        sender.send(Ok(result)).unwrap();
                    },
                    model::RpcMessage::RpcNotification{method, params} => {
                        cb(&method, params);
                    },
                    _ => println!("Unknown type"),
                };
            }
        })
    }
}
