extern crate log;
use crate::utils::github_calls::get_cairo_repos;

pub mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    get_cairo_repos();

    Ok(())
}
