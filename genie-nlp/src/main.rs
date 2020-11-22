use crate::genie::Genie;
use std::env;

mod search;
mod genie;
mod choice;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        Genie::perform_search_query(args[1].as_str());
    } else {
        println!("Please provide a question encapsulated in quotations.")
    }
}



