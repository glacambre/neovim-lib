// Auto generated {{date}}

use neovim::Neovim;
use rmp::Value;
use rmp::value::Integer;

pub enum ExtType {
    {% for typename in exttypes %}
    {{typename}},
    {% endfor %}
}

impl ExtType {
    pub fn from_typ(typ: u64) -> Result<ExtType, String> {
        match typ {
        {% for typename in exttypes %}
        {{exttypes[typename]}} => Ok(ExtType::{{typename}}),
        {% endfor %}
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
    {% for f in functions %}
    fn {{f.name}}(&mut self, {{f.argstring}}) -> Result<Value, String>;
    {% endfor %}
}

impl NeovimApi for Neovim {
    {% for f in functions %}
    fn {{f.name}}(&mut self, {{f.argstring}}) -> Result<Value, String> {
        self.session.call("{{f.name}}",
                          &vec![{{ f.parameters|map(attribute = "arg_converter")|join(", ") }}])
                    .map_err(map_generic_error)
    }

    {% endfor %}
}
