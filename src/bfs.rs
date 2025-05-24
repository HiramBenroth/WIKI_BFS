use crate::graph::{Graph, calculate_hash };
use crate::console_helper::clear_console;
use crate::wiki_nav::extract_title_from_url;
use std::time::Instant;

pub async fn breadth_first_search(start: String, end: String) {
    clear_console();
    println!("The Breadth First Search is initializing.");

    // Need to implement 2 Graphs but only use one
    let end_title = extract_title_from_url(&end).unwrap();
    let id = calculate_hash(&end_title);
    let mut graph :Graph = Graph::new(start , Some(id));

    let mut step : u32 = 0;
    let mut last_dip = 0;

    let start_time = Instant::now();

    while let Ok(false) = graph.step_unidir().await {
        step += 5;
        if step >= last_dip + 50 {
            clear_console();
            println!("The Breadth First Search is analizing.");
            println!("Current step: {}", graph.step );
            println!("Current depth: {}", graph.depth);
            last_dip = step;
        }
    }

    let dur = start_time.elapsed();
    clear_console();

    println!("The Breadth First Search is analizing.");
    println!("Current step: {}", graph.step );
    println!("Current depth: {}", graph.depth);


    println!("Found Destination");

    let path = graph.get_path();

    // print as single breadcrumb line
    println!("{}", path.join(" -> "));

    // print each step on its own line
    for (i, site) in path.iter().enumerate() {
        println!("step {}: {}", i, site);
    }

    println!("\nCompleted {} steps in {:.2?}. \nThats {:.2?} per step.", step, dur, (dur / step) )

} 

pub async fn bi_directional_bfs(start: String, end: String) {
    clear_console();
    println!("The Breadth First Search is initializing.");

    //Implement 2 graphs
    let mut end = Graph::new(end, None);
    let mut start = Graph::new(start, None);

    let mut stepTotal : u32 = 0;
    let mut lastDisp: u32 = 0;
    let mut found = false;
    let mut graphstep = 0;

    let start_time = Instant::now();

    while found == false {
        if graphstep == 0 {
            //This wil be start graph
            found = start.step_bidir(&end).await.unwrap();
            if found {end.destination = start.destination};
            stepTotal += 5;
            graphstep = 1;
        }
        else {
            //This is the other graph
            found = end.step_bidir(&start).await.unwrap();
            if found {start.destination = end.destination};
            stepTotal += 5;
            graphstep = 0;
        }

        // if stepTotal == lastDisp + 10 {
        //     clear_console();
        //     println!("The Breadth First Search is analizing.");
        //     println!("Current step: {}", start.step + end.step);
        //     println!("Current Start depth: {}", start.depth);
        //     println!("Current End depth: {}", end.depth);
        //     lastDisp = stepTotal;
        // }
    }
    
    


    let dur = start_time.elapsed();
    clear_console();

    let stepTotal = start.step + end.step;

    println!("The Breadth First Search is analizing.");
    println!("Current step: {}", stepTotal);
    println!("Current Start depth: {}", start.depth);
    println!("Current End depth: {}", end.depth);
    println!("Found Destination");


    let path_s = start.get_path();
    let path_e = end.get_path();

    // Combine paths, excluding the middle node from path_e to avoid duplication
    let mut full_path = path_s.clone(); // Clone path_s to start with
    if !path_e.is_empty() {
        // Append path_e in reverse (from middle to end), excluding the first element (middle node)
        full_path.extend(path_e.iter().rev().skip(1).cloned());
    }

    // Print as a single breadcrumb line
    println!("Full Path = {}", full_path.join(" -> "));

    // Print each step in one column
    for (i, site) in full_path.iter().enumerate() {
        println!("step {}: {}", i, site);
    }

    println!("\nCompleted {} steps in {:.2?}. \nThats {:.2?} per step.", stepTotal, dur, (dur / stepTotal) )

} 