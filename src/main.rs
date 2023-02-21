extern crate log;
use utils::github_structs::Contributor;

use crate::utils::github_calls::{get_cairo_repos, get_contributors};
use crate::utils::github_structs::{CairoRepos, Starker};

pub mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get the cairo repos using get_cairo_repos() and store them in a CairoRepos struct
    let cairo_repos = get_cairo_repos().unwrap();

    let mut starkers: Vec<Starker> = Vec::new();
    // for each element in cairo_repos.repos obtain the contributors and store them in a Contributor struct
    for repo in cairo_repos.repos {
        let contributors = get_contributors(&repo).unwrap(); 
        // Eliminate the duplicates from the contributors vector and sum their contributions
        // Push the new contributor to the starkers vector

        // push the contributors to the starkers vector and sum their contributions
        for contributor in contributors {
            starkers.push(contributor);
        }
        

    

        // for contributor in contributors {
        //     // Check if the contributor is already in the contributor vector
        //     if contributor.login == contributor.login {
        //         // If it is, sum the contributions
        //         contributor.contributions += contributor.contributions;
        //     } else {
        //         // If it is not, push the contributor to the contributor vector
        //         contributors.push(contributor);
        //     }
        // }

        dbg!(contributors);
    }

    // // get the contributors for the first repo in cairo_repos using get_contributors() and store them in a Contributor struct
    // let contributors = get_contributors(&cairo_repos.repos[0]).unwrap();
    // dbg!(contributors);

    Ok(())
}
