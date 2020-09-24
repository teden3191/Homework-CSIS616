// Eric Dennis
// CSIS 616
// HW2 yaml DFA and Graph
// Fall 2020

use std::io::Write;
use serde::{Deserialize};

// DFA Structure
#[derive(Debug, Deserialize)]
struct DFA {
    alphabet: Vec<char>,
    start: u32,
    accept: Vec<u32>,
    transitions: Vec<Vec<u32>>,    
    // n_states and states field not loaded from the YAML file
    // so we need to provide a default value for it 
    #[serde(default)]
    n_states: usize,
    #[serde(default)]
    states: Vec<u32>
}

// Definition a state node
#[derive(Debug)]
struct Node {
    state: u32,                   // Name of the State, initialized with u32 val when new node created
    to_this_state: Vec<u32>,      // states that transition to this node, initialized as empty vector
    from_this_state: Vec<u32>,    // states this node transitions to, initialized as empty vector
    acc: bool                     // boolean to mark if node is an accepting state, initialized as false
}

fn main() {
    let filename = get_filename(std::env::args());
    // Load the yaml file getting a Box pointing to a DFA instance on the heap
    let mut d = DFA::new_from_file(&filename);
    // Get number of states and add to 
    d.compute_states();
    // Get all possible states
    d.get_states();
    d.print("Your DFA: ");
    d.check_dfa();
    create_graph(d.start, d.accept.clone(), d.transitions.clone(), d.states, d.n_states);
    graphviz(d.accept, d.transitions, d.n_states);
}

/// Return the filename passed as the first parameter
fn get_filename(args: std::env::Args) -> String {
    // Get the arguments as a vector
    let args: Vec<String> = args.collect();
    // Make sure only one argument was passed
    if args.len() != 2 {
        writeln!(std::io::stderr(), "Usage: hw1 dfafile")
            .unwrap();
        std::process::exit(1);
    }    
    args[1].to_string()
}   

impl DFA {
    /// Create and return a DFA on the heap
    /// 
    /// Load the .yaml file specified into a DFA structure
    /// on the heap and return a point to it via a Box.
    fn new_from_file(filename: &str) -> Box<DFA> {
        let f = std::fs::File::open(filename)
                    .expect("Unable to open input");
        // Deserialize into the heap and return the pointer
        Box::new(serde_yaml::from_reader(f)
                    .expect("Unable to parse yaml"))
    }

    // Compute the number of states
    fn compute_states(&mut self) {
        self.n_states = self.transitions.len();
    }

    // Get Possible States From Transitions, sort and remove duplicates
    fn get_states(&mut self) {
        let x = self.n_states;
        for i in 0..x {
            let t = &self.transitions[i];
            let s1 = t[0];
            let s2 = t[1];
            self.states.push(s1);
            self.states.push(s2);
        }
        self.states.sort();     // Sort acc order
        self.states.dedup();    // Remove duplicates
    }

    // Section 2) - Check the DFA for Errors
    fn check_dfa(&mut self) {
        let star = &self.start;
        let stat = &self.states;
        let acc = &self.accept;
        let trans = &self.transitions;
        let n = self.n_states;
        let x = acc.len();
        let m = stat.len();
        let mut master_trans_arr = Vec::new();  // will use this later in Part b)

        //  Part a) - Check that transitions states are valid states        
        for i in 0..n {
            let t = &trans[i];
            let s1 = t[0];
            let s2 = t[1];
            master_trans_arr.push(s1);  // pushing all states listed in transitions
            master_trans_arr.push(s2);  // vector to masttransarr
            if stat.contains(&s1) && stat.contains(&s2){
                true;
            }
            else {
                if let Err(msg) = return_result(false) {
                    println!("Transition State Not in Possible States: {}", msg);
                    std::process::exit(1);
                }
            }
        }

        // Part b) - Check that all states are referenced in transitions
        for i in 0..m {
            if master_trans_arr.contains(&stat[i]) {
                true;
            }
            else {
                if let Err(msg) = return_result(false) {
                    println!("All states are not referenced in Transitions: {}", msg);
                    std::process::exit(1);
                }
            }
        }

        //  SECTION 2 part c
        // Check that start state is in set of states
        if stat.contains(star) {
            true;
        }
        else {
            if let Err(msg) = return_result(false) {
                println!("Start State Not in Possible States: {}", msg);
                std::process::exit(1);
            }
        }

        //  SECTION 2 part c continued
        // Check that set of accept states is part of set of all states
        for i in 0..x {
            if stat.contains(&acc[i]) {
                true;
            }
            else {
                if let Err(msg) = return_result(false) {
                    println!("Accepting State Not in Possible States: {}", msg);
                    std::process::exit(1);
                }
            }
        }
    }

    // Print DFA
    fn print(&self, s: &str) {
        println!("{}: {:?}", s, self);
    }
}

impl Node {    
    /// Create a new tree node with the value specified
    fn new(v: u32) -> Node{        
        Node {state: v, to_this_state: Vec::new(), from_this_state: Vec::new(), acc: false}
    }
}

//  Section 3) - DFA to Graph Structure
fn create_graph(star: u32, acc: std::vec::Vec<u32>, 
        trans: std::vec::Vec<std::vec::Vec<u32>>, stat: std::vec::Vec<u32>, x: usize) {
    
    // Initialize vector to hold all state structs
    let mut node_vector = vec![];

    // Empty node points to start state
    let mut empty = Node::new(0);
    empty.from_this_state.push(star);
    
    // First State
    let mut first_state = Node::new(star);
    first_state.to_this_state.push(empty.state);
    for i in 0..x {
        let t = &trans[i];
        let s1 = t[0];
        let s2 = t[1];
        // add states that first_state transitions to
        if s1 == first_state.state {
            first_state.from_this_state.push(s2);
        }
        // is this an accept state
        if acc.contains(&first_state.state) {
            first_state.acc = true;
        }
    }

    // Push empty node and first_state to Vector of nodes, "node_vector"
    node_vector.push(empty);
    node_vector.push(first_state);
    
    // create other remaining statea
    for i in 1..stat.len() {
        let current_s = stat[i];
        let mut new_state = Node::new(current_s);
        for j in 0..x {
            let t = &trans[j];
            let s1 = t[0];
            let s2 = t[1];
            // add states that THIS state transitions to
            if s1 == new_state.state {
                new_state.from_this_state.push(s2);
            }
            // add states that transition to THIS state
            if s2 == new_state.state {
                new_state.to_this_state.push(s1);
            }
            // is THIS state an accept state
            if acc.contains(&new_state.state) {
                new_state.acc = true;
            }
        }
        // push each state to the Vector of nodes, "node_vector"
        node_vector.push(new_state);
    }
    print_nodes(node_vector);
}

// Section 4) - Print Method for the Nodes Vector
fn print_nodes(v: Vec<Node>) {
    for i in 0..v.len() {
        println!("{:?}", v[i]);
    }
}

// Section 5) - Graphviz
fn graphviz(acc: std::vec::Vec<u32>, trans: std::vec::Vec<std::vec::Vec<u32>>, x: usize) {
    println!("digraph {{
        rankdir=LR;
        node [shape=point]; start;
        node [shape=doublecircle]; {:?}
        node [shape=circle];", acc);
    
    // Start with our dummy node
    let t = &trans[0];
    let s1 = t[0];
    println!("\tstart -> {};", s1);

    for i in 0..x {
        let t = &trans[i];
        let s1 = t[0];
        let s2 = t[1];
        println!("\t{} -> {};", s1, s2);
    }
    // Print the closing for the digraph
    println!("}}");
}

// Result Method
fn return_result(tf : bool) -> Result<&'static str, &'static str> {    
    if tf {
        Ok("Is OK man")
    } else {
        Err("Not OK Man!!!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn node_test() {
        // Did we get a node containing the values we set?
        let mut node = Node::new(53);
        node.to_this_state.push(52);
        node.from_this_state.push(54);
        assert_eq!(node.state, 53);
        assert_eq!(node.to_this_state[0], 52);
        assert_eq!(node.from_this_state[0], 54);
        assert_eq!(node.acc, false);
    }

    #[test]
    fn result_1() {
        // Get an Ok return code from the test function
        assert!(match return_result(true) {
                    Ok(msg) => {
                        println!("Ok: {}", msg);
                        true
                    }
                    Err(_) => false
                });
    }

    #[test]
    fn result_2() {
        // Get an Err return code from the test function
        assert!(match return_result(false) {
                    Err(msg) => {
                        println!("Err: {}", msg);
                        true
                    }
                    Ok(_) => false
                });
    }
}