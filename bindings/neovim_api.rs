// Auto generated {{date}}

use neovim::*;
use rmp::Value;
use rpc::*;
use session::Session;

{% for typename in exttypes %}
pub struct {{ typename }} <'a> {
    code_data: Value,
    code: u64,
    session: &'a Session,
}

impl <'a> {{ typename }} <'a> {
    pub fn new(session: &'a Session, code_data: Value) -> {{ typename }} {
        {{ typename }} {
            code_data: code_data,
            code: {{exttypes[typename]}},
            session: session,
        }
    }

    {% for f in functions %}
    {% if f.ext and f.name.startswith(typename.lower()) %}
    pub fn {{f.name}}(&mut self, {{f.argstring}}) -> Result<{{f.return_type.native_type_ret}}, String> {
        self.session.call("{{f.name}}",
                          &call_args![self.code_data.clone()
                          {% if f.parameters|count > 0 %}
                          , {{ f.parameters|map(attribute = "name")|join(", ") }}
                          {% endif %}
                          ])
                    .map(map_result)
                    .map_err(map_generic_error)
    }
    {% endif %}
    {% endfor %}
}

{% endfor %}

impl FromVal<Value> for Window {
    fn from_val(val: Value) -> Self {
        Window {
            code_data: val,
        }
    }
}

impl FromVal<Value> for Tabpage {
    fn from_val(val: Value) -> Self {
        Tabpage {
            code_data: val,
        }
    }
}

impl FromVal<Value> for Buffer {
    fn from_val(val: Value) -> Self {
        Buffer {
            code_data: val,
        }
    }
}

pub trait NeovimApi {
    {% for f in functions %}
    {% if not f.ext %}
    fn {{f.name}}(&mut self, {{f.argstring}}) -> Result<{{f.return_type.native_type_ret}}, String>;
    {% endif %}
    {% endfor %}
}

impl NeovimApi for Neovim {
    {% for f in functions %}
    {% if not f.ext %}
    fn {{f.name}}(&mut self, {{f.argstring}}) -> Result<{{f.return_type.native_type_ret}}, String> {
        self.session.call("{{f.name}}",
                          &call_args![{{ f.parameters|map(attribute = "name")|join(", ") }}])
                    .map(map_result)
                    .map_err(map_generic_error)
    }

    {% endif %}
    {% endfor %}
}
