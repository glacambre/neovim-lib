// Auto generated 2016-03-05 01:58:47.347989

use std::collections::HashMap;

pub struct Neovim {
    session: Session,
    ext_types: HashMap<u64, Fn>,
}

impl Neovim {
    pub fn new(session: Session) -> Neovim {
        let ext_types = HashMap::new();
        ext_types.insert(0, unpackBuffer);
        ext_types.insert(1, unpackWindow);
        ext_types.insert(2, unpackTabpage);
        Neovim {
            session: session,
            ext_types: ext_types,
        }
    }

    pub fn tabpage_get_windows(&mut self, tabpage: u64) {
        self.session.call("tabpage_get_windows", tabpage);
    }

    pub fn tabpage_get_var(&mut self, tabpage: u64, name: String) {
        self.session.call("tabpage_get_var", tabpage, name);
    }

    pub fn tabpage_set_var(&mut self, tabpage: u64, name: String, value: Value) {
        self.session.call("tabpage_set_var", tabpage, name, value);
    }

    pub fn tabpage_get_window(&mut self, tabpage: u64) {
        self.session.call("tabpage_get_window", tabpage);
    }

    pub fn tabpage_is_valid(&mut self, tabpage: u64) {
        self.session.call("tabpage_is_valid", tabpage);
    }

    pub fn window_get_buffer(&mut self, window: u64) {
        self.session.call("window_get_buffer", window);
    }

    pub fn window_get_cursor(&mut self, window: u64) {
        self.session.call("window_get_cursor", window);
    }

    pub fn window_set_cursor(&mut self, window: u64, pos: (u64, u64)) {
        self.session.call("window_set_cursor", window, pos);
    }

    pub fn window_get_height(&mut self, window: u64) {
        self.session.call("window_get_height", window);
    }

    pub fn window_set_height(&mut self, window: u64, height: u64) {
        self.session.call("window_set_height", window, height);
    }

    pub fn window_get_width(&mut self, window: u64) {
        self.session.call("window_get_width", window);
    }

    pub fn window_set_width(&mut self, window: u64, width: u64) {
        self.session.call("window_set_width", window, width);
    }

    pub fn window_get_var(&mut self, window: u64, name: String) {
        self.session.call("window_get_var", window, name);
    }

    pub fn window_set_var(&mut self, window: u64, name: String, value: Value) {
        self.session.call("window_set_var", window, name, value);
    }

    pub fn window_get_option(&mut self, window: u64, name: String) {
        self.session.call("window_get_option", window, name);
    }

    pub fn window_set_option(&mut self, window: u64, name: String, value: Value) {
        self.session.call("window_set_option", window, name, value);
    }

    pub fn window_get_position(&mut self, window: u64) {
        self.session.call("window_get_position", window);
    }

    pub fn window_get_tabpage(&mut self, window: u64) {
        self.session.call("window_get_tabpage", window);
    }

    pub fn window_is_valid(&mut self, window: u64) {
        self.session.call("window_is_valid", window);
    }

    pub fn buffer_line_count(&mut self, buffer: u64) {
        self.session.call("buffer_line_count", buffer);
    }

    pub fn buffer_get_line(&mut self, buffer: u64, index: u64) {
        self.session.call("buffer_get_line", buffer, index);
    }

    pub fn buffer_set_line(&mut self, buffer: u64, index: u64, line: String) {
        self.session.call("buffer_set_line", buffer, index, line);
    }

    pub fn buffer_del_line(&mut self, buffer: u64, index: u64) {
        self.session.call("buffer_del_line", buffer, index);
    }

    pub fn buffer_get_line_slice(&mut self,
                                 buffer: u64,
                                 start: u64,
                                 end: u64,
                                 include_start: bool,
                                 include_end: bool) {
        self.session.call("buffer_get_line_slice",
                          buffer,
                          start,
                          end,
                          include_start,
                          include_end);
    }

    pub fn buffer_set_line_slice(&mut self,
                                 buffer: u64,
                                 start: u64,
                                 end: u64,
                                 include_start: bool,
                                 include_end: bool,
                                 replacement: Vec<String>) {
        self.session.call("buffer_set_line_slice",
                          buffer,
                          start,
                          end,
                          include_start,
                          include_end,
                          replacement);
    }

    pub fn buffer_get_var(&mut self, buffer: u64, name: String) {
        self.session.call("buffer_get_var", buffer, name);
    }

    pub fn buffer_set_var(&mut self, buffer: u64, name: String, value: Value) {
        self.session.call("buffer_set_var", buffer, name, value);
    }

    pub fn buffer_get_option(&mut self, buffer: u64, name: String) {
        self.session.call("buffer_get_option", buffer, name);
    }

    pub fn buffer_set_option(&mut self, buffer: u64, name: String, value: Value) {
        self.session.call("buffer_set_option", buffer, name, value);
    }

    pub fn buffer_get_number(&mut self, buffer: u64) {
        self.session.call("buffer_get_number", buffer);
    }

    pub fn buffer_get_name(&mut self, buffer: u64) {
        self.session.call("buffer_get_name", buffer);
    }

    pub fn buffer_set_name(&mut self, buffer: u64, name: String) {
        self.session.call("buffer_set_name", buffer, name);
    }

    pub fn buffer_is_valid(&mut self, buffer: u64) {
        self.session.call("buffer_is_valid", buffer);
    }

    pub fn buffer_insert(&mut self, buffer: u64, lnum: u64, lines: Vec<String>) {
        self.session.call("buffer_insert", buffer, lnum, lines);
    }

    pub fn buffer_get_mark(&mut self, buffer: u64, name: String) {
        self.session.call("buffer_get_mark", buffer, name);
    }

    pub fn vim_command(&mut self, str: String) {
        self.session.call("vim_command", str);
    }

    pub fn vim_feedkeys(&mut self, keys: String, mode: String, escape_csi: bool) {
        self.session.call("vim_feedkeys", keys, mode, escape_csi);
    }

    pub fn vim_input(&mut self, keys: String) {
        self.session.call("vim_input", keys);
    }

    pub fn vim_replace_termcodes(&mut self,
                                 str: String,
                                 from_part: bool,
                                 do_lt: bool,
                                 special: bool) {
        self.session.call("vim_replace_termcodes", str, from_part, do_lt, special);
    }

    pub fn vim_command_output(&mut self, str: String) {
        self.session.call("vim_command_output", str);
    }

    pub fn vim_eval(&mut self, str: String) {
        self.session.call("vim_eval", str);
    }

    pub fn vim_call_function(&mut self, fname: String, args: Vec<Value>) {
        self.session.call("vim_call_function", fname, args);
    }

    pub fn vim_strwidth(&mut self, str: String) {
        self.session.call("vim_strwidth", str);
    }

    pub fn vim_list_runtime_paths(&mut self) {
        self.session.call("vim_list_runtime_paths");
    }

    pub fn vim_change_directory(&mut self, dir: String) {
        self.session.call("vim_change_directory", dir);
    }

    pub fn vim_get_current_line(&mut self) {
        self.session.call("vim_get_current_line");
    }

    pub fn vim_set_current_line(&mut self, line: String) {
        self.session.call("vim_set_current_line", line);
    }

    pub fn vim_del_current_line(&mut self) {
        self.session.call("vim_del_current_line");
    }

    pub fn vim_get_var(&mut self, name: String) {
        self.session.call("vim_get_var", name);
    }

    pub fn vim_set_var(&mut self, name: String, value: Value) {
        self.session.call("vim_set_var", name, value);
    }

    pub fn vim_get_vvar(&mut self, name: String) {
        self.session.call("vim_get_vvar", name);
    }

    pub fn vim_get_option(&mut self, name: String) {
        self.session.call("vim_get_option", name);
    }

    pub fn vim_set_option(&mut self, name: String, value: Value) {
        self.session.call("vim_set_option", name, value);
    }

    pub fn vim_out_write(&mut self, str: String) {
        self.session.call("vim_out_write", str);
    }

    pub fn vim_err_write(&mut self, str: String) {
        self.session.call("vim_err_write", str);
    }

    pub fn vim_report_error(&mut self, str: String) {
        self.session.call("vim_report_error", str);
    }

    pub fn vim_get_buffers(&mut self) {
        self.session.call("vim_get_buffers");
    }

    pub fn vim_get_current_buffer(&mut self) {
        self.session.call("vim_get_current_buffer");
    }

    pub fn vim_set_current_buffer(&mut self, buffer: u64) {
        self.session.call("vim_set_current_buffer", buffer);
    }

    pub fn vim_get_windows(&mut self) {
        self.session.call("vim_get_windows");
    }

    pub fn vim_get_current_window(&mut self) {
        self.session.call("vim_get_current_window");
    }

    pub fn vim_set_current_window(&mut self, window: u64) {
        self.session.call("vim_set_current_window", window);
    }

    pub fn vim_get_tabpages(&mut self) {
        self.session.call("vim_get_tabpages");
    }

    pub fn vim_get_current_tabpage(&mut self) {
        self.session.call("vim_get_current_tabpage");
    }

    pub fn vim_set_current_tabpage(&mut self, tabpage: u64) {
        self.session.call("vim_set_current_tabpage", tabpage);
    }

    pub fn vim_subscribe(&mut self, event: String) {
        self.session.call("vim_subscribe", event);
    }

    pub fn vim_unsubscribe(&mut self, event: String) {
        self.session.call("vim_unsubscribe", event);
    }

    pub fn vim_name_to_color(&mut self, name: String) {
        self.session.call("vim_name_to_color", name);
    }
}
