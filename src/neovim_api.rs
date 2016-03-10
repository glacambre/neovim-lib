// Auto generated 2016-03-10 18:55:49.209000

use neovim::*;
use rmp::Value;
use rpc::*;

pub enum ExtType {
    Buffer,
    Window,
    Tabpage,
}

impl ExtType {
    pub fn from_typ(typ: u64) -> Result<ExtType, String> {
        match typ {
            0 => Ok(ExtType::Buffer),
            1 => Ok(ExtType::Window),
            2 => Ok(ExtType::Tabpage),
            _ => Err("Not supported type".to_owned()),
        }
    }
}

pub trait NeovimApi {
    fn buffer_line_count(&mut self, buffer: u64) -> Result<u64, String>;
    fn buffer_get_line(&mut self, buffer: u64, index: u64) -> Result<String, String>;
    fn buffer_set_line(&mut self, buffer: u64, index: u64, line: &str) -> Result<(), String>;
    fn buffer_del_line(&mut self, buffer: u64, index: u64) -> Result<(), String>;
    fn buffer_get_line_slice(&mut self,
                             buffer: u64,
                             start: u64,
                             end: u64,
                             include_start: bool,
                             include_end: bool)
                             -> Result<Vec<String>, String>;
    fn buffer_set_line_slice(&mut self,
                             buffer: u64,
                             start: u64,
                             end: u64,
                             include_start: bool,
                             include_end: bool,
                             replacement: Vec<String>)
                             -> Result<(), String>;
    fn buffer_get_var(&mut self, buffer: u64, name: &str) -> Result<Value, String>;
    fn buffer_set_var(&mut self, buffer: u64, name: &str, value: Value) -> Result<Value, String>;
    fn buffer_get_option(&mut self, buffer: u64, name: &str) -> Result<Value, String>;
    fn buffer_set_option(&mut self, buffer: u64, name: &str, value: Value) -> Result<(), String>;
    fn buffer_get_number(&mut self, buffer: u64) -> Result<u64, String>;
    fn buffer_get_name(&mut self, buffer: u64) -> Result<String, String>;
    fn buffer_set_name(&mut self, buffer: u64, name: &str) -> Result<(), String>;
    fn buffer_is_valid(&mut self, buffer: u64) -> Result<bool, String>;
    fn buffer_insert(&mut self, buffer: u64, lnum: u64, lines: Vec<String>) -> Result<(), String>;
    fn buffer_get_mark(&mut self, buffer: u64, name: &str) -> Result<(u64, u64), String>;
    fn buffer_add_highlight(&mut self,
                            buffer: u64,
                            src_id: u64,
                            hl_group: &str,
                            line: u64,
                            col_start: u64,
                            col_end: u64)
                            -> Result<u64, String>;
    fn buffer_clear_highlight(&mut self,
                              buffer: u64,
                              src_id: u64,
                              line_start: u64,
                              line_end: u64)
                              -> Result<(), String>;
    fn tabpage_get_windows(&mut self, tabpage: u64) -> Result<Vec<u64>, String>;
    fn tabpage_get_var(&mut self, tabpage: u64, name: &str) -> Result<Value, String>;
    fn tabpage_set_var(&mut self, tabpage: u64, name: &str, value: Value) -> Result<Value, String>;
    fn tabpage_get_window(&mut self, tabpage: u64) -> Result<u64, String>;
    fn tabpage_is_valid(&mut self, tabpage: u64) -> Result<bool, String>;
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
    fn vim_get_buffers(&mut self) -> Result<Vec<u64>, String>;
    fn vim_get_current_buffer(&mut self) -> Result<u64, String>;
    fn vim_set_current_buffer(&mut self, buffer: u64) -> Result<(), String>;
    fn vim_get_windows(&mut self) -> Result<Vec<u64>, String>;
    fn vim_get_current_window(&mut self) -> Result<u64, String>;
    fn vim_set_current_window(&mut self, window: u64) -> Result<(), String>;
    fn vim_get_tabpages(&mut self) -> Result<Vec<u64>, String>;
    fn vim_get_current_tabpage(&mut self) -> Result<u64, String>;
    fn vim_set_current_tabpage(&mut self, tabpage: u64) -> Result<(), String>;
    fn vim_subscribe(&mut self, event: &str) -> Result<(), String>;
    fn vim_unsubscribe(&mut self, event: &str) -> Result<(), String>;
    fn vim_name_to_color(&mut self, name: &str) -> Result<u64, String>;
    fn vim_get_api_info(&mut self) -> Result<Vec<Value>, String>;
    fn window_get_buffer(&mut self, window: u64) -> Result<u64, String>;
    fn window_get_cursor(&mut self, window: u64) -> Result<(u64, u64), String>;
    fn window_set_cursor(&mut self, window: u64, pos: (u64, u64)) -> Result<(), String>;
    fn window_get_height(&mut self, window: u64) -> Result<u64, String>;
    fn window_set_height(&mut self, window: u64, height: u64) -> Result<(), String>;
    fn window_get_width(&mut self, window: u64) -> Result<u64, String>;
    fn window_set_width(&mut self, window: u64, width: u64) -> Result<(), String>;
    fn window_get_var(&mut self, window: u64, name: &str) -> Result<Value, String>;
    fn window_set_var(&mut self, window: u64, name: &str, value: Value) -> Result<Value, String>;
    fn window_get_option(&mut self, window: u64, name: &str) -> Result<Value, String>;
    fn window_set_option(&mut self, window: u64, name: &str, value: Value) -> Result<(), String>;
    fn window_get_position(&mut self, window: u64) -> Result<(u64, u64), String>;
    fn window_get_tabpage(&mut self, window: u64) -> Result<u64, String>;
    fn window_is_valid(&mut self, window: u64) -> Result<bool, String>;
    }

impl NeovimApi for Neovim {
    fn buffer_line_count(&mut self, buffer: u64) -> Result<u64, String> {
        self.session
            .call("buffer_line_count", &call_args![buffer])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buffer_get_line(&mut self, buffer: u64, index: u64) -> Result<String, String> {
        self.session
            .call("buffer_get_line", &call_args![buffer, index])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buffer_set_line(&mut self, buffer: u64, index: u64, line: &str) -> Result<(), String> {
        self.session
            .call("buffer_set_line", &call_args![buffer, index, line])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buffer_del_line(&mut self, buffer: u64, index: u64) -> Result<(), String> {
        self.session
            .call("buffer_del_line", &call_args![buffer, index])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buffer_get_line_slice(&mut self,
                             buffer: u64,
                             start: u64,
                             end: u64,
                             include_start: bool,
                             include_end: bool)
                             -> Result<Vec<String>, String> {
        self.session
            .call("buffer_get_line_slice",
                  &call_args![buffer, start, end, include_start, include_end])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buffer_set_line_slice(&mut self,
                             buffer: u64,
                             start: u64,
                             end: u64,
                             include_start: bool,
                             include_end: bool,
                             replacement: Vec<String>)
                             -> Result<(), String> {
        self.session
            .call("buffer_set_line_slice",
                  &call_args![buffer, start, end, include_start, include_end, replacement])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buffer_get_var(&mut self, buffer: u64, name: &str) -> Result<Value, String> {
        self.session
            .call("buffer_get_var", &call_args![buffer, name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buffer_set_var(&mut self, buffer: u64, name: &str, value: Value) -> Result<Value, String> {
        self.session
            .call("buffer_set_var", &call_args![buffer, name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buffer_get_option(&mut self, buffer: u64, name: &str) -> Result<Value, String> {
        self.session
            .call("buffer_get_option", &call_args![buffer, name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buffer_set_option(&mut self, buffer: u64, name: &str, value: Value) -> Result<(), String> {
        self.session
            .call("buffer_set_option", &call_args![buffer, name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buffer_get_number(&mut self, buffer: u64) -> Result<u64, String> {
        self.session
            .call("buffer_get_number", &call_args![buffer])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buffer_get_name(&mut self, buffer: u64) -> Result<String, String> {
        self.session
            .call("buffer_get_name", &call_args![buffer])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buffer_set_name(&mut self, buffer: u64, name: &str) -> Result<(), String> {
        self.session
            .call("buffer_set_name", &call_args![buffer, name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buffer_is_valid(&mut self, buffer: u64) -> Result<bool, String> {
        self.session
            .call("buffer_is_valid", &call_args![buffer])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buffer_insert(&mut self, buffer: u64, lnum: u64, lines: Vec<String>) -> Result<(), String> {
        self.session
            .call("buffer_insert", &call_args![buffer, lnum, lines])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buffer_get_mark(&mut self, buffer: u64, name: &str) -> Result<(u64, u64), String> {
        self.session
            .call("buffer_get_mark", &call_args![buffer, name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buffer_add_highlight(&mut self,
                            buffer: u64,
                            src_id: u64,
                            hl_group: &str,
                            line: u64,
                            col_start: u64,
                            col_end: u64)
                            -> Result<u64, String> {
        self.session
            .call("buffer_add_highlight",
                  &call_args![buffer, src_id, hl_group, line, col_start, col_end])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buffer_clear_highlight(&mut self,
                              buffer: u64,
                              src_id: u64,
                              line_start: u64,
                              line_end: u64)
                              -> Result<(), String> {
        self.session
            .call("buffer_clear_highlight",
                  &call_args![buffer, src_id, line_start, line_end])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn tabpage_get_windows(&mut self, tabpage: u64) -> Result<Vec<u64>, String> {
        self.session
            .call("tabpage_get_windows", &call_args![tabpage])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn tabpage_get_var(&mut self, tabpage: u64, name: &str) -> Result<Value, String> {
        self.session
            .call("tabpage_get_var", &call_args![tabpage, name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn tabpage_set_var(&mut self, tabpage: u64, name: &str, value: Value) -> Result<Value, String> {
        self.session
            .call("tabpage_set_var", &call_args![tabpage, name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn tabpage_get_window(&mut self, tabpage: u64) -> Result<u64, String> {
        self.session
            .call("tabpage_get_window", &call_args![tabpage])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn tabpage_is_valid(&mut self, tabpage: u64) -> Result<bool, String> {
        self.session
            .call("tabpage_is_valid", &call_args![tabpage])
            .map(map_result)
            .map_err(map_generic_error)
    }

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

    fn vim_get_buffers(&mut self) -> Result<Vec<u64>, String> {
        self.session
            .call("vim_get_buffers", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_get_current_buffer(&mut self) -> Result<u64, String> {
        self.session
            .call("vim_get_current_buffer", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_set_current_buffer(&mut self, buffer: u64) -> Result<(), String> {
        self.session
            .call("vim_set_current_buffer", &call_args![buffer])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_get_windows(&mut self) -> Result<Vec<u64>, String> {
        self.session
            .call("vim_get_windows", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_get_current_window(&mut self) -> Result<u64, String> {
        self.session
            .call("vim_get_current_window", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_set_current_window(&mut self, window: u64) -> Result<(), String> {
        self.session
            .call("vim_set_current_window", &call_args![window])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_get_tabpages(&mut self) -> Result<Vec<u64>, String> {
        self.session
            .call("vim_get_tabpages", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_get_current_tabpage(&mut self) -> Result<u64, String> {
        self.session
            .call("vim_get_current_tabpage", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn vim_set_current_tabpage(&mut self, tabpage: u64) -> Result<(), String> {
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

    fn window_get_buffer(&mut self, window: u64) -> Result<u64, String> {
        self.session
            .call("window_get_buffer", &call_args![window])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn window_get_cursor(&mut self, window: u64) -> Result<(u64, u64), String> {
        self.session
            .call("window_get_cursor", &call_args![window])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn window_set_cursor(&mut self, window: u64, pos: (u64, u64)) -> Result<(), String> {
        self.session
            .call("window_set_cursor", &call_args![window, pos])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn window_get_height(&mut self, window: u64) -> Result<u64, String> {
        self.session
            .call("window_get_height", &call_args![window])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn window_set_height(&mut self, window: u64, height: u64) -> Result<(), String> {
        self.session
            .call("window_set_height", &call_args![window, height])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn window_get_width(&mut self, window: u64) -> Result<u64, String> {
        self.session
            .call("window_get_width", &call_args![window])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn window_set_width(&mut self, window: u64, width: u64) -> Result<(), String> {
        self.session
            .call("window_set_width", &call_args![window, width])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn window_get_var(&mut self, window: u64, name: &str) -> Result<Value, String> {
        self.session
            .call("window_get_var", &call_args![window, name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn window_set_var(&mut self, window: u64, name: &str, value: Value) -> Result<Value, String> {
        self.session
            .call("window_set_var", &call_args![window, name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn window_get_option(&mut self, window: u64, name: &str) -> Result<Value, String> {
        self.session
            .call("window_get_option", &call_args![window, name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn window_set_option(&mut self, window: u64, name: &str, value: Value) -> Result<(), String> {
        self.session
            .call("window_set_option", &call_args![window, name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn window_get_position(&mut self, window: u64) -> Result<(u64, u64), String> {
        self.session
            .call("window_get_position", &call_args![window])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn window_get_tabpage(&mut self, window: u64) -> Result<u64, String> {
        self.session
            .call("window_get_tabpage", &call_args![window])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn window_is_valid(&mut self, window: u64) -> Result<bool, String> {
        self.session
            .call("window_is_valid", &call_args![window])
            .map(map_result)
            .map_err(map_generic_error)
    }
}
