
use serde::Deserialize;
#[derive(Debug, Deserialize)]
struct Repository {
    name: String,
    full_name: String,
    html_url: String,
    description: Option<String>,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    println!("Hello, world!");
    let github_user = "EleisonC";
    let url = format!("https://api.github.com/users/{}/repos", github_user);

    let client = reqwest::Client::builder().user_agent("FirstRust").build()?;
    let body: Vec<Repository> = client.get(url).send().await?.json().await?;
    println!("here is the response: {body:#?}");
    Ok(())
}
