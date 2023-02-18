use super::github_structs::{Repos, CairoRepos};
use lazy_static::lazy_static;
use reqwest::header::USER_AGENT;
use reqwest::StatusCode;
use owo_colors::OwoColorize;

lazy_static! {
    static ref GITHUB_API_BASE_URL: &'static str = "https://api.github.com";
}

#[tokio::main]
pub async fn get_cairo_repos() -> Option<CairoRepos> {
    // 1. instantiate a request http client
    let client = reqwest::Client::new();

    // 2. get all the repos with Cairo
    // https://api.github.com/search/repositories?q=language:cairo&order=desc
    let get_repos_endpoint: String = format!(
        "{base_url}/search/repositories?q=language:cairo&order=desc&per_page=100&page={page}",
        base_url = *GITHUB_API_BASE_URL,
        page = 1
    );
    // Send the request to the Github API and await for the response
    let get_repos_response = client
        .get(&get_repos_endpoint)
        .header(USER_AGENT, "reqwest")
        .send()
        .await
        .unwrap();
    
    // Check the status of the response and if it is not successfull return None
    if get_repos_response.status() != StatusCode::OK {
        return None;
    } 
    

    assert_eq!(&StatusCode::OK, &get_repos_response.status());
    // Parse the response body as Json in this case
    let repos_reponse: Repos = get_repos_response.json().await.unwrap();
    // From the first API call get the number of projects for the programming lang.
    let n_repos = repos_reponse.total_count;
    println!("{}: {}", "Total Cairo repos".green().bold(), &n_repos);
    // Initialize a vector with capacity to store each of the full names of language's repo.
    // let mut repos_info_vec: Vec<String> = Vec::with_capacity(n_repos.try_into().unwrap());
    let mut repos_info_vec: Vec<String> = Vec::new();
    // let mut repos_info_vec: [String; n_repos];
    // assert_eq!(repos_info_vec.capacity() as i64, n_repos);
    // Each API call returns a page with 100 repos. Get the total number of pages.
    let total_pages = &n_repos / 100;
    // To store the repos we will use a loop that will call the Github API multiple times
    // until the status of the response is not successfull; this means there are no more pages
    // with repos
    for page in 1..total_pages+2 {
        println!(
            "{}: {current_page} from {total_pages}",
            "Getting page".blue().bold(),
            current_page = page,
            total_pages = total_pages
        );
        let get_repos_endpoint: String = format!(
            "{base_url}/search/repositories?q=language:cairo&order=desc&per_page=100&page={page}",
            base_url = *GITHUB_API_BASE_URL,
            page = page
        );
        let get_repos_response = client
            .get(get_repos_endpoint)
            .header(USER_AGENT, "Cairo Lang statistics")
            .send()
            .await
            .unwrap();
        assert_eq!(&StatusCode::OK, &get_repos_response.status());
        let repos_reponse = get_repos_response.json::<Repos>().await.unwrap();
        // Initialize a vector of Items that will store all the repos
        let mut items = repos_reponse
            .items
            .into_iter()
            .map(|item| item.full_name)
            .collect::<Vec<String>>();
        repos_info_vec.append(&mut items);
    }
    // Print a success message with color to the console with the number of repos
    println!("{}", "Successfully retrived the Cairo Repos.".green().bold());
    // Return the number of repos and the vector with the full names of the repos
    Some(CairoRepos {
        num: n_repos as u64,
        repos: repos_info_vec,
    })
}


