use oranda::data::github::GithubRepo;

#[test]
fn it_parses_an_https_repo_string() {
    let input = "https://github.com/axodotdev/oranda";
    let actual_owner = "axodotdev";
    let actual_name = "oranda";
    let parsed = GithubRepo::from_url(input).unwrap();
    assert_eq!(parsed.owner, actual_owner);
    assert_eq!(parsed.name, actual_name);
}

#[test]
fn it_parses_an_https_repo_string_with_dot_git() {
    let input = "https://github.com/axodotdev/oranda.git";
    let actual_owner = "axodotdev";
    let actual_name = "oranda";
    let parsed = GithubRepo::from_url(input).unwrap();
    assert_eq!(parsed.owner, actual_owner);
    assert_eq!(parsed.name, actual_name);
}

#[test]
fn it_parses_an_ssh_repo_string() {
    let input = "git@github.com:axodotdev/oranda.git";
    let actual_owner = "axodotdev";
    let actual_name = "oranda";
    let parsed = GithubRepo::from_url(input).unwrap();
    assert_eq!(parsed.owner, actual_owner);
    assert_eq!(parsed.name, actual_name);
}
