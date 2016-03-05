use std::result;
use std::io::Result;
use std::io::{Error, ErrorKind};
use std::process::Stdio;
use std::process::{Command, Child, ChildStdin, ChildStdout};

use rmp::Value;

use rpc::Client;

/// An active Neovim session.
pub struct Session{
    client: ClientConnection
}

impl Session {
    /// Connect to a Neovim instance by spawning a new one.
    pub fn new_child() -> Result<Session> {
        if cfg!(target_os = "linux") {
            Self::new_child_path("nvim")
        } else {
            Self::new_child_path("nvim.exe")
        }
    }

    /// Connect to a Neovim instance by spawning a new one.
    pub fn new_child_path(program: &str) -> Result<Session> {
        let mut child = try!(Command::new(program)
                                 .arg("--embed")
                                 .stdin(Stdio::piped())
                                 .stdout(Stdio::piped())
                                 .spawn());
        let stdout = try!(child.stdout
                               .take()
                               .ok_or_else(|| Error::new(ErrorKind::Other, "Can't open stdout")));
        let stdin = try!(child.stdin
                              .take()
                              .ok_or_else(|| Error::new(ErrorKind::Other, "Can't open stdin")));

        Ok(Session { client: ClientConnection::Child(Client::new(stdout, stdin), child) })
    }

    /// Sync call
    pub fn call(&mut self, method: &str, args: &Vec<Value>) -> result::Result<Value, Value> {
        match self.client {
            ClientConnection::Child(ref mut client, _) => {
                client.call(method, args)
            }
        }
    }
}

enum ClientConnection{
    Child(Client<ChildStdout, ChildStdin>, Child),
}
