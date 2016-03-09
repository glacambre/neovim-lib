// Auto generated 2016-03-09 14:31:41.129355

use neovim::Neovim;
use rmp::Value;
use rmp::value::Integer;

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

pub fn convert_array_of_string(vec: &Vec<String>) -> Value {
    Value::Array(vec.iter().map(|s| Value::String(s.to_owned())).collect())
}

pub fn map_generic_error(err: Value) -> String {
    match err {
        Value::String(val) => val.to_owned(),
        val => format!("Unknow error type: {:?}", val),
    }
}

pub trait NeovimApi {
    fn tabpage_get_windows(&mut self, tabpage: u64) -> Result<Value, String>;
    fn tabpage_get_var(&mut self, tabpage: u64, name: &str) -> Result<Value, String>;
    fn tabpage_set_var(&mut self, tabpage: u64, name: &str, value: Value) -> Result<Value, String>;
    fn tabpage_get_window(&mut self, tabpage: u64) -> Result<Value, String>;
    fn tabpage_is_valid(&mut self, tabpage: u64) -> Result<Value, String>;
    fn window_get_buffer(&mut self, window: u64) -> Result<Value, String>;
    fn window_get_cursor(&mut self, window: u64) -> Result<Value, String>;
    fn window_set_cursor(&mut self, window: u64, pos: (u64, u64)) -> Result<Value, String>;
    fn window_get_height(&mut self, window: u64) -> Result<Value, String>;
    fn window_set_height(&mut self, window: u64, height: u64) -> Result<Value, String>;
    fn window_get_width(&mut self, window: u64) -> Result<Value, String>;
    fn window_set_width(&mut self, window: u64, width: u64) -> Result<Value, String>;
    fn window_get_var(&mut self, window: u64, name: &str) -> Result<Value, String>;
    fn window_set_var(&mut self, window: u64, name: &str, value: Value) -> Result<Value, String>;
    fn window_get_option(&mut self, window: u64, name: &str) -> Result<Value, String>;
    fn window_set_option(&mut self,
                         window: u64,
                         name: &str,
                         value: Value)
                         -> Result<Value, String>;
    fn window_get_position(&mut self, window: u64) -> Result<Value, String>;
    fn window_get_tabpage(&mut self, window: u64) -> Result<Value, String>;
    fn window_is_valid(&mut self, window: u64) -> Result<Value, String>;
    fn buffer_line_count(&mut self, buffer: u64) -> Result<Value, String>;
    fn buffer_get_line(&mut self, buffer: u64, index: u64) -> Result<Value, String>;
    fn buffer_set_line(&mut self, buffer: u64, index: u64, line: &str) -> Result<Value, String>;
    fn buffer_del_line(&mut self, buffer: u64, index: u64) -> Result<Value, String>;
    fn buffer_get_line_slice(&mut self,
                             buffer: u64,
                             start: u64,
                             end: u64,
                             include_start: bool,
                             include_end: bool)
                             -> Result<Value, String>;
    fn buffer_set_line_slice(&mut self,
                             buffer: u64,
                             start: u64,
                             end: u64,
                             include_start: bool,
                             include_end: bool,
                             replacement: Vec<String>)
                             -> Result<Value, String>;
    fn buffer_get_var(&mut self, buffer: u64, name: &str) -> Result<Value, String>;
    fn buffer_set_var(&mut self, buffer: u64, name: &str, value: Value) -> Result<Value, String>;
    fn buffer_get_option(&mut self, buffer: u64, name: &str) -> Result<Value, String>;
    fn buffer_set_option(&mut self,
                         buffer: u64,
                         name: &str,
                         value: Value)
                         -> Result<Value, String>;
    fn buffer_get_number(&mut self, buffer: u64) -> Result<Value, String>;
    fn buffer_get_name(&mut self, buffer: u64) -> Result<Value, String>;
    fn buffer_set_name(&mut self, buffer: u64, name: &str) -> Result<Value, String>;
    fn buffer_is_valid(&mut self, buffer: u64) -> Result<Value, String>;
    fn buffer_insert(&mut self,
                     buffer: u64,
                     lnum: u64,
                     lines: Vec<String>)
                     -> Result<Value, String>;
    fn buffer_get_mark(&mut self, buffer: u64, name: &str) -> Result<Value, String>;
    fn vim_command(&mut self, str: &str) -> Result<Value, String>;
    fn vim_feedkeys(&mut self, keys: &str, mode: &str, escape_csi: bool) -> Result<Value, String>;
    fn vim_input(&mut self, keys: &str) -> Result<Value, String>;
    fn vim_replace_termcodes(&mut self,
                             str: &str,
                             from_part: bool,
                             do_lt: bool,
                             special: bool)
                             -> Result<Value, String>;
    fn vim_command_output(&mut self, str: &str) -> Result<Value, String>;
    fn vim_eval(&mut self, str: &str) -> Result<Value, String>;
    fn vim_call_function(&mut self, fname: &str, args: Vec<Value>) -> Result<Value, String>;
    fn vim_strwidth(&mut self, str: &str) -> Result<Value, String>;
    fn vim_list_runtime_paths(&mut self) -> Result<Value, String>;
    fn vim_change_directory(&mut self, dir: &str) -> Result<Value, String>;
    fn vim_get_current_line(&mut self) -> Result<Value, String>;
    fn vim_set_current_line(&mut self, line: &str) -> Result<Value, String>;
    fn vim_del_current_line(&mut self) -> Result<Value, String>;
    fn vim_get_var(&mut self, name: &str) -> Result<Value, String>;
    fn vim_set_var(&mut self, name: &str, value: Value) -> Result<Value, String>;
    fn vim_get_vvar(&mut self, name: &str) -> Result<Value, String>;
    fn vim_get_option(&mut self, name: &str) -> Result<Value, String>;
    fn vim_set_option(&mut self, name: &str, value: Value) -> Result<Value, String>;
    fn vim_out_write(&mut self, str: &str) -> Result<Value, String>;
    fn vim_err_write(&mut self, str: &str) -> Result<Value, String>;
    fn vim_report_error(&mut self, str: &str) -> Result<Value, String>;
    fn vim_get_buffers(&mut self) -> Result<Value, String>;
    fn vim_get_current_buffer(&mut self) -> Result<Value, String>;
    fn vim_set_current_buffer(&mut self, buffer: u64) -> Result<Value, String>;
    fn vim_get_windows(&mut self) -> Result<Value, String>;
    fn vim_get_current_window(&mut self) -> Result<Value, String>;
    fn vim_set_current_window(&mut self, window: u64) -> Result<Value, String>;
    fn vim_get_tabpages(&mut self) -> Result<Value, String>;
    fn vim_get_current_tabpage(&mut self) -> Result<Value, String>;
    fn vim_set_current_tabpage(&mut self, tabpage: u64) -> Result<Value, String>;
    fn vim_subscribe(&mut self, event: &str) -> Result<Value, String>;
    fn vim_unsubscribe(&mut self, event: &str) -> Result<Value, String>;
    fn vim_name_to_color(&mut self, name: &str) -> Result<Value, String>;
    fn vim_get_api_info(&mut self) -> Result<Value, String>;
    }

impl NeovimApi for Neovim {
    fn tabpage_get_windows(&mut self, tabpage: u64) -> Result<Value, String> {
        self.session
            .call("tabpage_get_windows",
                  &vec![Value::Integer(Integer::U64(tabpage))])
            .map_err(map_generic_error)
    }

    fn tabpage_get_var(&mut self, tabpage: u64, name: &str) -> Result<Value, String> {
        self.session
            .call("tabpage_get_var",
                  &vec![Value::Integer(Integer::U64(tabpage)), Value::String(name.to_owned())])
            .map_err(map_generic_error)
    }

    fn tabpage_set_var(&mut self, tabpage: u64, name: &str, value: Value) -> Result<Value, String> {
        self.session
            .call("tabpage_set_var",
                  &vec![Value::Integer(Integer::U64(tabpage)),
                        Value::String(name.to_owned()),
                        value])
            .map_err(map_generic_error)
    }

    fn tabpage_get_window(&mut self, tabpage: u64) -> Result<Value, String> {
        self.session
            .call("tabpage_get_window",
                  &vec![Value::Integer(Integer::U64(tabpage))])
            .map_err(map_generic_error)
    }

    fn tabpage_is_valid(&mut self, tabpage: u64) -> Result<Value, String> {
        self.session
            .call("tabpage_is_valid",
                  &vec![Value::Integer(Integer::U64(tabpage))])
            .map_err(map_generic_error)
    }

    fn window_get_buffer(&mut self, window: u64) -> Result<Value, String> {
        self.session
            .call("window_get_buffer",
                  &vec![Value::Integer(Integer::U64(window))])
            .map_err(map_generic_error)
    }

    fn window_get_cursor(&mut self, window: u64) -> Result<Value, String> {
        self.session
            .call("window_get_cursor",
                  &vec![Value::Integer(Integer::U64(window))])
            .map_err(map_generic_error)
    }

    fn window_set_cursor(&mut self, window: u64, pos: (u64, u64)) -> Result<Value, String> {
        self.session
            .call("window_set_cursor",
                  &vec![Value::Integer(Integer::U64(window)),
                        Value::Array(vec![Value::Integer(Integer::U64(pos.0)),
                                          Value::Integer(Integer::U64(pos.1))])])
            .map_err(map_generic_error)
    }

    fn window_get_height(&mut self, window: u64) -> Result<Value, String> {
        self.session
            .call("window_get_height",
                  &vec![Value::Integer(Integer::U64(window))])
            .map_err(map_generic_error)
    }

    fn window_set_height(&mut self, window: u64, height: u64) -> Result<Value, String> {
        self.session
            .call("window_set_height",
                  &vec![Value::Integer(Integer::U64(window)), Value::Integer(Integer::U64(height))])
            .map_err(map_generic_error)
    }

    fn window_get_width(&mut self, window: u64) -> Result<Value, String> {
        self.session
            .call("window_get_width",
                  &vec![Value::Integer(Integer::U64(window))])
            .map_err(map_generic_error)
    }

    fn window_set_width(&mut self, window: u64, width: u64) -> Result<Value, String> {
        self.session
            .call("window_set_width",
                  &vec![Value::Integer(Integer::U64(window)), Value::Integer(Integer::U64(width))])
            .map_err(map_generic_error)
    }

    fn window_get_var(&mut self, window: u64, name: &str) -> Result<Value, String> {
        self.session
            .call("window_get_var",
                  &vec![Value::Integer(Integer::U64(window)), Value::String(name.to_owned())])
            .map_err(map_generic_error)
    }

    fn window_set_var(&mut self, window: u64, name: &str, value: Value) -> Result<Value, String> {
        self.session
            .call("window_set_var",
                  &vec![Value::Integer(Integer::U64(window)),
                        Value::String(name.to_owned()),
                        value])
            .map_err(map_generic_error)
    }

    fn window_get_option(&mut self, window: u64, name: &str) -> Result<Value, String> {
        self.session
            .call("window_get_option",
                  &vec![Value::Integer(Integer::U64(window)), Value::String(name.to_owned())])
            .map_err(map_generic_error)
    }

    fn window_set_option(&mut self,
                         window: u64,
                         name: &str,
                         value: Value)
                         -> Result<Value, String> {
        self.session
            .call("window_set_option",
                  &vec![Value::Integer(Integer::U64(window)),
                        Value::String(name.to_owned()),
                        value])
            .map_err(map_generic_error)
    }

    fn window_get_position(&mut self, window: u64) -> Result<Value, String> {
        self.session
            .call("window_get_position",
                  &vec![Value::Integer(Integer::U64(window))])
            .map_err(map_generic_error)
    }

    fn window_get_tabpage(&mut self, window: u64) -> Result<Value, String> {
        self.session
            .call("window_get_tabpage",
                  &vec![Value::Integer(Integer::U64(window))])
            .map_err(map_generic_error)
    }

    fn window_is_valid(&mut self, window: u64) -> Result<Value, String> {
        self.session
            .call("window_is_valid",
                  &vec![Value::Integer(Integer::U64(window))])
            .map_err(map_generic_error)
    }

    fn buffer_line_count(&mut self, buffer: u64) -> Result<Value, String> {
        self.session
            .call("buffer_line_count",
                  &vec![Value::Integer(Integer::U64(buffer))])
            .map_err(map_generic_error)
    }

    fn buffer_get_line(&mut self, buffer: u64, index: u64) -> Result<Value, String> {
        self.session
            .call("buffer_get_line",
                  &vec![Value::Integer(Integer::U64(buffer)), Value::Integer(Integer::U64(index))])
            .map_err(map_generic_error)
    }

    fn buffer_set_line(&mut self, buffer: u64, index: u64, line: &str) -> Result<Value, String> {
        self.session
            .call("buffer_set_line",
                  &vec![Value::Integer(Integer::U64(buffer)),
                        Value::Integer(Integer::U64(index)),
                        Value::String(line.to_owned())])
            .map_err(map_generic_error)
    }

    fn buffer_del_line(&mut self, buffer: u64, index: u64) -> Result<Value, String> {
        self.session
            .call("buffer_del_line",
                  &vec![Value::Integer(Integer::U64(buffer)), Value::Integer(Integer::U64(index))])
            .map_err(map_generic_error)
    }

    fn buffer_get_line_slice(&mut self,
                             buffer: u64,
                             start: u64,
                             end: u64,
                             include_start: bool,
                             include_end: bool)
                             -> Result<Value, String> {
        self.session
            .call("buffer_get_line_slice",
                  &vec![Value::Integer(Integer::U64(buffer)),
                        Value::Integer(Integer::U64(start)),
                        Value::Integer(Integer::U64(end)),
                        Value::Boolean(include_start),
                        Value::Boolean(include_end)])
            .map_err(map_generic_error)
    }

    fn buffer_set_line_slice(&mut self,
                             buffer: u64,
                             start: u64,
                             end: u64,
                             include_start: bool,
                             include_end: bool,
                             replacement: Vec<String>)
                             -> Result<Value, String> {
        self.session
            .call("buffer_set_line_slice",
                  &vec![Value::Integer(Integer::U64(buffer)),
                        Value::Integer(Integer::U64(start)),
                        Value::Integer(Integer::U64(end)),
                        Value::Boolean(include_start),
                        Value::Boolean(include_end),
                        convert_array_of_string(&replacement)])
            .map_err(map_generic_error)
    }

    fn buffer_get_var(&mut self, buffer: u64, name: &str) -> Result<Value, String> {
        self.session
            .call("buffer_get_var",
                  &vec![Value::Integer(Integer::U64(buffer)), Value::String(name.to_owned())])
            .map_err(map_generic_error)
    }

    fn buffer_set_var(&mut self, buffer: u64, name: &str, value: Value) -> Result<Value, String> {
        self.session
            .call("buffer_set_var",
                  &vec![Value::Integer(Integer::U64(buffer)),
                        Value::String(name.to_owned()),
                        value])
            .map_err(map_generic_error)
    }

    fn buffer_get_option(&mut self, buffer: u64, name: &str) -> Result<Value, String> {
        self.session
            .call("buffer_get_option",
                  &vec![Value::Integer(Integer::U64(buffer)), Value::String(name.to_owned())])
            .map_err(map_generic_error)
    }

    fn buffer_set_option(&mut self,
                         buffer: u64,
                         name: &str,
                         value: Value)
                         -> Result<Value, String> {
        self.session
            .call("buffer_set_option",
                  &vec![Value::Integer(Integer::U64(buffer)),
                        Value::String(name.to_owned()),
                        value])
            .map_err(map_generic_error)
    }

    fn buffer_get_number(&mut self, buffer: u64) -> Result<Value, String> {
        self.session
            .call("buffer_get_number",
                  &vec![Value::Integer(Integer::U64(buffer))])
            .map_err(map_generic_error)
    }

    fn buffer_get_name(&mut self, buffer: u64) -> Result<Value, String> {
        self.session
            .call("buffer_get_name",
                  &vec![Value::Integer(Integer::U64(buffer))])
            .map_err(map_generic_error)
    }

    fn buffer_set_name(&mut self, buffer: u64, name: &str) -> Result<Value, String> {
        self.session
            .call("buffer_set_name",
                  &vec![Value::Integer(Integer::U64(buffer)), Value::String(name.to_owned())])
            .map_err(map_generic_error)
    }

    fn buffer_is_valid(&mut self, buffer: u64) -> Result<Value, String> {
        self.session
            .call("buffer_is_valid",
                  &vec![Value::Integer(Integer::U64(buffer))])
            .map_err(map_generic_error)
    }

    fn buffer_insert(&mut self,
                     buffer: u64,
                     lnum: u64,
                     lines: Vec<String>)
                     -> Result<Value, String> {
        self.session
            .call("buffer_insert",
                  &vec![Value::Integer(Integer::U64(buffer)),
                        Value::Integer(Integer::U64(lnum)),
                        convert_array_of_string(&lines)])
            .map_err(map_generic_error)
    }

    fn buffer_get_mark(&mut self, buffer: u64, name: &str) -> Result<Value, String> {
        self.session
            .call("buffer_get_mark",
                  &vec![Value::Integer(Integer::U64(buffer)), Value::String(name.to_owned())])
            .map_err(map_generic_error)
    }

    fn vim_command(&mut self, str: &str) -> Result<Value, String> {
        self.session
            .call("vim_command", &vec![Value::String(str.to_owned())])
            .map_err(map_generic_error)
    }

    fn vim_feedkeys(&mut self, keys: &str, mode: &str, escape_csi: bool) -> Result<Value, String> {
        self.session
            .call("vim_feedkeys",
                  &vec![Value::String(keys.to_owned()),
                        Value::String(mode.to_owned()),
                        Value::Boolean(escape_csi)])
            .map_err(map_generic_error)
    }

    fn vim_input(&mut self, keys: &str) -> Result<Value, String> {
        self.session
            .call("vim_input", &vec![Value::String(keys.to_owned())])
            .map_err(map_generic_error)
    }

    fn vim_replace_termcodes(&mut self,
                             str: &str,
                             from_part: bool,
                             do_lt: bool,
                             special: bool)
                             -> Result<Value, String> {
        self.session
            .call("vim_replace_termcodes",
                  &vec![Value::String(str.to_owned()),
                        Value::Boolean(from_part),
                        Value::Boolean(do_lt),
                        Value::Boolean(special)])
            .map_err(map_generic_error)
    }

    fn vim_command_output(&mut self, str: &str) -> Result<Value, String> {
        self.session
            .call("vim_command_output", &vec![Value::String(str.to_owned())])
            .map_err(map_generic_error)
    }

    fn vim_eval(&mut self, str: &str) -> Result<Value, String> {
        self.session
            .call("vim_eval", &vec![Value::String(str.to_owned())])
            .map_err(map_generic_error)
    }

    fn vim_call_function(&mut self, fname: &str, args: Vec<Value>) -> Result<Value, String> {
        self.session
            .call("vim_call_function",
                  &vec![Value::String(fname.to_owned()), Value::Array(args)])
            .map_err(map_generic_error)
    }

    fn vim_strwidth(&mut self, str: &str) -> Result<Value, String> {
        self.session
            .call("vim_strwidth", &vec![Value::String(str.to_owned())])
            .map_err(map_generic_error)
    }

    fn vim_list_runtime_paths(&mut self) -> Result<Value, String> {
        self.session
            .call("vim_list_runtime_paths", &vec![])
            .map_err(map_generic_error)
    }

    fn vim_change_directory(&mut self, dir: &str) -> Result<Value, String> {
        self.session
            .call("vim_change_directory", &vec![Value::String(dir.to_owned())])
            .map_err(map_generic_error)
    }

    fn vim_get_current_line(&mut self) -> Result<Value, String> {
        self.session
            .call("vim_get_current_line", &vec![])
            .map_err(map_generic_error)
    }

    fn vim_set_current_line(&mut self, line: &str) -> Result<Value, String> {
        self.session
            .call("vim_set_current_line",
                  &vec![Value::String(line.to_owned())])
            .map_err(map_generic_error)
    }

    fn vim_del_current_line(&mut self) -> Result<Value, String> {
        self.session
            .call("vim_del_current_line", &vec![])
            .map_err(map_generic_error)
    }

    fn vim_get_var(&mut self, name: &str) -> Result<Value, String> {
        self.session
            .call("vim_get_var", &vec![Value::String(name.to_owned())])
            .map_err(map_generic_error)
    }

    fn vim_set_var(&mut self, name: &str, value: Value) -> Result<Value, String> {
        self.session
            .call("vim_set_var", &vec![Value::String(name.to_owned()), value])
            .map_err(map_generic_error)
    }

    fn vim_get_vvar(&mut self, name: &str) -> Result<Value, String> {
        self.session
            .call("vim_get_vvar", &vec![Value::String(name.to_owned())])
            .map_err(map_generic_error)
    }

    fn vim_get_option(&mut self, name: &str) -> Result<Value, String> {
        self.session
            .call("vim_get_option", &vec![Value::String(name.to_owned())])
            .map_err(map_generic_error)
    }

    fn vim_set_option(&mut self, name: &str, value: Value) -> Result<Value, String> {
        self.session
            .call("vim_set_option",
                  &vec![Value::String(name.to_owned()), value])
            .map_err(map_generic_error)
    }

    fn vim_out_write(&mut self, str: &str) -> Result<Value, String> {
        self.session
            .call("vim_out_write", &vec![Value::String(str.to_owned())])
            .map_err(map_generic_error)
    }

    fn vim_err_write(&mut self, str: &str) -> Result<Value, String> {
        self.session
            .call("vim_err_write", &vec![Value::String(str.to_owned())])
            .map_err(map_generic_error)
    }

    fn vim_report_error(&mut self, str: &str) -> Result<Value, String> {
        self.session
            .call("vim_report_error", &vec![Value::String(str.to_owned())])
            .map_err(map_generic_error)
    }

    fn vim_get_buffers(&mut self) -> Result<Value, String> {
        self.session
            .call("vim_get_buffers", &vec![])
            .map_err(map_generic_error)
    }

    fn vim_get_current_buffer(&mut self) -> Result<Value, String> {
        self.session
            .call("vim_get_current_buffer", &vec![])
            .map_err(map_generic_error)
    }

    fn vim_set_current_buffer(&mut self, buffer: u64) -> Result<Value, String> {
        self.session
            .call("vim_set_current_buffer",
                  &vec![Value::Integer(Integer::U64(buffer))])
            .map_err(map_generic_error)
    }

    fn vim_get_windows(&mut self) -> Result<Value, String> {
        self.session
            .call("vim_get_windows", &vec![])
            .map_err(map_generic_error)
    }

    fn vim_get_current_window(&mut self) -> Result<Value, String> {
        self.session
            .call("vim_get_current_window", &vec![])
            .map_err(map_generic_error)
    }

    fn vim_set_current_window(&mut self, window: u64) -> Result<Value, String> {
        self.session
            .call("vim_set_current_window",
                  &vec![Value::Integer(Integer::U64(window))])
            .map_err(map_generic_error)
    }

    fn vim_get_tabpages(&mut self) -> Result<Value, String> {
        self.session
            .call("vim_get_tabpages", &vec![])
            .map_err(map_generic_error)
    }

    fn vim_get_current_tabpage(&mut self) -> Result<Value, String> {
        self.session
            .call("vim_get_current_tabpage", &vec![])
            .map_err(map_generic_error)
    }

    fn vim_set_current_tabpage(&mut self, tabpage: u64) -> Result<Value, String> {
        self.session
            .call("vim_set_current_tabpage",
                  &vec![Value::Integer(Integer::U64(tabpage))])
            .map_err(map_generic_error)
    }

    fn vim_subscribe(&mut self, event: &str) -> Result<Value, String> {
        self.session
            .call("vim_subscribe", &vec![Value::String(event.to_owned())])
            .map_err(map_generic_error)
    }

    fn vim_unsubscribe(&mut self, event: &str) -> Result<Value, String> {
        self.session
            .call("vim_unsubscribe", &vec![Value::String(event.to_owned())])
            .map_err(map_generic_error)
    }

    fn vim_name_to_color(&mut self, name: &str) -> Result<Value, String> {
        self.session
            .call("vim_name_to_color", &vec![Value::String(name.to_owned())])
            .map_err(map_generic_error)
    }

    fn vim_get_api_info(&mut self) -> Result<Value, String> {
        self.session
            .call("vim_get_api_info", &vec![])
            .map_err(map_generic_error)
    }
}
