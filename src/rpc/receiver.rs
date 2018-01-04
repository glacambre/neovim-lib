use std::time::{Duration, Instant};
use std::sync::mpsc;
use std::thread;

use rmpv::Value;

pub trait Receiver {
    fn receive(&self, receiver: mpsc::Receiver<Result<Value, Value>>) -> Result<Value, Value>;
}

pub struct WaitReceiver {
    timeout: Option<Duration>,
}

impl WaitReceiver {
    pub fn new() -> Self {
        WaitReceiver { timeout: Some(Duration::from_secs(10)) }
    }

    fn receive_inf(&self, receiver: mpsc::Receiver<Result<Value, Value>>) -> Result<Value, Value> {
        receiver.recv().unwrap()
    }
}

impl Receiver for WaitReceiver {
    fn receive(&self, receiver: mpsc::Receiver<Result<Value, Value>>) -> Result<Value, Value> {
        if let Some(dur) = self.timeout {
            let instant = Instant::now();
            let delay = Duration::from_millis(1);

            loop {
                match receiver.try_recv() {
                    Err(mpsc::TryRecvError::Empty) => {
                        thread::sleep(delay);
                        if instant.elapsed() >= dur {
                            return Err(Value::from("Wait timeout"));
                        }
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        return Err(Value::from("Channel disconnected"))
                    }
                    Ok(val) => return val,
                };
            }
        } else {
            self.receive_inf(receiver)
        }
    }
}
