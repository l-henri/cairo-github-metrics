#![allow(dead_code)]

use serde::Deserialize;
pub struct CairoRepos {
    pub num: u64,
    pub repos: Vec<String>,
}

#[derive(Debug)]
pub struct Starker {
    pub login: String,
    pub contributions: u64,
}

#[derive(Deserialize, Debug)]
pub struct Contributor {
    login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    r#type: String,
    site_admin: bool,
    contributions: i64,
}

// pub type Contributors = Vec<Contributor>;

#[derive(Deserialize, Debug)]
pub struct User {
    pub login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    r#type: String,
    site_admin: bool,
    name: String,
    company: Option<serde_json::Value>,
    blog: String,
    location: Option<serde_json::Value>,
    email: Option<serde_json::Value>,
    hireable: Option<serde_json::Value>,
    bio: String,
    twitter_username: String,
    public_repos: i64,
    public_gists: i64,
    followers: i64,
    following: i64,
    created_at: String,
    updated_at: String,
}

// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

#[derive(Deserialize, Debug)]
pub struct Repos {
    pub total_count: i64,
    incomplete_results: bool,
    pub items: Vec<Item>,
}

#[derive(Deserialize, Debug)]
pub struct Item {
    id: i64,
    node_id: String,
    name: String,
    pub full_name: String,
    private: bool,
    owner: Owner,
    html_url: String,
    description: Option<String>,
    fork: bool,
    url: String,
    forks_url: String,
    keys_url: String,
    collaborators_url: String,
    teams_url: String,
    hooks_url: String,
    issue_events_url: String,
    events_url: String,
    assignees_url: String,
    branches_url: String,
    tags_url: String,
    blobs_url: String,
    git_tags_url: String,
    git_refs_url: String,
    trees_url: String,
    statuses_url: String,
    languages_url: String,
    stargazers_url: String,
    contributors_url: String,
    subscribers_url: String,
    subscription_url: String,
    commits_url: String,
    git_commits_url: String,
    comments_url: String,
    issue_comment_url: String,
    contents_url: String,
    compare_url: String,
    merges_url: String,
    archive_url: String,
    downloads_url: String,
    issues_url: String,
    pulls_url: String,
    milestones_url: String,
    notifications_url: String,
    labels_url: String,
    releases_url: String,
    deployments_url: String,
    created_at: String,
    updated_at: String,
    pushed_at: String,
    git_url: String,
    ssh_url: String,
    clone_url: String,
    svn_url: String,
    homepage: Option<String>,
    size: i64,
    stargazers_count: i64,
    watchers_count: i64,
    language: Option<String>,
    has_issues: bool,
    has_projects: bool,
    has_downloads: bool,
    has_wiki: bool,
    has_pages: bool,
    has_discussions: bool,
    forks_count: i64,
    mirror_url: Option<serde_json::Value>,
    archived: bool,
    disabled: bool,
    open_issues_count: i64,
    license: Option<License>,
    allow_forking: bool,
    is_template: bool,
    web_commit_signoff_required: bool,
    topics: Vec<String>,
    visibility: Option<String>,
    forks: i64,
    open_issues: i64,
    watchers: i64,
    default_branch: Option<String>,
    score: f64,
}

#[derive(Deserialize, Debug)]
pub struct License {
    key: String,
    name: String,
    spdx_id: String,
    url: Option<String>,
    node_id: String,
}

#[derive(Deserialize, Debug)]
pub struct Owner {
    login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    r#type: Option<String>,
    // owner_type: Option<String>,
    site_admin: bool,
}
