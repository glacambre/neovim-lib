// Auto generated {{date}}

use neovim::*;
use rmp::Value;
use rpc::*;

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

pub trait NeovimApi {
    {% for f in functions %}
    fn {{f.name}}(&mut self, {{f.argstring}}) -> Result<{{f.return_type.native_type_ret}}, String>;
    {% endfor %}
}

impl NeovimApi for Neovim {
    {% for f in functions %}
    fn {{f.name}}(&mut self, {{f.argstring}}) -> Result<{{f.return_type.native_type_ret}}, String> {
        self.session.call("{{f.name}}",
                          &call_args![{{ f.parameters|map(attribute = "name")|join(", ") }}])
                    .map(map_result)
                    .map_err(map_generic_error)
    }

    {% endfor %}
}
