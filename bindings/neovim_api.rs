// Auto generated {{date}}

use std::collections::HashMap;

pub struct Neovim {
    session: Session,
    ext_types: HashMap<u64, Fn>,
}

impl Neovim {
    pub fn new(session: Session) -> Neovim {
        let ext_types = HashMap::new();
        {% for typename in exttypes %}
        ext_types.insert({{exttypes[typename]}}, unpack{{typename}});
        {% endfor %}
        Neovim {
            session: session,
            ext_types: ext_types,
        }
    }

    {% for f in functions %}
    pub fn {{f.name}}(&mut self, {{f.argstring}}) {
        self.session.call("{{f.name}}",
                          {{ f.parameters|map(attribute = "name")|join(", ") }});
    }

    {% endfor %}
}
