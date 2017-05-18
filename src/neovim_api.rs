// Auto generated 2017-05-18 13:27:41.539000

use neovim::*;
use rpc::*;

pub struct Buffer {
    code_data: Value,
}

impl Buffer {
    pub fn new(code_data: Value) -> Buffer {
        Buffer { code_data: code_data }
    }

    pub fn get_line(&self, neovim: &mut Neovim, index: u64) -> Result<String, CallError> {
        neovim
            .session
            .call("buffer_get_line",
                  &call_args![self.code_data.clone(), index])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn set_line(&self, neovim: &mut Neovim, index: u64, line: &str) -> Result<(), CallError> {
        neovim
            .session
            .call("buffer_set_line",
                  &call_args![self.code_data.clone(), index, line])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn del_line(&self, neovim: &mut Neovim, index: u64) -> Result<(), CallError> {
        neovim
            .session
            .call("buffer_del_line",
                  &call_args![self.code_data.clone(), index])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn get_line_slice(&self,
                          neovim: &mut Neovim,
                          start: u64,
                          end: u64,
                          include_start: bool,
                          include_end: bool)
                          -> Result<Vec<String>, CallError> {
        neovim
            .session
            .call("buffer_get_line_slice",
                  &call_args![self.code_data.clone(),
                              start,
                              end,
                              include_start,
                              include_end])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn set_line_slice(&self,
                          neovim: &mut Neovim,
                          start: u64,
                          end: u64,
                          include_start: bool,
                          include_end: bool,
                          replacement: Vec<String>)
                          -> Result<(), CallError> {
        neovim
            .session
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
    pub fn set_var(&self,
                   neovim: &mut Neovim,
                   name: &str,
                   value: Value)
                   -> Result<Value, CallError> {
        neovim
            .session
            .call("buffer_set_var",
                  &call_args![self.code_data.clone(), name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn del_var(&self, neovim: &mut Neovim, name: &str) -> Result<Value, CallError> {
        neovim
            .session
            .call("buffer_del_var", &call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn insert(&self,
                  neovim: &mut Neovim,
                  lnum: u64,
                  lines: Vec<String>)
                  -> Result<(), CallError> {
        neovim
            .session
            .call("buffer_insert",
                  &call_args![self.code_data.clone(), lnum, lines])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn line_count(&self, neovim: &mut Neovim) -> Result<u64, CallError> {
        neovim
            .session
            .call("buffer_line_count", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn get_lines(&self,
                     neovim: &mut Neovim,
                     start: u64,
                     end: u64,
                     strict_indexing: bool)
                     -> Result<Vec<String>, CallError> {
        neovim
            .session
            .call("buffer_get_lines",
                  &call_args![self.code_data.clone(), start, end, strict_indexing])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn set_lines(&self,
                     neovim: &mut Neovim,
                     start: u64,
                     end: u64,
                     strict_indexing: bool,
                     replacement: Vec<String>)
                     -> Result<(), CallError> {
        neovim
            .session
            .call("buffer_set_lines",
                  &call_args![self.code_data.clone(),
                              start,
                              end,
                              strict_indexing,
                              replacement])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn get_var(&self, neovim: &mut Neovim, name: &str) -> Result<Value, CallError> {
        neovim
            .session
            .call("buffer_get_var", &call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn get_option(&self, neovim: &mut Neovim, name: &str) -> Result<Value, CallError> {
        neovim
            .session
            .call("buffer_get_option",
                  &call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn set_option(&self,
                      neovim: &mut Neovim,
                      name: &str,
                      value: Value)
                      -> Result<(), CallError> {
        neovim
            .session
            .call("buffer_set_option",
                  &call_args![self.code_data.clone(), name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn get_number(&self, neovim: &mut Neovim) -> Result<u64, CallError> {
        neovim
            .session
            .call("buffer_get_number", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn get_name(&self, neovim: &mut Neovim) -> Result<String, CallError> {
        neovim
            .session
            .call("buffer_get_name", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn set_name(&self, neovim: &mut Neovim, name: &str) -> Result<(), CallError> {
        neovim
            .session
            .call("buffer_set_name", &call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn is_valid(&self, neovim: &mut Neovim) -> Result<bool, CallError> {
        neovim
            .session
            .call("buffer_is_valid", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn get_mark(&self, neovim: &mut Neovim, name: &str) -> Result<(u64, u64), CallError> {
        neovim
            .session
            .call("buffer_get_mark", &call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn add_highlight(&self,
                         neovim: &mut Neovim,
                         src_id: u64,
                         hl_group: &str,
                         line: u64,
                         col_start: u64,
                         col_end: u64)
                         -> Result<u64, CallError> {
        neovim
            .session
            .call("buffer_add_highlight",
                  &call_args![self.code_data.clone(),
                              src_id,
                              hl_group,
                              line,
                              col_start,
                              col_end])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn clear_highlight(&self,
                           neovim: &mut Neovim,
                           src_id: u64,
                           line_start: u64,
                           line_end: u64)
                           -> Result<(), CallError> {
        neovim
            .session
            .call("buffer_clear_highlight",
                  &call_args![self.code_data.clone(), src_id, line_start, line_end])
            .map(map_result)
            .map_err(map_generic_error)
    }
}

pub struct Window {
    code_data: Value,
}

impl Window {
    pub fn new(code_data: Value) -> Window {
        Window { code_data: code_data }
    }

    pub fn set_var(&self,
                   neovim: &mut Neovim,
                   name: &str,
                   value: Value)
                   -> Result<Value, CallError> {
        neovim
            .session
            .call("window_set_var",
                  &call_args![self.code_data.clone(), name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn del_var(&self, neovim: &mut Neovim, name: &str) -> Result<Value, CallError> {
        neovim
            .session
            .call("window_del_var", &call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn get_buffer(&self, neovim: &mut Neovim) -> Result<Buffer, CallError> {
        neovim
            .session
            .call("window_get_buffer", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn get_cursor(&self, neovim: &mut Neovim) -> Result<(u64, u64), CallError> {
        neovim
            .session
            .call("window_get_cursor", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn set_cursor(&self, neovim: &mut Neovim, pos: (u64, u64)) -> Result<(), CallError> {
        neovim
            .session
            .call("window_set_cursor",
                  &call_args![self.code_data.clone(), pos])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn get_height(&self, neovim: &mut Neovim) -> Result<u64, CallError> {
        neovim
            .session
            .call("window_get_height", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn set_height(&self, neovim: &mut Neovim, height: u64) -> Result<(), CallError> {
        neovim
            .session
            .call("window_set_height",
                  &call_args![self.code_data.clone(), height])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn get_width(&self, neovim: &mut Neovim) -> Result<u64, CallError> {
        neovim
            .session
            .call("window_get_width", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn set_width(&self, neovim: &mut Neovim, width: u64) -> Result<(), CallError> {
        neovim
            .session
            .call("window_set_width",
                  &call_args![self.code_data.clone(), width])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn get_var(&self, neovim: &mut Neovim, name: &str) -> Result<Value, CallError> {
        neovim
            .session
            .call("window_get_var", &call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn get_option(&self, neovim: &mut Neovim, name: &str) -> Result<Value, CallError> {
        neovim
            .session
            .call("window_get_option",
                  &call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn set_option(&self,
                      neovim: &mut Neovim,
                      name: &str,
                      value: Value)
                      -> Result<(), CallError> {
        neovim
            .session
            .call("window_set_option",
                  &call_args![self.code_data.clone(), name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn get_position(&self, neovim: &mut Neovim) -> Result<(u64, u64), CallError> {
        neovim
            .session
            .call("window_get_position", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn get_tabpage(&self, neovim: &mut Neovim) -> Result<Tabpage, CallError> {
        neovim
            .session
            .call("window_get_tabpage", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn is_valid(&self, neovim: &mut Neovim) -> Result<bool, CallError> {
        neovim
            .session
            .call("window_is_valid", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
}

pub struct Tabpage {
    code_data: Value,
}

impl Tabpage {
    pub fn new(code_data: Value) -> Tabpage {
        Tabpage { code_data: code_data }
    }

    pub fn set_var(&self,
                   neovim: &mut Neovim,
                   name: &str,
                   value: Value)
                   -> Result<Value, CallError> {
        neovim
            .session
            .call("tabpage_set_var",
                  &call_args![self.code_data.clone(), name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn del_var(&self, neovim: &mut Neovim, name: &str) -> Result<Value, CallError> {
        neovim
            .session
            .call("tabpage_del_var", &call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn get_windows(&self, neovim: &mut Neovim) -> Result<Vec<Window>, CallError> {
        neovim
            .session
            .call("tabpage_get_windows", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn get_var(&self, neovim: &mut Neovim, name: &str) -> Result<Value, CallError> {
        neovim
            .session
            .call("tabpage_get_var", &call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn get_window(&self, neovim: &mut Neovim) -> Result<Window, CallError> {
        neovim
            .session
            .call("tabpage_get_window", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    pub fn is_valid(&self, neovim: &mut Neovim) -> Result<bool, CallError> {
        neovim
            .session
            .call("tabpage_is_valid", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
}


impl FromVal<Value> for Buffer {
    fn from_val(val: Value) -> Self {
        Buffer::new(val)
    }
}

impl<'a> IntoVal<Value> for &'a Buffer {
    fn into_val(self) -> Value {
        self.code_data.clone()
    }
}
impl FromVal<Value> for Window {
    fn from_val(val: Value) -> Self {
        Window::new(val)
    }
}

impl<'a> IntoVal<Value> for &'a Window {
    fn into_val(self) -> Value {
        self.code_data.clone()
    }
}
impl FromVal<Value> for Tabpage {
    fn from_val(val: Value) -> Self {
        Tabpage::new(val)
    }
}

impl<'a> IntoVal<Value> for &'a Tabpage {
    fn into_val(self) -> Value {
        self.code_data.clone()
    }
}

pub trait NeovimApi {
    fn buf_line_count(&mut self, buffer: &Buffer) -> Result<u64, CallError>;
    fn buf_get_lines(&mut self,
                     buffer: &Buffer,
                     start: u64,
                     end: u64,
                     strict_indexing: bool)
                     -> Result<Vec<String>, CallError>;
    fn buf_set_lines(&mut self,
                     buffer: &Buffer,
                     start: u64,
                     end: u64,
                     strict_indexing: bool,
                     replacement: Vec<String>)
                     -> Result<(), CallError>;
    fn buf_get_var(&mut self, buffer: &Buffer, name: &str) -> Result<Value, CallError>;
    fn buf_get_changedtick(&mut self, buffer: &Buffer) -> Result<u64, CallError>;
    fn buf_set_var(&mut self, buffer: &Buffer, name: &str, value: Value) -> Result<(), CallError>;
    fn buf_del_var(&mut self, buffer: &Buffer, name: &str) -> Result<(), CallError>;
    fn buf_get_option(&mut self, buffer: &Buffer, name: &str) -> Result<Value, CallError>;
    fn buf_set_option(&mut self,
                      buffer: &Buffer,
                      name: &str,
                      value: Value)
                      -> Result<(), CallError>;
    fn buf_get_number(&mut self, buffer: &Buffer) -> Result<u64, CallError>;
    fn buf_get_name(&mut self, buffer: &Buffer) -> Result<String, CallError>;
    fn buf_set_name(&mut self, buffer: &Buffer, name: &str) -> Result<(), CallError>;
    fn buf_is_valid(&mut self, buffer: &Buffer) -> Result<bool, CallError>;
    fn buf_get_mark(&mut self, buffer: &Buffer, name: &str) -> Result<(u64, u64), CallError>;
    fn buf_add_highlight(&mut self,
                         buffer: &Buffer,
                         src_id: u64,
                         hl_group: &str,
                         line: u64,
                         col_start: u64,
                         col_end: u64)
                         -> Result<u64, CallError>;
    fn buf_clear_highlight(&mut self,
                           buffer: &Buffer,
                           src_id: u64,
                           line_start: u64,
                           line_end: u64)
                           -> Result<(), CallError>;
    fn tabpage_list_wins(&mut self, tabpage: &Tabpage) -> Result<Vec<Window>, CallError>;
    fn tabpage_get_var(&mut self, tabpage: &Tabpage, name: &str) -> Result<Value, CallError>;
    fn tabpage_set_var(&mut self,
                       tabpage: &Tabpage,
                       name: &str,
                       value: Value)
                       -> Result<(), CallError>;
    fn tabpage_del_var(&mut self, tabpage: &Tabpage, name: &str) -> Result<(), CallError>;
    fn tabpage_get_win(&mut self, tabpage: &Tabpage) -> Result<Window, CallError>;
    fn tabpage_get_number(&mut self, tabpage: &Tabpage) -> Result<u64, CallError>;
    fn tabpage_is_valid(&mut self, tabpage: &Tabpage) -> Result<bool, CallError>;
    fn ui_attach(&mut self,
                 width: u64,
                 height: u64,
                 options: Vec<(Value, Value)>)
                 -> Result<(), CallError>;
    fn ui_detach(&mut self) -> Result<(), CallError>;
    fn ui_try_resize(&mut self, width: u64, height: u64) -> Result<(), CallError>;
    fn ui_set_option(&mut self, name: &str, value: Value) -> Result<(), CallError>;
    fn command(&mut self, command: &str) -> Result<(), CallError>;
    fn feedkeys(&mut self, keys: &str, mode: &str, escape_csi: bool) -> Result<(), CallError>;
    fn input(&mut self, keys: &str) -> Result<u64, CallError>;
    fn replace_termcodes(&mut self,
                         str: &str,
                         from_part: bool,
                         do_lt: bool,
                         special: bool)
                         -> Result<String, CallError>;
    fn command_output(&mut self, str: &str) -> Result<String, CallError>;
    fn eval(&mut self, expr: &str) -> Result<Value, CallError>;
    fn call_function(&mut self, fname: &str, args: Vec<Value>) -> Result<Value, CallError>;
    fn strwidth(&mut self, str: &str) -> Result<u64, CallError>;
    fn list_runtime_paths(&mut self) -> Result<Vec<String>, CallError>;
    fn set_current_dir(&mut self, dir: &str) -> Result<(), CallError>;
    fn get_current_line(&mut self) -> Result<String, CallError>;
    fn set_current_line(&mut self, line: &str) -> Result<(), CallError>;
    fn del_current_line(&mut self) -> Result<(), CallError>;
    fn get_var(&mut self, name: &str) -> Result<Value, CallError>;
    fn set_var(&mut self, name: &str, value: Value) -> Result<(), CallError>;
    fn del_var(&mut self, name: &str) -> Result<(), CallError>;
    fn get_vvar(&mut self, name: &str) -> Result<Value, CallError>;
    fn get_option(&mut self, name: &str) -> Result<Value, CallError>;
    fn set_option(&mut self, name: &str, value: Value) -> Result<(), CallError>;
    fn out_write(&mut self, str: &str) -> Result<(), CallError>;
    fn err_write(&mut self, str: &str) -> Result<(), CallError>;
    fn err_writeln(&mut self, str: &str) -> Result<(), CallError>;
    fn list_bufs(&mut self) -> Result<Vec<Buffer>, CallError>;
    fn get_current_buf(&mut self) -> Result<Buffer, CallError>;
    fn set_current_buf(&mut self, buffer: &Buffer) -> Result<(), CallError>;
    fn list_wins(&mut self) -> Result<Vec<Window>, CallError>;
    fn get_current_win(&mut self) -> Result<Window, CallError>;
    fn set_current_win(&mut self, window: &Window) -> Result<(), CallError>;
    fn list_tabpages(&mut self) -> Result<Vec<Tabpage>, CallError>;
    fn get_current_tabpage(&mut self) -> Result<Tabpage, CallError>;
    fn set_current_tabpage(&mut self, tabpage: &Tabpage) -> Result<(), CallError>;
    fn subscribe(&mut self, event: &str) -> Result<(), CallError>;
    fn unsubscribe(&mut self, event: &str) -> Result<(), CallError>;
    fn get_color_by_name(&mut self, name: &str) -> Result<u64, CallError>;
    fn get_color_map(&mut self) -> Result<Vec<(Value, Value)>, CallError>;
    fn get_mode(&mut self) -> Result<Vec<(Value, Value)>, CallError>;
    fn get_api_info(&mut self) -> Result<Vec<Value>, CallError>;
    fn call_atomic(&mut self, calls: Vec<Value>) -> Result<Vec<Value>, CallError>;
    fn win_get_buf(&mut self, window: &Window) -> Result<Buffer, CallError>;
    fn win_get_cursor(&mut self, window: &Window) -> Result<(u64, u64), CallError>;
    fn win_set_cursor(&mut self, window: &Window, pos: (u64, u64)) -> Result<(), CallError>;
    fn win_get_height(&mut self, window: &Window) -> Result<u64, CallError>;
    fn win_set_height(&mut self, window: &Window, height: u64) -> Result<(), CallError>;
    fn win_get_width(&mut self, window: &Window) -> Result<u64, CallError>;
    fn win_set_width(&mut self, window: &Window, width: u64) -> Result<(), CallError>;
    fn win_get_var(&mut self, window: &Window, name: &str) -> Result<Value, CallError>;
    fn win_set_var(&mut self, window: &Window, name: &str, value: Value) -> Result<(), CallError>;
    fn win_del_var(&mut self, window: &Window, name: &str) -> Result<(), CallError>;
    fn win_get_option(&mut self, window: &Window, name: &str) -> Result<Value, CallError>;
    fn win_set_option(&mut self,
                      window: &Window,
                      name: &str,
                      value: Value)
                      -> Result<(), CallError>;
    fn win_get_position(&mut self, window: &Window) -> Result<(u64, u64), CallError>;
    fn win_get_tabpage(&mut self, window: &Window) -> Result<Tabpage, CallError>;
    fn win_get_number(&mut self, window: &Window) -> Result<u64, CallError>;
    fn win_is_valid(&mut self, window: &Window) -> Result<bool, CallError>;
}

impl NeovimApi for Neovim {
    fn buf_line_count(&mut self, buffer: &Buffer) -> Result<u64, CallError> {
        self.session
            .call("nvim_buf_line_count", &call_args![buffer])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buf_get_lines(&mut self,
                     buffer: &Buffer,
                     start: u64,
                     end: u64,
                     strict_indexing: bool)
                     -> Result<Vec<String>, CallError> {
        self.session
            .call("nvim_buf_get_lines",
                  &call_args![buffer, start, end, strict_indexing])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buf_set_lines(&mut self,
                     buffer: &Buffer,
                     start: u64,
                     end: u64,
                     strict_indexing: bool,
                     replacement: Vec<String>)
                     -> Result<(), CallError> {
        self.session
            .call("nvim_buf_set_lines",
                  &call_args![buffer, start, end, strict_indexing, replacement])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buf_get_var(&mut self, buffer: &Buffer, name: &str) -> Result<Value, CallError> {
        self.session
            .call("nvim_buf_get_var", &call_args![buffer, name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buf_get_changedtick(&mut self, buffer: &Buffer) -> Result<u64, CallError> {
        self.session
            .call("nvim_buf_get_changedtick", &call_args![buffer])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buf_set_var(&mut self, buffer: &Buffer, name: &str, value: Value) -> Result<(), CallError> {
        self.session
            .call("nvim_buf_set_var", &call_args![buffer, name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buf_del_var(&mut self, buffer: &Buffer, name: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_buf_del_var", &call_args![buffer, name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buf_get_option(&mut self, buffer: &Buffer, name: &str) -> Result<Value, CallError> {
        self.session
            .call("nvim_buf_get_option", &call_args![buffer, name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buf_set_option(&mut self,
                      buffer: &Buffer,
                      name: &str,
                      value: Value)
                      -> Result<(), CallError> {
        self.session
            .call("nvim_buf_set_option", &call_args![buffer, name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buf_get_number(&mut self, buffer: &Buffer) -> Result<u64, CallError> {
        self.session
            .call("nvim_buf_get_number", &call_args![buffer])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buf_get_name(&mut self, buffer: &Buffer) -> Result<String, CallError> {
        self.session
            .call("nvim_buf_get_name", &call_args![buffer])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buf_set_name(&mut self, buffer: &Buffer, name: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_buf_set_name", &call_args![buffer, name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buf_is_valid(&mut self, buffer: &Buffer) -> Result<bool, CallError> {
        self.session
            .call("nvim_buf_is_valid", &call_args![buffer])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buf_get_mark(&mut self, buffer: &Buffer, name: &str) -> Result<(u64, u64), CallError> {
        self.session
            .call("nvim_buf_get_mark", &call_args![buffer, name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buf_add_highlight(&mut self,
                         buffer: &Buffer,
                         src_id: u64,
                         hl_group: &str,
                         line: u64,
                         col_start: u64,
                         col_end: u64)
                         -> Result<u64, CallError> {
        self.session
            .call("nvim_buf_add_highlight",
                  &call_args![buffer, src_id, hl_group, line, col_start, col_end])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn buf_clear_highlight(&mut self,
                           buffer: &Buffer,
                           src_id: u64,
                           line_start: u64,
                           line_end: u64)
                           -> Result<(), CallError> {
        self.session
            .call("nvim_buf_clear_highlight",
                  &call_args![buffer, src_id, line_start, line_end])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn tabpage_list_wins(&mut self, tabpage: &Tabpage) -> Result<Vec<Window>, CallError> {
        self.session
            .call("nvim_tabpage_list_wins", &call_args![tabpage])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn tabpage_get_var(&mut self, tabpage: &Tabpage, name: &str) -> Result<Value, CallError> {
        self.session
            .call("nvim_tabpage_get_var", &call_args![tabpage, name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn tabpage_set_var(&mut self,
                       tabpage: &Tabpage,
                       name: &str,
                       value: Value)
                       -> Result<(), CallError> {
        self.session
            .call("nvim_tabpage_set_var", &call_args![tabpage, name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn tabpage_del_var(&mut self, tabpage: &Tabpage, name: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_tabpage_del_var", &call_args![tabpage, name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn tabpage_get_win(&mut self, tabpage: &Tabpage) -> Result<Window, CallError> {
        self.session
            .call("nvim_tabpage_get_win", &call_args![tabpage])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn tabpage_get_number(&mut self, tabpage: &Tabpage) -> Result<u64, CallError> {
        self.session
            .call("nvim_tabpage_get_number", &call_args![tabpage])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn tabpage_is_valid(&mut self, tabpage: &Tabpage) -> Result<bool, CallError> {
        self.session
            .call("nvim_tabpage_is_valid", &call_args![tabpage])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn ui_attach(&mut self,
                 width: u64,
                 height: u64,
                 options: Vec<(Value, Value)>)
                 -> Result<(), CallError> {
        self.session
            .call("nvim_ui_attach", &call_args![width, height, options])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn ui_detach(&mut self) -> Result<(), CallError> {
        self.session
            .call("nvim_ui_detach", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn ui_try_resize(&mut self, width: u64, height: u64) -> Result<(), CallError> {
        self.session
            .call("nvim_ui_try_resize", &call_args![width, height])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn ui_set_option(&mut self, name: &str, value: Value) -> Result<(), CallError> {
        self.session
            .call("nvim_ui_set_option", &call_args![name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn command(&mut self, command: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_command", &call_args![command])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn feedkeys(&mut self, keys: &str, mode: &str, escape_csi: bool) -> Result<(), CallError> {
        self.session
            .call("nvim_feedkeys", &call_args![keys, mode, escape_csi])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn input(&mut self, keys: &str) -> Result<u64, CallError> {
        self.session
            .call("nvim_input", &call_args![keys])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn replace_termcodes(&mut self,
                         str: &str,
                         from_part: bool,
                         do_lt: bool,
                         special: bool)
                         -> Result<String, CallError> {
        self.session
            .call("nvim_replace_termcodes",
                  &call_args![str, from_part, do_lt, special])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn command_output(&mut self, str: &str) -> Result<String, CallError> {
        self.session
            .call("nvim_command_output", &call_args![str])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn eval(&mut self, expr: &str) -> Result<Value, CallError> {
        self.session
            .call("nvim_eval", &call_args![expr])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn call_function(&mut self, fname: &str, args: Vec<Value>) -> Result<Value, CallError> {
        self.session
            .call("nvim_call_function", &call_args![fname, args])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn strwidth(&mut self, str: &str) -> Result<u64, CallError> {
        self.session
            .call("nvim_strwidth", &call_args![str])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn list_runtime_paths(&mut self) -> Result<Vec<String>, CallError> {
        self.session
            .call("nvim_list_runtime_paths", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_current_dir(&mut self, dir: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_set_current_dir", &call_args![dir])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_current_line(&mut self) -> Result<String, CallError> {
        self.session
            .call("nvim_get_current_line", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_current_line(&mut self, line: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_set_current_line", &call_args![line])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn del_current_line(&mut self) -> Result<(), CallError> {
        self.session
            .call("nvim_del_current_line", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_var(&mut self, name: &str) -> Result<Value, CallError> {
        self.session
            .call("nvim_get_var", &call_args![name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_var(&mut self, name: &str, value: Value) -> Result<(), CallError> {
        self.session
            .call("nvim_set_var", &call_args![name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn del_var(&mut self, name: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_del_var", &call_args![name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_vvar(&mut self, name: &str) -> Result<Value, CallError> {
        self.session
            .call("nvim_get_vvar", &call_args![name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_option(&mut self, name: &str) -> Result<Value, CallError> {
        self.session
            .call("nvim_get_option", &call_args![name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_option(&mut self, name: &str, value: Value) -> Result<(), CallError> {
        self.session
            .call("nvim_set_option", &call_args![name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn out_write(&mut self, str: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_out_write", &call_args![str])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn err_write(&mut self, str: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_err_write", &call_args![str])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn err_writeln(&mut self, str: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_err_writeln", &call_args![str])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn list_bufs(&mut self) -> Result<Vec<Buffer>, CallError> {
        self.session
            .call("nvim_list_bufs", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_current_buf(&mut self) -> Result<Buffer, CallError> {
        self.session
            .call("nvim_get_current_buf", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_current_buf(&mut self, buffer: &Buffer) -> Result<(), CallError> {
        self.session
            .call("nvim_set_current_buf", &call_args![buffer])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn list_wins(&mut self) -> Result<Vec<Window>, CallError> {
        self.session
            .call("nvim_list_wins", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_current_win(&mut self) -> Result<Window, CallError> {
        self.session
            .call("nvim_get_current_win", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_current_win(&mut self, window: &Window) -> Result<(), CallError> {
        self.session
            .call("nvim_set_current_win", &call_args![window])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn list_tabpages(&mut self) -> Result<Vec<Tabpage>, CallError> {
        self.session
            .call("nvim_list_tabpages", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_current_tabpage(&mut self) -> Result<Tabpage, CallError> {
        self.session
            .call("nvim_get_current_tabpage", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_current_tabpage(&mut self, tabpage: &Tabpage) -> Result<(), CallError> {
        self.session
            .call("nvim_set_current_tabpage", &call_args![tabpage])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn subscribe(&mut self, event: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_subscribe", &call_args![event])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn unsubscribe(&mut self, event: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_unsubscribe", &call_args![event])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_color_by_name(&mut self, name: &str) -> Result<u64, CallError> {
        self.session
            .call("nvim_get_color_by_name", &call_args![name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_color_map(&mut self) -> Result<Vec<(Value, Value)>, CallError> {
        self.session
            .call("nvim_get_color_map", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_mode(&mut self) -> Result<Vec<(Value, Value)>, CallError> {
        self.session
            .call("nvim_get_mode", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_api_info(&mut self) -> Result<Vec<Value>, CallError> {
        self.session
            .call("nvim_get_api_info", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn call_atomic(&mut self, calls: Vec<Value>) -> Result<Vec<Value>, CallError> {
        self.session
            .call("nvim_call_atomic", &call_args![calls])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn win_get_buf(&mut self, window: &Window) -> Result<Buffer, CallError> {
        self.session
            .call("nvim_win_get_buf", &call_args![window])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn win_get_cursor(&mut self, window: &Window) -> Result<(u64, u64), CallError> {
        self.session
            .call("nvim_win_get_cursor", &call_args![window])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn win_set_cursor(&mut self, window: &Window, pos: (u64, u64)) -> Result<(), CallError> {
        self.session
            .call("nvim_win_set_cursor", &call_args![window, pos])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn win_get_height(&mut self, window: &Window) -> Result<u64, CallError> {
        self.session
            .call("nvim_win_get_height", &call_args![window])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn win_set_height(&mut self, window: &Window, height: u64) -> Result<(), CallError> {
        self.session
            .call("nvim_win_set_height", &call_args![window, height])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn win_get_width(&mut self, window: &Window) -> Result<u64, CallError> {
        self.session
            .call("nvim_win_get_width", &call_args![window])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn win_set_width(&mut self, window: &Window, width: u64) -> Result<(), CallError> {
        self.session
            .call("nvim_win_set_width", &call_args![window, width])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn win_get_var(&mut self, window: &Window, name: &str) -> Result<Value, CallError> {
        self.session
            .call("nvim_win_get_var", &call_args![window, name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn win_set_var(&mut self, window: &Window, name: &str, value: Value) -> Result<(), CallError> {
        self.session
            .call("nvim_win_set_var", &call_args![window, name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn win_del_var(&mut self, window: &Window, name: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_win_del_var", &call_args![window, name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn win_get_option(&mut self, window: &Window, name: &str) -> Result<Value, CallError> {
        self.session
            .call("nvim_win_get_option", &call_args![window, name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn win_set_option(&mut self,
                      window: &Window,
                      name: &str,
                      value: Value)
                      -> Result<(), CallError> {
        self.session
            .call("nvim_win_set_option", &call_args![window, name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn win_get_position(&mut self, window: &Window) -> Result<(u64, u64), CallError> {
        self.session
            .call("nvim_win_get_position", &call_args![window])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn win_get_tabpage(&mut self, window: &Window) -> Result<Tabpage, CallError> {
        self.session
            .call("nvim_win_get_tabpage", &call_args![window])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn win_get_number(&mut self, window: &Window) -> Result<u64, CallError> {
        self.session
            .call("nvim_win_get_number", &call_args![window])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn win_is_valid(&mut self, window: &Window) -> Result<bool, CallError> {
        self.session
            .call("nvim_win_is_valid", &call_args![window])
            .map(map_result)
            .map_err(map_generic_error)
    }
}
