extern crate neovim_lib;
extern crate rmp;

use neovim_lib::session::Session;
use neovim_lib::neovim_api::Neovim;

#[test]
fn start_stop_test() {
    let mut session = if cfg!(target_os = "windows") {
        Session::new_child_path("E:\\Neovim\\bin\\nvim.exe").unwrap()
    } else {
        Session::new_child().unwrap()
    };

    let mut nvim = Neovim::new(session);
    println!("{:?}", nvim.vim_get_api_info().unwrap());
}
