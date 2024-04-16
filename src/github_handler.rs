use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Repository {
id: i64,
node_id: String,
name: String,
full_name: String,
private: bool,
html_url: String,
description: Option<String>,
fork: bool,
url: String,
archive_url: String,
assignees_url: String,
blobs_url: String,
branches_url: String,
collaborators_url: String,
comments_url: String,
commits_url: String,
compare_url: String,
contents_url: String,
contributors_url: String,
deployments_url: String,
downloads_url: String,
events_url: String,
forks_url: String,
git_commits_url: String,
git_refs_url: String,
git_tags_url: String,
hooks_url: String,
issue_comment_url: String,
issue_events_url: String,
issues_url: String,
keys_url: String,
labels_url: String,
languages_url: String,
merges_url: String,
milestones_url: String,
notifications_url: String,
pulls_url: String,
releases_url: String,
stargazers_url: String,
statuses_url: String,
subscribers_url: String,
subscription_url: String,
tags_url: String,
teams_url: String,
trees_url: String,
clone_url: Option<String>,
mirror_url: Option<String>,
svn_url: String,
homepage: Option<String>,
language: Option<String>,
forks_count: i64,
stargazers_count: i64,
watchers_count: i64,
size: i64,
default_branch: String,
open_issues_count: i64,
is_template: bool,
topics: Vec<String>,
has_issues: bool,
has_projects: bool,
has_wiki: bool,
has_pages: bool,
has_downloads: bool,
has_discussions: bool,
archived: bool,
disabled: bool,
visibility: String,
pushed_at: Option<String>,
created_at: Option<String>,
updated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PullRequest {
url: String,
id: i64,
node_id: String,
html_url: String,
diff_url: String,
patch_url: String,
issue_url: String,
commits_url: String,
review_comments_url: String,
review_comment_url: String,
comments_url: String,
statuses_url: String,
number: i32,
state: String,
locked: bool,
title: String,
body: Option<String>,
}
pub struct GithubInstance {
token: String,
}

#[derive(Debug, Serialize)]
pub struct NewGHIssue {
pub title: String,
pub body: String
}

#[derive(Debug, Deserialize)]
pub struct GTIssueRes {
id: i64,
node_id: String,
url: String,
repository_url: String,
labels_url: String,
comments_url: String,
events_url: String,
html_url: String,
number: i32,
state: String,
title: String,
body: String,
}

impl GithubInstance {
    pub fn new(git_token: String) -> GithubInstance {
        GithubInstance {
            token: git_token,
        }
    }

    pub async fn fetch_all_owner_repos(&self) -> Result<Vec<Repository>, reqwest::Error> {
        let client = Client::builder().user_agent("FirstRust").build()?;
        let response = client.get("https://api.github.com/user/repos?type=owner".to_owned()).bearer_auth(&self.token).send().await?;
        let repos: Vec<Repository> = response.json().await?;
        Ok(repos)
    }

    pub async fn fetch_specific_repo(&self, repo:String, owner:String) -> Result<Repository, reqwest::Error> {
        let client = Client::builder().user_agent("FirstRust").build()?;
        // let owner = "EleisonC".to_string();
        // let repo = "NestApplication".to_string();
        let url = format!("https://api.github.com/repos/{owner}/{repo}");
        let response = client.get(url).bearer_auth(&self.token).send().await?;
        let repo: Repository = response.json().await?;
        Ok(repo)
    }

    pub async fn fetch_prs_for_a_repo(&self, owner: &String, repo: &String) -> Result<Vec<PullRequest>, reqwest::Error>{
        let client = Client::builder().user_agent("FallOut").build()?;
        let url = format!("https://api.github.com/repos/{owner}/{repo}/pulls?state=all");
        let response = client.get(url).bearer_auth(&self.token).send().await?;
        let prs_results: Vec<PullRequest> = response.json().await?;
        Ok(prs_results)
    }

    pub async fn ct_a_github_issue_sp_repo(&self, owner: &String, repo: &String, issue: NewGHIssue) -> Result<GTIssueRes, reqwest::Error> {
        let client = Client::builder().user_agent("MaxPayne").build()?;
        let url = format!("https://api.github.com/repos/{owner}/{repo}/issues");
        
        let response = client.post(url).bearer_auth(&self.token).json(&issue).send().await?;
        let issue_results: GTIssueRes = response.json().await?;
        println!("Here is some code: {:#?}", issue_results);
        Ok(issue_results)
    }

    pub async fn fetch_all_repo_issues(&self, owner: &String, repo: &String) -> Result<Vec<GTIssueRes>, reqwest::Error>{
        let client = Client::builder().user_agent("MaxPayne").build()?;
        let url = format!("https://api.github.com/repos/{owner}/{repo}/issues");

        let response = client.get(url).bearer_auth(&self.token).send().await?;
        let repo_issues: Vec<GTIssueRes> = response.json().await?;
        Ok(repo_issues)
    } 
}
