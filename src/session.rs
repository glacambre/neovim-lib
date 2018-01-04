use std::result;
use std::net::{TcpStream, ToSocketAddrs};
use std::io::Result;
use std::io::{Error, ErrorKind, Stdin, Stdout};
use std::process::Stdio;
use std::process::{Command, Child, ChildStdin, ChildStdout};
use std::thread::JoinHandle;

use std::path::Path;
#[cfg(unix)]
use unix_socket::UnixStream;

use rpc;
use rpc::Client;
use rpc::handler::Handler;

use rmpv::Value;


/// An active Neovim session.
pub struct Session<R: rpc::Receiver> {
    client: ClientConnection,
    receiver: R,
}

macro_rules! call_args {
    () => (Vec::new());
    ($($e:expr), +,) => (call_args![$($e),*]);
    ($($e:expr), +) => {{
        let mut vec = Vec::new();
        $(
            vec.push($e.into_val());
        )*
        vec
    }};
}

pub struct SessionBuilder {
    client: ClientConnection,
}

impl SessionBuilder {
    /// Connect to nvim instance via tcp
    pub fn new_tcp<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let stream = TcpStream::connect(addr)?;
        let read = stream.try_clone()?;
        Ok(SessionBuilder {
            client: ClientConnection::Tcp(Client::new(stream, read)),
        })
    }

    #[cfg(unix)]
    /// Connect to nvim instance via unix socket
    pub fn new_unix_socket<P: AsRef<Path>>(path: P) -> Result<Self> {
        let stream = UnixStream::connect(path)?;
        let read = stream.try_clone()?;
        Ok(SessionBuilder {
            client: ClientConnection::UnixSocket(Client::new(stream, read)),
        })
    }

    /// Connect to a Neovim instance by spawning a new one.
    pub fn new_child() -> Result<Self> {
        if cfg!(target_os = "linux") || cfg!(target_os = "freebsd") {
            Self::new_child_path("nvim")
        } else {
            Self::new_child_path("nvim.exe")
        }
    }

    /// Connect to a Neovim instance by spawning a new one
    pub fn new_child_path<S: AsRef<Path>>(program: S) -> Result<Self> {
        Self::new_child_cmd(Command::new(program.as_ref()).arg("--embed"))
    }

    /// Connect to a Neovim instance by spawning a new one
    ///
    /// stdin/stdout settings will be rewrited to `Stdio::piped()`
    pub fn new_child_cmd(cmd: &mut Command) -> Result<Self> {
        let mut child = cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;
        let stdout = child.stdout.take().ok_or_else(|| {
            Error::new(ErrorKind::Other, "Can't open stdout")
        })?;
        let stdin = child.stdin.take().ok_or_else(|| {
            Error::new(ErrorKind::Other, "Can't open stdin")
        })?;

        Ok(SessionBuilder {
            client: ClientConnection::Child(Client::new(stdout, stdin), child),
        })
    }

    /// Connect to a Neovim instance that spawned this process over stdin/stdout.
    pub fn new_parent() -> Result<Self> {
        use std::io;

        Ok(SessionBuilder {
            client: ClientConnection::Parent(Client::new(io::stdin(), io::stdout())),
        })
    }

    pub fn get(self) -> Session<rpc::WaitReceiver> {
        Session {
            client: self.client,
            receiver: rpc::WaitReceiver::new(),
        }
    }

    pub fn get_with_receiver<R: rpc::Receiver>(self, receiver: R) -> Session<R> {
        Session {
            client: self.client,
            receiver: receiver,
        }
    }
}

impl<R: rpc::Receiver> Session<R> {
    pub fn get_receiver_mut(&mut self) -> &mut R {
        &mut self.receiver
    }

    /// Start processing rpc response and notifications
    pub fn start_event_loop_handler<H>(&mut self, handler: H)
    where
        H: Handler + Send + 'static,
    {
        match self.client {
            ClientConnection::Child(ref mut client, _) => client.start_event_loop_handler(handler),
            ClientConnection::Parent(ref mut client) => client.start_event_loop_handler(handler),
            ClientConnection::Tcp(ref mut client) => client.start_event_loop_handler(handler),

            #[cfg(unix)]
            ClientConnection::UnixSocket(ref mut client) => {
                client.start_event_loop_handler(handler)
            }
        }
    }

    /// Start processing rpc response and notifications
    pub fn start_event_loop(&mut self) {
        match self.client {
            ClientConnection::Child(ref mut client, _) => client.start_event_loop(),
            ClientConnection::Parent(ref mut client) => client.start_event_loop(),
            ClientConnection::Tcp(ref mut client) => client.start_event_loop(),

            #[cfg(unix)]
            ClientConnection::UnixSocket(ref mut client) => client.start_event_loop(),
        }
    }

    /// Sync call. Call can be made only after event loop begin processing
    pub fn call(&mut self, method: &str, args: Vec<Value>) -> result::Result<Value, Value> {
        match self.client {
            ClientConnection::Child(ref mut client, _) => client.call(method, args, &self.receiver),
            ClientConnection::Parent(ref mut client) => client.call(method, args, &self.receiver),
            ClientConnection::Tcp(ref mut client) => client.call(method, args, &self.receiver),

            #[cfg(unix)]
            ClientConnection::UnixSocket(ref mut client) => {
                client.call(method, args, &self.receiver)
            }
        }
    }

    /// Wait dispatch thread to finish.
    ///
    /// This can happens in case child process connection is lost for some reason.
    pub fn take_dispatch_guard(&mut self) -> JoinHandle<()> {
        match self.client {
            ClientConnection::Child(ref mut client, _) => client.take_dispatch_guard(),
            ClientConnection::Parent(ref mut client) => client.take_dispatch_guard(),
            ClientConnection::Tcp(ref mut client) => client.take_dispatch_guard(),

            #[cfg(unix)]
            ClientConnection::UnixSocket(ref mut client) => client.take_dispatch_guard(),
        }
    }
}

enum ClientConnection {
    Child(Client<ChildStdout, ChildStdin>, Child),
    Parent(Client<Stdin, Stdout>),
    Tcp(Client<TcpStream, TcpStream>),

    #[cfg(unix)]
    UnixSocket(Client<UnixStream, UnixStream>),
}
