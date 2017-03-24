use rpc::value::Value;

pub trait Handler {
    fn handle_notify(&mut self, _name: &str, _args: Vec<Value>) {}

    fn handle_request(&mut self, _name: &str, _args: Vec<Value>) -> Result<Value, Value> {
        Err(Value::String("Not implemented".to_owned()))
    }
}

pub struct DefaultHandler();

impl Handler for DefaultHandler {}
