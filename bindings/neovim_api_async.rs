// Auto generated {{date}}

use crate::neovim::*;
use crate::neovim_api::*;
use crate::rpc::*;
use crate::r#async::AsyncCall;

pub trait NeovimApiAsync {
    {% for f in functions if not f.ext %}
    /// since: {{f.since}}
    fn {{f.name|replace('nvim_', '')}}_async(&mut self, {{f.argstring}}) -> AsyncCall<'_, {{f.return_type.native_type_ret}}>;
    {% endfor %}
}

impl NeovimApiAsync for Neovim {
    {% for f in functions if not f.ext %}
    fn {{f.name|replace('nvim_', '')}}_async(&mut self, {{f.argstring}}) -> AsyncCall<'_, {{f.return_type.native_type_ret}}> {
        self.session.call_async::<{{f.return_type.native_type_ret}}>("{{f.name}}",
                          call_args![{{ f.parameters|map(attribute = "name")|join(", ") }}])
    }

    {% endfor %}
}
