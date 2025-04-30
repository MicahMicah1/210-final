use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Write};

mod readdata;
use readdata::{DataFrame, ColumnVal};

mod winpath;
use winpath::{Graph, find_transitive_win_path};
// import modules

// defines types vertex which is a unsigned interger and list of edges which is a vector of tuples with unsigned integers
type Vertex = usize;
type ListOfEdges = Vec<(Vertex, Vertex)>;

fn main() -> Result<(), Box<dyn Error>> {
    let types = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]; // defines types, I ended up just doing all strings as I dont really use the other data.
    let mut df1 = DataFrame::new(); // initialze a new data frame
    df1.read_csv("ufc-data.csv", &types)?; //read data from file and uses the defined types
    let mut name_to_id: HashMap<String, usize> = HashMap::new(); // initialze an empty hashmap
    let mut id_counter = 0; // initialze a coutner starting at 0
    let mut edges: ListOfEdges = vec![]; // initialze an empty edge vector

    // A loop that reads through each row in the dataframe and takes the column corresponding to the winners and losers and creates a clone to be used for the hashmapping
    for row in &df1.data { 
        let winner = match &row[9] {
            ColumnVal::One(s) => s.clone(),
            _ => continue,
        };
        let loser = match &row[10] {
            ColumnVal::One(s) => s.clone(),
            _ => continue,
        };

        //converts to lowercase to get rid of case sensitivity.
        let winner_lc = winner.to_lowercase();
        let loser_lc = loser.to_lowercase();

        // for each winner and loser check if they exist in name_to_id, if they do enter the number into winner id, if they dont add assign them new number then add one to the counter
        let winner_id = *name_to_id.entry(winner_lc.clone()).or_insert_with(|| {
            let id = id_counter;
            id_counter += 1;
            id
        });

        let loser_id = *name_to_id.entry(loser_lc.clone()).or_insert_with(|| {
            let id = id_counter;
            id_counter += 1;
            id
        });
        //pusbes a new edge with winner_id and loser_id into a vector
        edges.push((winner_id, loser_id));
    }
    //create a directed graph with the number of vertices equaling the number of unique fighters and the edges using the created vector
    let graph = Graph::create_directed(id_counter, &edges);
    // 
    let mut id_to_name: HashMap<usize, String> = HashMap::new(); // initialzes a hashmap that will  extract fighter names from their fighter ID
    for row in &df1.data {
        if let (ColumnVal::One(winner), ColumnVal::One(loser)) = (&row[9], &row[10]) { // if both winner and loser are present in the data assign to a name and iterate
            // find the id num of both winner and loser
            let wid = *name_to_id.get(&winner.to_lowercase()).unwrap(); 
            let lid = *name_to_id.get(&loser.to_lowercase()).unwrap();
            // enter in the id num into the hashmap as the key and then enter in a clone of the string as the value.
            id_to_name.entry(wid).or_insert_with(|| winner.clone());
            id_to_name.entry(lid).or_insert_with(|| loser.clone());
        }
    }
    // create mutable empty strings for inputs.
    let mut input1 = String::new();
    let mut input2 = String::new();

    // allows user to enter in  names

    print!("\nEnter winner's name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input1).unwrap();

    print!("Enter target loser's name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input2).unwrap();

    let from = input1.trim();
    let to = input2.trim();

    find_transitive_win_path(&graph, &name_to_id, &id_to_name, from, to);

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use winpath::Graph;
    #[test]
    fn test_basic_graph() { // test a directed graph is constructed. 
        let mut name_to_id = HashMap::new();
        name_to_id.insert("a".to_string(), 0);
        name_to_id.insert("b".to_string(), 1);
        name_to_id.insert("c".to_string(), 2);

        let mut id_to_name = HashMap::new();
        id_to_name.insert(0, "A".to_string());
        id_to_name.insert(1, "B".to_string());
        id_to_name.insert(2, "C".to_string());

        let edges = vec![(0, 1), (1, 2)];
        let graph = Graph::create_directed(3, &edges);

        assert_eq!(graph.outedges[0], vec![1]); // A -> B
        assert_eq!(graph.outedges[1], vec![2]); // B -> C
        assert_eq!(graph.outedges[2].len(), 0); // C -> no one
    }
    #[test]
    fn test_name_to_id_mapping() { // test for name to ID
        let mut name_to_id = HashMap::new();
        name_to_id.insert("dustin poirier".to_string(), 0);
        name_to_id.insert("conor mcgregor".to_string(), 1);

        assert_eq!(name_to_id.get("dustin poirier"), Some(&0));
        assert_eq!(name_to_id.get("conor mcgregor"), Some(&1));
    }
    #[test]
    fn test_id_to_name_mapping() {  // test for Id to name 
        let mut id_to_name = HashMap::new();
        id_to_name.insert(0, "Dustin Poirier".to_string());
        id_to_name.insert(1, "Conor McGregor".to_string());
        
        assert_eq!(id_to_name.get(&0), Some(&"Dustin Poirier".to_string()));
        assert_eq!(id_to_name.get(&1), Some(&"Conor McGregor".to_string()));
    }
    #[test]
    fn test_path_does_not_panic() { // Test to make sure my simple graph runs
        let mut name_to_id = HashMap::new();
        name_to_id.insert("dustin poirier".to_string(), 0);
        name_to_id.insert("conor mcgregor".to_string(), 1);

        let mut id_to_name = HashMap::new();
        id_to_name.insert(0, "Dustin Poirier".to_string());
        id_to_name.insert(1, "Conor McGregor".to_string());
        let edges = vec![(0, 1)];
        let graph = Graph::create_directed(2, &edges);
        find_transitive_win_path(&graph, &name_to_id, &id_to_name, "Dustin Poirier", "Conor McGregor");
    }

        
}




