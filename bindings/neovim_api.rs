// Auto generated {{date}}

use session::Session;
use rmp::Value;
use rmp::value::Integer;

pub struct Neovim {
    session: Session,
    {% for typename in exttypes %}
    {{typename|lower}}_unpack_id: u64,
    {% endfor %}
}

fn convert_array_of_string(vec: &Vec<String>) -> Value {
    Value::Array(vec.iter().map(|s| Value::String(s.to_owned())).collect())
}

fn map_generic_error(err: Value) -> String {
    match err {
        Value::String(val) => val.to_owned(),
        val => format!("Unknow error type: {:?}", val),
    }
}

impl Neovim {
    pub fn new(session: Session) -> Neovim {
        Neovim {
            session: session,
            {% for typename in exttypes %}
            {{typename|lower}}_unpack_id: {{exttypes[typename]}},
            {% endfor %}
        }
    }

    {% for f in functions %}
    pub fn {{f.name}}(&mut self, {{f.argstring}}) -> Result<Value, String> {
        self.session.call("{{f.name}}",
                          &vec![{{ f.parameters|map(attribute = "arg_converter")|join(", ") }}])
                    .map_err(map_generic_error)
    }

    {% endfor %}
}
