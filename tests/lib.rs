extern crate neovim_lib;

use neovim_lib::session::Session;

#[test]
fn start_stop_test() {
    Session::new_child_path("E:\\Neovim\\bin\\nvim.exe").unwrap();
}
