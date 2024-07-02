use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct RepoWithNameOwner {
    #[serde(rename = "nameWithOwner")]
    pub name_with_owner: String,
}

impl std::fmt::Display for RepoWithNameOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name_with_owner)
    }
}
