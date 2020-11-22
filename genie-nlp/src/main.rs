use crate::genie::Genie;

mod search;
mod genie;
mod choice;

fn main() {
    Genie::perform_search_query("What is the square root of pi?");
}



