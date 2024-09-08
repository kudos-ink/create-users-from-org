use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub github_organization: String,
    pub github_token: String,
    pub issues_api: String,
    pub issues_api_token: String,
}
