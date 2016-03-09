use session::Session;

pub struct Neovim {
    session: Session,
}

impl Neovim {
    pub fn new(session: Session) -> Neovim {
        Neovim { session: session };
    }
}
