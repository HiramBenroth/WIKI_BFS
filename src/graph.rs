use crate::wiki_nav::{extract_title_from_url, get_links};
use std::collections::{HashMap, VecDeque, HashSet};
use futures::stream::{FuturesUnordered, StreamExt};

/* GRAPH STRUCTURE START */
pub struct Graph {
    queue: VecDeque<u64>,         // ids to visit
    hash_queue : HashSet<u64>,
    pub nodes: HashMap<u64, Node>,    // visited and known nodes
    pub step: u32,              // step counter
    pub destination: Option<u64>,
    pub depth: u16,
    pub end_hash : u64
}

impl Graph {
    // setup graph from starting url
    pub fn new(url: String, is_one : Option<u64>) -> Self {
        let title = extract_title_from_url(&url).unwrap();
        let id = calculate_hash(&title);
        let root_node  = Node::new(title, None, 0, id);
        let mut new_queue: VecDeque<u64> = VecDeque::new();
        let mut node_map = HashMap::new();
        let mut new_hqueue = HashSet::new();
        new_hqueue.insert(id);
        new_queue.push_back(id);
        node_map.insert(id, root_node);

        Self {
            queue: new_queue,
            nodes: node_map,
            step: 0,
            destination: None,
            depth : 0,
            hash_queue :new_hqueue,
            end_hash: is_one.unwrap_or_default()
        }
    }

    // runs 1 step and returns true if target is reached
    pub async fn step_unidir(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        if self.end_hash == 0 {
            return Ok(false);
        }

        // step 1: Prepare a batch of up to 5 node IDs from the queue
        let batch_size = 10;
        let mut batch_ids = Vec::new();

        for _ in 0..batch_size {
            if let Some(id) = self.queue.pop_front() {
                self.hash_queue.remove(&id);
                self.step += 1;

                // step 2: Check if current node is the destination
                if self.end_hash == id {
                    self.destination = Some(id);
                    return Ok(true);
                }

                batch_ids.push(id);
            }
        }

        // step 3: Set up asynchronous tasks to fetch links concurrently
        let mut tasks = FuturesUnordered::new();

        for id in &batch_ids {
            // step 3a: Retrieve the node and check visited status
            if let Some(node) = self.nodes.get_mut(id) {
                if node.visited {
                    continue;
                }

                node.visited = true;

                // step 3b: Track new depth if needed
                if node.depth > self.depth {
                    self.depth = node.depth;
                }

                // step 3c: Prepare async task to fetch links
                let site = node.site.clone();
                let node_id = *id;
                tasks.push(async move {
                    (node_id, get_links(&site).await.unwrap_or_default())
                });
            }
        }

        // step 4: Process all fetched link results
        while let Some((parent_id, links)) = tasks.next().await {
            let mut hash_map: HashMap<u64, String> = links
                .iter()
                .map(|link| (calculate_hash(link), link.to_string()))
                .collect();

            // step 4a: Retain only new (unvisited and unqueued) nodes
            hash_map.retain(|id, _| !self.nodes.contains_key(id) && !self.hash_queue.contains(id));

            // step 4b: Create new nodes and add them to graph
            for (id, link) in hash_map {
                let new_node = Node::new(link, Some(parent_id), self.depth + 1, id);
                self.nodes.insert(id, new_node);
                self.queue.push_back(id);
                self.hash_queue.insert(id);
            }
    }

    Ok(false)

}

    pub async fn step_bidir(&mut self, goal : &Graph) -> Result<bool, Box<dyn std::error::Error>> {
        let end_hash = &goal.nodes;

        // step 1: Prepare a batch of up to 5 node IDs from the queue
        let batch_size = 10;
        let mut batch_ids = Vec::new();

        for _ in 0..batch_size {
            if let Some(id) = self.queue.pop_front() {
                self.hash_queue.remove(&id);
                self.step += 1;

                // step 2: Check if current node is the destination
                if end_hash.contains_key(&id) {
                    self.destination = Some(id);
                    return Ok(true);
                }

                batch_ids.push(id);
            }
        }

        // step 3: Set up asynchronous tasks to fetch links concurrently
        let mut tasks = FuturesUnordered::new();

        for id in &batch_ids {
            // step 3a: Retrieve the node and check visited status
            if let Some(node) = self.nodes.get_mut(id) {
                if node.visited {
                    continue;
                }

                node.visited = true;

                // step 3b: Track new depth if needed
                if node.depth > self.depth {
                    self.depth = node.depth;
                }

                // step 3c: Prepare async task to fetch links
                let site = node.site.clone();
                let node_id = *id;
                tasks.push(async move {
                    (node_id, get_links(&site).await.unwrap_or_default())
                });
            }
        }

        // step 4: Process all fetched link results
        while let Some((parent_id, links)) = tasks.next().await {
            let mut hash_map: HashMap<u64, String> = links
                .iter()
                .map(|link| (calculate_hash(link), link.to_string()))
                .collect();

            // step 4a: Retain only new (unvisited and unqueued) nodes
            hash_map.retain(|id, _| !self.nodes.contains_key(id) && !self.hash_queue.contains(id));

            // step 4b: Create new nodes and add them to graph
            for (id, link) in hash_map {
                let new_node = Node::new(link, Some(parent_id), self.depth + 1, id);
                self.nodes.insert(id, new_node);
                self.queue.push_back(id);
                self.hash_queue.insert(id);
            }
    }

    Ok(false)

}


    // backtrack path from destination to root
    pub fn get_path(&self) -> Vec<String> {
        let mut path = Vec::new();
        let mut current_id = match self.destination {
            Some(id) => id,
            None => return path,
        };

        while let Some(node) = self.nodes.get(&current_id) {
            path.push(node.site.clone());
            match node.parent_id {
                Some(pid) => current_id = pid,
                None => break,
            }
        }

        path.reverse();
        path
    }
}

/* GRAPH STRUCTURE END */

/* NODE STRUCTURE START */
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Node {
    pub parent_id: Option<u64>,    // who linked to this node
    id: u64,                   // hash of site name
    pub depth: u16,                // how far from start
    pub site: String,              // page name
    pub visited: bool          // if already explored
}

impl Node {
    // make a new node from title and parent info
    fn new(title: String, parent: Option<u64>, depth: u16, id: u64) -> Self
    {
        //let id = calculate_hash(title.as_str());
            Node {
                parent_id: parent,
                depth,
                id,
                site: title,
                visited: false,
            }
    }
}
/* NODE STRUCTURE END */

// simple hasher for node ID from string
pub fn calculate_hash(s: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}
