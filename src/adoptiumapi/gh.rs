use serde::{Deserialize, Serialize};

pub type AdoptiumRepos = Vec<AdoptiumRepo>;

async fn get_repos() -> Result<AdoptiumRepos, reqwest::Error> {
    const URL: &str = "https://api.github.com/orgs/adoptium/repos?per_page=100";

    crate::CLIENT.get(URL).send().await?.json().await
}

pub async fn get_binary_repos() -> Result<AdoptiumRepos, reqwest::Error> {
    let repos = get_repos().await?;

    let binary_repos = repos
        .into_iter()
        .filter(|repo| repo.name.ends_with("-binaries"))
        .collect();

    Ok(binary_repos)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdoptiumRepo {
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub private: bool,
    pub owner: Owner,
    pub html_url: String,
    pub description: Option<String>,
    pub fork: bool,
    pub url: String,
    pub forks_url: String,
    pub keys_url: String,
    pub collaborators_url: String,
    pub teams_url: String,
    pub hooks_url: String,
    pub issue_events_url: String,
    pub events_url: String,
    pub assignees_url: String,
    pub branches_url: String,
    pub tags_url: String,
    pub blobs_url: String,
    pub git_tags_url: String,
    pub git_refs_url: String,
    pub trees_url: String,
    pub statuses_url: String,
    pub languages_url: String,
    pub stargazers_url: String,
    pub contributors_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub commits_url: String,
    pub git_commits_url: String,
    pub comments_url: String,
    pub issue_comment_url: String,
    pub contents_url: String,
    pub compare_url: String,
    pub merges_url: String,
    pub archive_url: String,
    pub downloads_url: String,
    pub issues_url: String,
    pub pulls_url: String,
    pub milestones_url: String,
    pub notifications_url: String,
    pub labels_url: String,
    pub releases_url: String,
    pub deployments_url: String,
    pub created_at: String,
    pub updated_at: String,
    pub pushed_at: String,
    pub git_url: String,
    pub ssh_url: String,
    pub clone_url: String,
    pub svn_url: String,
    pub homepage: Option<String>,
    pub size: i64,
    pub stargazers_count: i64,
    pub watchers_count: i64,
    pub language: Option<String>,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_downloads: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub forks_count: i64,
    pub mirror_url: Option<serde_json::Value>,
    pub archived: bool,
    pub disabled: bool,
    pub open_issues_count: i64,
    pub license: Option<License>,
    pub allow_forking: bool,
    pub is_template: bool,
    pub web_commit_signoff_required: bool,
    pub topics: Vec<String>,
    pub visibility: Visibility,
    pub forks: i64,
    pub open_issues: i64,
    pub watchers: i64,
    pub default_branch: DefaultBranch,
    pub permissions: Permissions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct License {
    pub key: Key,
    pub name: Name,
    pub spdx_id: SpdxId,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    pub login: Login,
    pub id: i64,
    pub node_id: OwnerNodeId,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: FollowingUrl,
    pub gists_url: GistsUrl,
    pub starred_url: StarredUrl,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: EventsUrl,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub owner_type: Type,
    pub site_admin: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Permissions {
    pub admin: bool,
    pub maintain: bool,
    pub push: bool,
    pub triage: bool,
    pub pull: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DefaultBranch {
    #[serde(rename = "main")]
    Main,
    #[serde(rename = "master")]
    Master,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Key {
    #[serde(rename = "apache-2.0")]
    Apache20,
    #[serde(rename = "gpl-2.0")]
    Gpl20,
    #[serde(rename = "mit")]
    Mit,
    #[serde(rename = "other")]
    Other,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Name {
    #[serde(rename = "Apache License 2.0")]
    ApacheLicense20,
    #[serde(rename = "GNU General Public License v2.0")]
    GnuGeneralPublicLicenseV20,
    #[serde(rename = "MIT License")]
    MitLicense,
    Other,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SpdxId {
    #[serde(rename = "Apache-2.0")]
    Apache20,
    #[serde(rename = "GPL-2.0")]
    Gpl20,
    #[serde(rename = "MIT")]
    Mit,
    #[serde(rename = "NOASSERTION")]
    Noassertion,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EventsUrl {
    #[serde(rename = "https://api.github.com/users/adoptium/events{/privacy}")]
    HttpsApiGithubComUsersAdoptiumEventsPrivacy,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FollowingUrl {
    #[serde(rename = "https://api.github.com/users/adoptium/following{/other_user}")]
    HttpsApiGithubComUsersAdoptiumFollowingOtherUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GistsUrl {
    #[serde(rename = "https://api.github.com/users/adoptium/gists{/gist_id}")]
    HttpsApiGithubComUsersAdoptiumGistsGistId,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Login {
    #[serde(rename = "adoptium")]
    Adoptium,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OwnerNodeId {
    #[serde(rename = "MDEyOk9yZ2FuaXphdGlvbjY3MTY1Mzcy")]
    MdEyOk9YZ2FuaXphdGlvbjY3Mty1Mzcy,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    Organization,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StarredUrl {
    #[serde(rename = "https://api.github.com/users/adoptium/starred{/owner}{/repo}")]
    HttpsApiGithubComUsersAdoptiumStarredOwnerRepo,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Visibility {
    #[serde(rename = "public")]
    Public,
}
