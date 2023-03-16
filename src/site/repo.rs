use crate::errors::*;
use reqwest::header::USER_AGENT;
use url::Url;

pub struct Repo {
    pub owner: String,
    pub repo: String,
}

pub fn parse(repo: &str) -> Result<Repo> {
    let repo_parsed = match Url::parse(repo) {
        Ok(parsed) => Ok(parsed),
        Err(parse_error) => Err(OrandaError::RepoParseError {
            repo: repo.to_string(),
            details: parse_error.to_string(),
        }),
    };
    let parsed = repo_parsed?;
    let parsed_url = parsed.path_segments().map(|c| c.collect::<Vec<_>>());
    if let Some(url_parts) = parsed_url {
        Ok(Repo {
            owner: url_parts[0].to_string(),
            repo: url_parts[1].to_string(),
        })
    } else {
        Err(OrandaError::RepoParseError {
            repo: repo.to_string(),
            details: "This URL is not structured the expected way, expected more segments-"
                .to_owned(),
        })
    }
}

pub fn fetch_funding_file(repo: Repo) -> Result<reqwest::blocking::Response> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/contents/.github/FUNDING.yml",
        repo.owner, repo.repo
    );
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let header = format!("oranda-{}", VERSION);

    let response = reqwest::blocking::Client::new()
        .get(url)
        .header(USER_AGENT, header)
        .send()?;

    match response.error_for_status() {
        Ok(r) => Ok(r),
        Err(e) => Err(OrandaError::GithubFundingFetchError {
            details: e.to_string(),
        }),
    }
}
