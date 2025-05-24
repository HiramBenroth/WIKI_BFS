// These are going to be my includes
mod console_helper;
mod wiki_nav;
mod graph;
mod bfs;

use console_helper::{ clear_console, input::get_input_string, Menu};
use bfs::{breadth_first_search, bi_directional_bfs};

#[tokio::main]
async fn main() {
    // Simple introduction

    let main : Menu  = Menu{
        name : "".to_string(),
        options: vec![
            "Exit", //0
            "Create New Search (Breadth First Search) ", //1
            "Create new Search (Bi Direction BFS)",//2
       ],
    };

    loop {
        clear_console();
        println!("Welcome to my Wikipedia Connections Breadth First Search Tool!");

        let choice = main.get_option();
        match choice {
            0 => break,
            1=> {
                let start_link = get_input_string("What is the first wikipedia link? ".to_string());
                let end_link = get_input_string("What is the second wikipedia link? ".to_string());
                breadth_first_search(start_link, end_link).await;
                get_input_string("Press Enter to continue".to_string());
            }
            2 => {
                let start_link = get_input_string("What is the first wikipedia link? ".to_string());
                let end_link = get_input_string("What is the second wikipedia link? ".to_string());
                bi_directional_bfs(start_link, end_link).await;
                get_input_string("Press Enter to continue".to_string());
            }
            _=> {
                break;
            }
        }
    }
}

