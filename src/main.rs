extern crate dotenv_codegen;
use dotenv_codegen::dotenv;

mod github_handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    println!("Hello, world!");
    let github_fetcher = github_handler::GithubInstance::new(
        dotenv!("GITHUB_TOKEN").to_owned()
    );
    let repos: Vec<github_handler::Repository> = github_fetcher.fetch_all_owner_repos().await?;

    let owner = "EleisonC".to_string();
    let repo = "fetchRust".to_string();
    let one_repo: github_handler::Repository = github_fetcher.fetch_specific_repo(repo.clone(), owner.clone()).await?;

    let prs_results: Vec<github_handler::PullRequest> = github_fetcher.fetch_prs_for_a_repo(&owner, &repo).await?;
    let new_issue = github_handler::NewGHIssue {
        title: "Second Issue I created".to_string(),
        body: "This issue was create via a rust program".to_string()
    };

    let created_issue: github_handler::GTIssueRes = github_fetcher.ct_a_github_issue_sp_repo(&owner, &repo, new_issue).await?;

    let my_issues: Vec<github_handler::GTIssueRes> = github_fetcher.fetch_all_repo_issues(&owner, &repo).await?;

    println!("We have over: {:#?}", repos.len());
    println!("here is the response: {one_repo:#?}");
    println!("Here are PRS: {prs_results:#?}");
    println!("Well we have created our first issue: {:#?}", created_issue);
    println!("This is all issues on the {repo}: {my_issues:#?}");
    Ok(())
}
