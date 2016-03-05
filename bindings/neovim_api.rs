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
    pub fn {{f.name}}(&mut self, {{f.argstring}}) -> Result<Value, Value> {
        self.session.call("{{f.name}}",
                          &vec![{{ f.parameters|map(attribute = "arg_converter")|join(", ") }}])
    }

    {% endfor %}
}
