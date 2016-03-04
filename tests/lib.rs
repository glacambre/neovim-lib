extern crate neovim_lib;
extern crate rmp;

use neovim_lib::session::Session;
use rmp::Value;
use std::thread;
use std::time::Duration;

#[test]
fn start_stop_test() {
    let mut session = if cfg!(target_os = "windows") {
        Session::new_child_path("E:\\Neovim\\bin\\nvim.exe").unwrap()
    } else {
        Session::new_child().unwrap()
    };

    session.call("vim_command_output", &vec![Value::String("echo \"Hello world\"".to_owned())]);

    thread::sleep(Duration::from_secs(2));
}
