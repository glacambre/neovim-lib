extern crate neovim_lib;
extern crate rmp;

use neovim_lib::session::Session;
use neovim_lib::neovim::Neovim;
use neovim_lib::neovim_api::NeovimApi;

#[test]
fn start_stop_test() {
    let session = if cfg!(target_os = "windows") {
        Session::new_child_path("E:\\Neovim\\bin\\nvim.exe").unwrap()
    } else {
        Session::new_child().unwrap()
    };

    let mut nvim = Neovim::new(session);
    println!("{:?}", nvim.vim_get_api_info().unwrap());
}

#[ignore]
#[test]
fn remote_test() {
    let session = Session::new_tcp("127.0.0.1:6666").unwrap();
    let mut nvim = Neovim::new(session);
    nvim.vim_command("echo \"Test\"").unwrap();
}
