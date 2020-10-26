// Eric Dennis
// CSIS 616
// HW3 yaml DFA and Test String State Transitions
// Fall 2020
// Started with Dr Crosby code for HW2
// Added get_inputstring method to get a test string and validate_string method
//      to validate the string and print the state transitions for the test string
//      based on the StateGraph derived from the DFA discribed in the yaml file

//! CSIS-616 - Program #2
//! 
//! Ralph W. Crosby PhD.
//! 
//! 
//! # Usage
//! 
//! ```
//! program2_drc filename
//! ```
//! or
//! ```
//! cargo run filename
//! ```
//! 
//! where: `filename` is a yaml file containing the DFA definition
//! 
//! # Output
//! 
//! To `stderr`: Debug display of the internal graph structure
//! 
//! To `stdout`: Graphviz definitions of the graph structure

use serde::{Deserialize};
use std::io::Write;

// *********************************************************************
//// # Deterministic Finite Automata Structure
//// 
//// Create a structure that the YAML files will be deserialized into.
//// Note the use of the `Deserialize` trait
//// 
#[derive(Debug, Deserialize)]
struct DFA {

    /// The set of characters comprising the alphabet
    alphabet: Vec<char>,
    /// State number (1 relative) for the start state
    start: usize,
    /// Set of accept states (1 relative)
    accept: Vec<usize>,
    /// Matrix of transitions, rows are states, columns characters in the alphabet
    transitions: Vec<Vec<usize>>,    
}

// *********************************************************************
//// # Definition of a single state
#[derive(Debug)]
struct State {
    /// Is this an accept state
    accept_state: bool,
    /// Set of transitions (0 relative)
    transitions: Vec<usize>
}

// *********************************************************************
//// # State based representation of the DFA
#[derive(Debug)]
struct StateGraph {
    /// The set of characters comprising the alphabet
    alphabet: Vec<char>,
    /// State number (0 relative) for the start state
    start_state: usize,
    /// Vector of state objects
    states: Vec<Box<State>>
}

// *********************************************************************
fn main() {

    // Get and validat the filename on the command line
    let filename = get_filename(std::env::args());

    // Load the yaml file getting a Box pointing to a DFA
    // instance on the heap
    let dfa = DFA::new_from_file(&filename);
    //dfa.print("DFA from yaml: ");

    // Validate the DFA
    dfa.validate().expect("Validation Failure:");

    // Get a state structure for the DFA
    let state_graph = StateGraph::new_from_dfa(&dfa);

    println!("{:?}", state_graph);
    state_graph.write_graphviz();

    // Get String to Validate against DFA
    let vinput = get_inputstring();
    //println!("Test String as vec: {:?}", vinput);

    // Validate a test string and show its state transitions
    state_graph.validate_string(&vinput);
}

// *********************************************************************
//// Return the filename passed as the first parameter
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

// *********************************************************************
//// Get Test String to Validate against DFA
fn get_inputstring() -> Vec<char> {
    let mut s1 = String::new();
    println!("*****Enter String to Check*****");    // Ask user for test string
    std::io::stdin().read_line(&mut s1).unwrap();
    // Check that something was entered
    if s1.len() <= 2 {      // <= 2 because Rust includes \r\n as chars ending user input
        writeln!(std::io::stderr(), "Usage: Enter a string to validate")
            .unwrap();
        std::process::exit(1);
    }
    //println!("Test String {:?}", s1);
    let n = s1.chars().count() - 2;
    let s2 = &s1[0..n];     // use slice to remove \r\n chars
    let mut v = Vec::new();
    for c in s2.chars() {
        v.push(c);
    }
    v
}

// *********************************************************************
//// Implement the methods of the DFA structure
impl DFA {

    //// Create and return a DFA on the heap
    //// 
    //// Load the .yaml file specified into a DFA structure
    //// on the heap and return a point to it via a Box.
    fn new_from_file(filename: &str) -> Box<DFA> {

        let f = std::fs::File::open(filename)
                    .expect("Unable to open input");

        // Deserialize into the heap and return the pointer
        Box::new(serde_yaml::from_reader(f)
                    .expect("Unable to parse yaml") )
    }

    //// Validate the correctness of the DFA
    fn validate(&self) -> Result<(), String> {

        // The number of characters in the alphabet should match the number
        // of columns in each state row
        for (rnum, row) in self.transitions.iter().enumerate() {

            if row.len() != self.alphabet.len() {
                return Err(format!("Wrong number of columns({}) in row {}, should be {}",
                                    row.len(), rnum + 1, self.alphabet.len() ))
            }
        }

        // Validate that all states in the transition table are valid
        for (rnum, row) in self.transitions.iter().enumerate() {
            for (cnum, state) in row.iter().enumerate() {

                if *state as usize >  self.transitions.len() {
                    return Err(format!("Invalid transition state({}) in row {}, column {}",
                                        state, rnum + 1, cnum + 1 ))
                }    
            }
        }

        // The start and accept states must be valid
        if self.start as usize > self.transitions.len() {
            return Err(format!("Start state({}), is not valid", self.start))
        }

        for acc_state in self.accept.iter() {
            if *acc_state as usize  > self.transitions.len() {
                return Err(format!("Accept state({}), is not valid", acc_state))
            }
        }
        Ok(())
    }
    // //// Guess what this does!
    // fn print(&self, s: &str) {
    //     println!("{}: {:?}", s, self);
    // }
}

// *********************************************************************
/// Implement the methods of the State Graph structure
impl StateGraph<> {

    //// Create a state graph from a DFA structure
    fn new_from_dfa(dfa: &DFA) -> Box<StateGraph> {

        // Create an empty graph object
        let mut graph = Box::new(StateGraph{alphabet: dfa.alphabet.clone(), 
                                            start_state: dfa.start - 1,
                                            states: vec!() });

        // Look through the transition table building state objects
        for row in dfa.transitions.iter() {
            let mut v = Box::new(State{accept_state: false, transitions: vec!()});
            for col in row {
                v.transitions.push(col-1);
            } 
            graph.states.push(v);
        }    

        // Set the accept states
        for astate in dfa.accept.iter() {
            graph.states[*astate - 1].accept_state = true;
        }
        graph
    }

    //// Validate Test String and show State Transitions
    fn validate_string(&self, v: &Vec<char>) {

        // Quick check to make sure all chars in test string/vector are part of the alphabet
        let alpha = &self.alphabet;
        for i in 0..v.len() {
            if alpha.contains(&v[i]) {
                true;
            }
            else {
                if let Err(msg) = return_result(false) {
                    println!("{} String contains Characters that are not in the alphabet: {:?}", msg, alpha);
                    std::process::exit(1);
                }
            }
        }
        
        // Loop through test string/vector and StateGraph to validate each transition
        let mut curr = &self.states[0];
        let mut m = 0;
        for i in 0..v.len() {
            for (n, state) in self.states.iter().enumerate() {
                //println!("n = {}, m = {}", n, m);
                // We see a X and n = state we are transitioning from
                if v[i] == alpha[0] && n == m {
                    println!("\t\u{03B4} (q{}, {}) -> q{}", m+1, v[i], state.transitions[0]+1);
                    m = state.transitions[0];
                    curr = &self.states[state.transitions[0]];
                    break;
                }
                // We see a Y and n = state we are transitioning from
                if v[i] == alpha[1] && n == m {
                    println!("\t\u{03B4} (q{}, {}) -> q{}", m+1, v[i], state.transitions[1]+1);
                    m = state.transitions[1];
                    curr = &self.states[state.transitions[1]];
                    break;
                }
            }
        }
        // String Accepted
        if curr.accept_state == true {
            println!("*****String Accepted*****");
        }
        // String Rejected
        else {
            if let Err(msg) = return_result(false) {
                println!("{} String ends in a Non Accepting State", msg);
                std::process::exit(1);
            }
        }
    }

    //// Write the graph to stdout
    fn write_graphviz(&self) {

        println!("digraph {{");
        println!("\trankdir=LR;");
        println!("\tnode [shape=point]; start;");
        
        for (n, state) in self.states.iter().enumerate() {
            if state.accept_state {
                println!("\tnode [shape=doublecircle]; q{};", n+1);
            }
        }
        
        println!("\tnode [shape=circle];");
        println!("\tstart -> q{}", self.start_state+1);

        for (n, state) in self.states.iter().enumerate() {

            for (i, ch) in self.alphabet.iter().enumerate() {
                println!("\tq{} -> q{} [label=\"{}\"];", n+1, state.transitions[i] + 1, ch);
            }
        }

        println!("}}");
    }

    // Result Method
}

// Result Method
fn return_result(tf : bool) -> Result<&'static str, &'static str> {    
    if tf {
        Ok("***ACCEPTED***")
    } else {
        Err("*****String REJECTED*****")
    }
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