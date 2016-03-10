use session::Session;
use rpc::*;
use rmp::Value;

pub struct Neovim {
    pub session: Session,
}

pub fn convert_array_of_string(vec: &Vec<String>) -> Value {
    Value::Array(vec.iter().map(|s| Value::String(s.to_owned())).collect())
}

pub fn map_generic_error(err: Value) -> String {
    match err {
        Value::String(val) => val.to_owned(),
        val => format!("Unknow error type: {:?}", val),
    }
}

impl Neovim {
    pub fn new(session: Session) -> Neovim {
        Neovim { session: session }
    }

    /// Register as a remote UI.
    ///
    /// After this method is called, the client will receive redraw notifications.
    pub fn ui_attach(&mut self, width: u64, height: u64, rgb: bool) -> Result<(), String> {
        self.session
            .call("ui_attach", &call_args!(width, height, rgb))
            .map_err(map_generic_error)
            .map(|_| ())
    }

    /// Unregister as a remote UI.
    pub fn ui_detach(&mut self) -> Result<(), String> {
        self.session.call("ui_detach", &vec![]).map_err(map_generic_error).map(|_| ())
    }

    /// Notify nvim that the client window has resized.
    ///
    /// If possible, nvim will send a redraw request to resize.
    pub fn ui_try_resize(&mut self, width: u64, height: u64) -> Result<(), String> {
        self.session
            .call("ui_try_resize", &call_args!(width, height))
            .map_err(map_generic_error)
            .map(|_| ())
    }
}
