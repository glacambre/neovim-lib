//! # Rust library for Neovim clients
//!
//! Implements support for rust plugins for [Neovim](https://github.com/neovim/neovim) through its msgpack-rpc API.
//! # Examples
//! ```no_run
//! use neovim_lib::session::Session;
//! use neovim_lib::neovim::Neovim;
//! use neovim_lib::neovim_api::NeovimApi;
//! 
//! let session = Session::new_tcp("127.0.0.1:6666").unwrap();
//! let mut nvim = Neovim::new(session);
//! let buffers = nvim.get_buffers().unwrap();
//! buffers[0].set_line(&mut nvim, 0, "replace first line").unwrap();
//! nvim.command("vsplit").unwrap();
//! let windows = nvim.get_windows().unwrap();
//! windows[0].set_width(&mut nvim, 10).unwrap();
//! ```
extern crate rmp;
extern crate rmp_serialize;
extern crate rustc_serialize;

mod rpc;
#[macro_use]
pub mod session;
pub mod neovim;
pub mod neovim_api;

pub use neovim::Neovim;
pub use neovim_api::NeovimApi;
