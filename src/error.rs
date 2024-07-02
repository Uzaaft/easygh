#[derive(Debug)]
pub enum EasyGHError {
    FzfNotFound,
}

impl std::fmt::Display for EasyGHError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EasyGHError::FzfNotFound => write!(f, "could not find fzf, is it installed?"),
        }
    }
}
