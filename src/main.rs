use std::env;
use regex::Regex;

fn main() {
    let branch_ref = env::var("GITHUB_HEAD_REF").expect("GITHUB_HEAD_REF must be set");
    let ticket_matcher = Regex::new(r"([a-zA-Z]+-[0-9]+).*").unwrap();

    match ticket_matcher.captures(branch_ref.as_str()) {
        None => {
            println!("{} does not contain a ticket id", branch_ref);
            std::process::exit(1);
        },
        Some(matches) => {
            let ticket_name = matches.get(1).expect("expected a match").as_str();
            println!("::set-output name=ticket-id::{}", ticket_name);
        }
    }
}
