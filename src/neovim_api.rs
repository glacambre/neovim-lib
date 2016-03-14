// Auto generated 2016-03-14 13:15:19.665000

use neovim::*;
use rmp::Value;
use rpc::*;
use session::Session;

pub struct Buffer<'a> {
    code_data: Value,
    code: u64,
    session: &'a Session,
}

impl<'a> Buffer<'a> {
    pub fn new(session: &'a Session, code_data: Value) -> Buffer {
        Buffer {
            code_data: code_data,
            code: 0,
            session: session,
        }
    }

    pub fn buffer_line_count(&mut self) -> Result<u64, String> {
        self.session
            .call("buffer_line_count", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn buffer_get_line(&mut self, index: u64) -> Result<String, String> {
        self.session
            .call("buffer_get_line",
                  &call_args![self.code_data.clone(), index])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn buffer_set_line(&mut self, index: u64, line: &str) -> Result<(), String> {
        self.session
            .call("buffer_set_line",
                  &call_args![self.code_data.clone(), index, line])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn buffer_del_line(&mut self, index: u64) -> Result<(), String> {
        self.session
            .call("buffer_del_line",
                  &call_args![self.code_data.clone(), index])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn buffer_get_line_slice(&mut self,
                                 start: u64,
                                 end: u64,
                                 include_start: bool,
                                 include_end: bool)
                                 -> Result<Vec<String>, String> {
        self.session
            .call("buffer_get_line_slice",
                  &call_args![self.code_data.clone(), start, end, include_start, include_end])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn buffer_set_line_slice(&mut self,
                                 start: u64,
                                 end: u64,
                                 include_start: bool,
                                 include_end: bool,
                                 replacement: Vec<String>)
                                 -> Result<(), String> {
        self.session
            .call("buffer_set_line_slice",
                  &call_args![self.code_data.clone(),
                              start,
                              end,
                              include_start,
                              include_end,
                              replacement])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn buffer_get_var(&mut self, name: &str) -> Result<Value, String> {
        self.session
            .call("buffer_get_var", &call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn buffer_set_var(&mut self, name: &str, value: Value) -> Result<Value, String> {
        self.session
            .call("buffer_set_var",
                  &call_args![self.code_data.clone(), name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn buffer_get_option(&mut self, name: &str) -> Result<Value, String> {
        self.session
            .call("buffer_get_option",
                  &call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn buffer_set_option(&mut self, name: &str, value: Value) -> Result<(), String> {
        self.session
            .call("buffer_set_option",
                  &call_args![self.code_data.clone(), name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn buffer_get_number(&mut self) -> Result<u64, String> {
        self.session
            .call("buffer_get_number", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn buffer_get_name(&mut self) -> Result<String, String> {
        self.session
            .call("buffer_get_name", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn buffer_set_name(&mut self, name: &str) -> Result<(), String> {
        self.session
            .call("buffer_set_name", &call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn buffer_is_valid(&mut self) -> Result<bool, String> {
        self.session
            .call("buffer_is_valid", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn buffer_insert(&mut self, lnum: u64, lines: Vec<String>) -> Result<(), String> {
        self.session
            .call("buffer_insert",
                  &call_args![self.code_data.clone(), lnum, lines])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn buffer_get_mark(&mut self, name: &str) -> Result<(u64, u64), String> {
        self.session
            .call("buffer_get_mark", &call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn buffer_add_highlight(&mut self,
                                src_id: u64,
                                hl_group: &str,
                                line: u64,
                                col_start: u64,
                                col_end: u64)
                                -> Result<u64, String> {
        self.session
            .call("buffer_add_highlight",
                  &call_args![self.code_data.clone(), src_id, hl_group, line, col_start, col_end])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn buffer_clear_highlight(&mut self,
                                  src_id: u64,
                                  line_start: u64,
                                  line_end: u64)
                                  -> Result<(), String> {
        self.session
            .call("buffer_clear_highlight",
                  &call_args![self.code_data.clone(), src_id, line_start, line_end])
            .map(map_result)
            .map_err(map_generic_error)
    }
}

pub struct Window<'a> {
    code_data: Value,
    code: u64,
    session: &'a Session,
}

impl<'a> Window<'a> {
    pub fn new(session: &'a Session, code_data: Value) -> Window {
        Window {
            code_data: code_data,
            code: 1,
            session: session,
        }
    }

    pub fn window_get_buffer(&mut self) -> Result<Buffer, String> {
        self.session
            .call("window_get_buffer", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn window_get_cursor(&mut self) -> Result<(u64, u64), String> {
        self.session
            .call("window_get_cursor", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn window_set_cursor(&mut self, pos: (u64, u64)) -> Result<(), String> {
        self.session
            .call("window_set_cursor",
                  &call_args![self.code_data.clone(), pos])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn window_get_height(&mut self) -> Result<u64, String> {
        self.session
            .call("window_get_height", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn window_set_height(&mut self, height: u64) -> Result<(), String> {
        self.session
            .call("window_set_height",
                  &call_args![self.code_data.clone(), height])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn window_get_width(&mut self) -> Result<u64, String> {
        self.session
            .call("window_get_width", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn window_set_width(&mut self, width: u64) -> Result<(), String> {
        self.session
            .call("window_set_width",
                  &call_args![self.code_data.clone(), width])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn window_get_var(&mut self, name: &str) -> Result<Value, String> {
        self.session
            .call("window_get_var", &call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn window_set_var(&mut self, name: &str, value: Value) -> Result<Value, String> {
        self.session
            .call("window_set_var",
                  &call_args![self.code_data.clone(), name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn window_get_option(&mut self, name: &str) -> Result<Value, String> {
        self.session
            .call("window_get_option",
                  &call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn window_set_option(&mut self, name: &str, value: Value) -> Result<(), String> {
        self.session
            .call("window_set_option",
                  &call_args![self.code_data.clone(), name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn window_get_position(&mut self) -> Result<(u64, u64), String> {
        self.session
            .call("window_get_position", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn window_get_tabpage(&mut self) -> Result<Tabpage, String> {
        self.session
            .call("window_get_tabpage", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn window_is_valid(&mut self) -> Result<bool, String> {
        self.session
            .call("window_is_valid", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
}

pub struct Tabpage<'a> {
    code_data: Value,
    code: u64,
    session: &'a Session,
}

impl<'a> Tabpage<'a> {
    pub fn new(session: &'a Session, code_data: Value) -> Tabpage {
        Tabpage {
            code_data: code_data,
            code: 2,
            session: session,
        }
    }

    pub fn tabpage_get_windows(&mut self) -> Result<Vec<Window>, String> {
        self.session
            .call("tabpage_get_windows", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn tabpage_get_var(&mut self, name: &str) -> Result<Value, String> {
        self.session
            .call("tabpage_get_var", &call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn tabpage_set_var(&mut self, name: &str, value: Value) -> Result<Value, String> {
        self.session
            .call("tabpage_set_var",
                  &call_args![self.code_data.clone(), name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn tabpage_get_window(&mut self) -> Result<Window, String> {
        self.session
            .call("tabpage_get_window", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn tabpage_is_valid(&mut self) -> Result<bool, String> {
        self.session
            .call("tabpage_is_valid", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
}


impl FromVal<Value> for Window {
    fn from_val(val: Value) -> Self {
        Window { code_data: val }
    }
}

impl FromVal<Value> for Tabpage {
    fn from_val(val: Value) -> Self {
        Tabpage { code_data: val }
    }
}

impl FromVal<Value> for Buffer {
    fn from_val(val: Value) -> Self {
        Buffer { code_data: val }
    }
}

pub trait NeovimApi {
    fn vim_command(&mut self, str: &str) -> Result<(), String>;
    fn vim_feedkeys(&mut self, keys: &str, mode: &str, escape_csi: bool) -> Result<(), String>;
    fn vim_input(&mut self, keys: &str) -> Result<u64, String>;
    fn vim_replace_termcodes(&mut self,
                             str: &str,
                             from_part: bool,
                             do_lt: bool,
                             special: bool)
                             -> Result<String, String>;
    fn vim_command_output(&mut self, str: &str) -> Result<String, String>;
    fn vim_eval(&mut self, str: &str) -> Result<Value, String>;
    fn vim_call_function(&mut self, fname: &str, args: Vec<Value>) -> Result<Value, String>;
    fn vim_strwidth(&mut self, str: &str) -> Result<u64, String>;
    fn vim_list_runtime_paths(&mut self) -> Result<Vec<String>, String>;
    fn vim_change_directory(&mut self, dir: &str) -> Result<(), String>;
    fn vim_get_current_line(&mut self) -> Result<String, String>;
    fn vim_set_current_line(&mut self, line: &str) -> Result<(), String>;
    fn vim_del_current_line(&mut self) -> Result<(), String>;
    fn vim_get_var(&mut self, name: &str) -> Result<Value, String>;
    fn vim_set_var(&mut self, name: &str, value: Value) -> Result<Value, String>;
    fn vim_get_vvar(&mut self, name: &str) -> Result<Value, String>;
    fn vim_get_option(&mut self, name: &str) -> Result<Value, String>;
    fn vim_set_option(&mut self, name: &str, value: Value) -> Result<(), String>;
    fn vim_out_write(&mut self, str: &str) -> Result<(), String>;
    fn vim_err_write(&mut self, str: &str) -> Result<(), String>;
    fn vim_report_error(&mut self, str: &str) -> Result<(), String>;
    fn vim_get_buffers(&mut self) -> Result<Vec<Buffer>, String>;
    fn vim_get_current_buffer(&mut self) -> Result<Buffer, String>;
    fn vim_set_current_buffer(&mut self, buffer: Buffer) -> Result<(), String>;
    fn vim_get_windows(&mut self) -> Result<Vec<Window>, String>;
    fn vim_get_current_window(&mut self) -> Result<Window, String>;
    fn vim_set_current_window(&mut self, window: Window) -> Result<(), String>;
    fn vim_get_tabpages(&mut self) -> Result<Vec<Tabpage>, String>;
    fn vim_get_current_tabpage(&mut self) -> Result<Tabpage, String>;
    fn vim_set_current_tabpage(&mut self, tabpage: Tabpage) -> Result<(), String>;
    fn vim_subscribe(&mut self, event: &str) -> Result<(), String>;
    fn vim_unsubscribe(&mut self, event: &str) -> Result<(), String>;
    fn vim_name_to_color(&mut self, name: &str) -> Result<u64, String>;
    fn vim_get_api_info(&mut self) -> Result<Vec<Value>, String>;
                                                                                                                        }

impl NeovimApi for Neovim {
    fn vim_command(&mut self, str: &str) -> Result<(), String> {
        self.session
            .call("vim_command", &call_args![str])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_feedkeys(&mut self, keys: &str, mode: &str, escape_csi: bool) -> Result<(), String> {
        self.session
            .call("vim_feedkeys", &call_args![keys, mode, escape_csi])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_input(&mut self, keys: &str) -> Result<u64, String> {
        self.session
            .call("vim_input", &call_args![keys])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_replace_termcodes(&mut self,
                             str: &str,
                             from_part: bool,
                             do_lt: bool,
                             special: bool)
                             -> Result<String, String> {
        self.session
            .call("vim_replace_termcodes",
                  &call_args![str, from_part, do_lt, special])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_command_output(&mut self, str: &str) -> Result<String, String> {
        self.session
            .call("vim_command_output", &call_args![str])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_eval(&mut self, str: &str) -> Result<Value, String> {
        self.session
            .call("vim_eval", &call_args![str])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_call_function(&mut self, fname: &str, args: Vec<Value>) -> Result<Value, String> {
        self.session
            .call("vim_call_function", &call_args![fname, args])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_strwidth(&mut self, str: &str) -> Result<u64, String> {
        self.session
            .call("vim_strwidth", &call_args![str])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_list_runtime_paths(&mut self) -> Result<Vec<String>, String> {
        self.session
            .call("vim_list_runtime_paths", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_change_directory(&mut self, dir: &str) -> Result<(), String> {
        self.session
            .call("vim_change_directory", &call_args![dir])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_get_current_line(&mut self) -> Result<String, String> {
        self.session
            .call("vim_get_current_line", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_set_current_line(&mut self, line: &str) -> Result<(), String> {
        self.session
            .call("vim_set_current_line", &call_args![line])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_del_current_line(&mut self) -> Result<(), String> {
        self.session
            .call("vim_del_current_line", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_get_var(&mut self, name: &str) -> Result<Value, String> {
        self.session
            .call("vim_get_var", &call_args![name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_set_var(&mut self, name: &str, value: Value) -> Result<Value, String> {
        self.session
            .call("vim_set_var", &call_args![name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_get_vvar(&mut self, name: &str) -> Result<Value, String> {
        self.session
            .call("vim_get_vvar", &call_args![name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_get_option(&mut self, name: &str) -> Result<Value, String> {
        self.session
            .call("vim_get_option", &call_args![name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_set_option(&mut self, name: &str, value: Value) -> Result<(), String> {
        self.session
            .call("vim_set_option", &call_args![name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_out_write(&mut self, str: &str) -> Result<(), String> {
        self.session
            .call("vim_out_write", &call_args![str])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_err_write(&mut self, str: &str) -> Result<(), String> {
        self.session
            .call("vim_err_write", &call_args![str])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_report_error(&mut self, str: &str) -> Result<(), String> {
        self.session
            .call("vim_report_error", &call_args![str])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_get_buffers(&mut self) -> Result<Vec<Buffer>, String> {
        self.session
            .call("vim_get_buffers", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_get_current_buffer(&mut self) -> Result<Buffer, String> {
        self.session
            .call("vim_get_current_buffer", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_set_current_buffer(&mut self, buffer: Buffer) -> Result<(), String> {
        self.session
            .call("vim_set_current_buffer", &call_args![buffer])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_get_windows(&mut self) -> Result<Vec<Window>, String> {
        self.session
            .call("vim_get_windows", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_get_current_window(&mut self) -> Result<Window, String> {
        self.session
            .call("vim_get_current_window", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_set_current_window(&mut self, window: Window) -> Result<(), String> {
        self.session
            .call("vim_set_current_window", &call_args![window])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_get_tabpages(&mut self) -> Result<Vec<Tabpage>, String> {
        self.session
            .call("vim_get_tabpages", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_get_current_tabpage(&mut self) -> Result<Tabpage, String> {
        self.session
            .call("vim_get_current_tabpage", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_set_current_tabpage(&mut self, tabpage: Tabpage) -> Result<(), String> {
        self.session
            .call("vim_set_current_tabpage", &call_args![tabpage])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_subscribe(&mut self, event: &str) -> Result<(), String> {
        self.session
            .call("vim_subscribe", &call_args![event])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_unsubscribe(&mut self, event: &str) -> Result<(), String> {
        self.session
            .call("vim_unsubscribe", &call_args![event])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_name_to_color(&mut self, name: &str) -> Result<u64, String> {
        self.session
            .call("vim_name_to_color", &call_args![name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_get_api_info(&mut self) -> Result<Vec<Value>, String> {
        self.session
            .call("vim_get_api_info", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }
}
