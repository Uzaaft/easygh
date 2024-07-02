#[derive(Debug)]
pub enum EasyGHError {
    FzfNotFound,
    GHNotFound,
    FzfAndGHNotFound,
    FzfReadError,
    GHRepoListError,
    GHRepoDeleteError,
    FzfSpawnError,
}

impl std::fmt::Display for EasyGHError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EasyGHError::FzfNotFound => write!(f, "could not find fzf, is it installed?"),
            EasyGHError::GHNotFound => write!(f, "could not find gh, is it installed?"),
            EasyGHError::FzfAndGHNotFound => {
                write!(f, "could not find gh and fzf, are they installed?")
            }
            EasyGHError::FzfReadError => write!(f, "could not read from fzf"),
            EasyGHError::GHRepoListError => write!(f, "could not list repos"),
            EasyGHError::GHRepoDeleteError => write!(
                f,
                "could not delete repo. Make sure you have the correct permissions."
            ),
            EasyGHError::FzfSpawnError => write!(f, "could not spawn fzf"),
        }
    }
}
