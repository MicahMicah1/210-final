use std::collections::{HashMap, VecDeque};


// Implements Graph methods and definitions and creates a function that finds the shortest path in a directed graph.

// Graph (Struct): A struct that takes n (number of vertices) and outedges (each vertices outedges/neighbors)
pub struct Graph {
    pub n: usize,
    pub outedges: Vec<Vec<usize>>,
}

impl Graph {
    // a function that initializes a instance of a graph where n is the number of vertices and the outedges are an empty vector of vectors.
    pub fn new(n: usize) -> Self { // only takes n as an argument
        Graph {
            n,
            outedges: vec![vec![]; n],
        }
    }//returns an instance of a graph with n and a vector of n empty vectors.
    
    // A function that adds directed edges to an initialized graph.
    pub fn add_directed_edges(&mut self, edges: &[(usize, usize)]) { // takes an instance of a graph and a slice/reference of edges where origin and destination are in tuple form. 
        for (u, v) in edges { // Iterates through each tuple pair and takes first entry as index and pushes second entry onto its outedges vector.
            self.outedges[*u].push(*v);
        }// returns the graph instance with outedges added on.
    }
    

    // Iterates through each vertices' outedges and sorts in ascending order.
    pub fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }
    // A function that takes n (number of vertices) and a reference to tuples of edges and returns an instance of a graph.
    pub fn create_directed(n: usize, edges: &[(usize, usize)]) -> Self {
        let mut g = Self::new(n); // creates a new graph with initialized empty vectors in the outedges 
        g.add_directed_edges(edges); // uses the add directed edge function on the edge list to create the edge list
        g.sort_graph_lists();   // sorts the outedges
        g     // returns the new directed graph
    }
}
// A function that takes a reference to a graph, a hashmap with names as keys and ids as values, a hashmap with ids as keys and names as values, and from and to arguments that are both string slices.
pub fn find_transitive_win_path(
    graph: &Graph,
    name_to_id: &HashMap<String, usize>,
    id_to_name: &HashMap<usize, String>,
    from: &str,
    to: &str,
) {
    let start = match name_to_id.get(&from.to_lowercase()) { // takes the string entered as a starting point and makes it case sensitive and then matches to a numerical value.
        Some(id) => *id, 
        None => {
            println!("Fighter '{}' not found.", from);
            return;
        } // checks to ensure the fighter entered is found
    };
    let goal = match name_to_id.get(&to.to_lowercase()) {
        Some(id) => *id, // takes the string entered as the ending point and makes it case sensitive and then matches to a numerical value.
        None => {
            println!("Fighter '{}' not found.", to);
            return;
        }
    }; // checks to ensure the fighter entered is found

    let mut visited = vec![false; graph.n]; // initializes a vector of n# falses that repersent whether a node has been visitied
    let mut parent = vec![None; graph.n]; // initialzes an empty vector of n entries to be used to track who a nodes parents were 
    let mut queue = VecDeque::new(); // initialze a queue

    visited[start] = true; 
    queue.push_back(start); //add starting entry to the queue

    while let Some(current) = queue.pop_front() { // Take the first item in queue and peform BFS
        if current == goal {
            break; // checks if we have found the desired end fighter break the loop.
        }
        for &neighbor in &graph.outedges[current] { 
            if !visited[neighbor] { // for each item in the current's neighbor list that we have not visited, mark as visited and then add it to the parent list and then add the unvisisted neighbor to the queue.
                visited[neighbor] = true;
                parent[neighbor] = Some(current);
                queue.push_back(neighbor);
            }
        }
    }
    // if we cannot find a path do:
    if !visited[goal] {
        println!("No transitive win path from '{}' to '{}'.", from, to);
        return;
    } 
    //if we can find a path:
    let mut path = vec![];
    let mut current = goal;
    // add each number in the parent vector to a new vector and then push the start vector and then reverse to 'reconstruct' path
    while let Some(p) = parent[current] {
        path.push(current);
        current = p;
    }
    path.push(start);
    path.reverse();

    // itereate through each number in the path vector and use the hashmap id to name to find the corresponding name. then collect each name into a new vector.
    let path_names: Vec<String> = path
        .into_iter()
        .map(|id| id_to_name.get(&id).unwrap().clone())
        .collect();
    println!("Transitive win path: {:?}", path_names);
}
