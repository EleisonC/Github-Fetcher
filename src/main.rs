use reqwest::Client;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
struct Repository {
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

struct GithubInstance {
    token: String,
}

impl GithubInstance {
    fn new(git_token: String) -> GithubInstance {
        GithubInstance {
            token: git_token,
        }
    }

    async fn fetch_all_owner_repos(&self) -> Result<Vec<Repository>, reqwest::Error> {
        let client = Client::builder().user_agent("FirstRust").build()?;
        let response = client.get("https://api.github.com/user/repos?type=owner".to_owned()).bearer_auth(&self.token).send().await?;
        let repos: Vec<Repository> = response.json().await?;
        Ok(repos)
    }

    async fn fetch_specific_repo(&self, repo:String, owner:String) -> Result<Repository, reqwest::Error> {
        let client = Client::builder().user_agent("FirstRust").build()?;
        // let owner = "EleisonC".to_string();
        // let repo = "NestApplication".to_string();
        let url = format!("https://api.github.com/repos/{owner}/{repo}");
        let response = client.get(url).bearer_auth(&self.token).send().await?;
        let repo: Repository = response.json().await?;
        Ok(repo)
    }
}

extern crate dotenv_codegen;
use dotenv_codegen::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    println!("Hello, world!");
    let github_fetcher = GithubInstance::new(
        dotenv!("GITHUB_TOKEN").to_owned()
    );
    let repos: Vec<Repository> = github_fetcher.fetch_all_owner_repos().await?;

    let owner = "EleisonC".to_string();
    let repo = "NestApplication".to_string();    
    let one_repo: Repository = github_fetcher.fetch_specific_repo(repo, owner).await?;

    println!("We have over: {:#?}", repos.len());
    println!("here is the response: {one_repo:#?}");
    Ok(())
}
