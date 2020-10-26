//use serde::{Deserialize};
use std::io::Write;


// *********************************************************************
//// # Definition of a Regex
#[derive(Debug)]
struct Regex {
    /// The vector of the regex the StateGraph represents
    regex: Vec<char>,
    /// The set of characters comprising the alphabet
    alphabet: Vec<char>,
    /// The set of characters comprising the operators
    operators: Vec<char>,
    /// Start State set to 0 on creation
    start: usize,
    /// Set of accept states (0 relative)
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
//// # State based representation of the Regex
#[derive(Debug)]
struct StateGraph {
    /// The vector of the regex the StateGraph represents
    regex: Vec<char>,
    /// The set of characters comprising the alphabet
    alphabet: Vec<char>,
    /// The set of operators in the regex
    operators: Vec<char>,
    /// State number (0 relative) for the start state
    //start_state: usize,
    /// Vector of state objects
    states: Vec<Box<State>>
}

fn main() {

    // Vector of the Alphabet a-z, 0-9, and ' '
    let alpha = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
                     's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ' '];
    // Vector of the possible operations
    let ops = vec!['*', '|', '(', ')'];   // concatination not included since there is no operation symbol

    // println!("Alphabet: {:?}", alpha);
    // println!("Operators: {:?}", ops);

    // Read Regex from CMD Line
    let reg = get_regex();
    println!("Your Regex: {:?}", reg);
    validate_regex(&reg, &alpha, &ops);

    let mut regex = Regex::create_regex(&reg, &alpha, &ops);
    println!("{:?}", regex);
    regex.get_transitions();    
    println!("{:?}", regex);
    //regex.get_accept_state();

    let state_graph = StateGraph::construct_graph(&regex);
    println!("{:?}", state_graph);

    state_graph.write_graphviz();

    // Read in test string
    let vinput = get_inputstring();

    // Validate a test string and show its state transitions
    state_graph.validate_string(&vinput);

}

// *********************************************************************
//// Get Regex from CMD Line
fn get_regex() -> Vec<char> {
    let mut s1 = String::new();
    println!("*****Enter REGEX to Check*****"); // Ask user for test string
    std::io::stdin().read_line(&mut s1).unwrap();
    // Check that something was entered
    if s1.len() <= 2 {
        // <= 2 because Rust includes \r\n as chars ending user input
        writeln!(std::io::stderr(), "Usage: Enter a string to validate").unwrap();
        std::process::exit(1);
    }
    //println!("Test String {:?}", s1);
    let n = s1.chars().count() - 2;
    let s2 = &s1[0..n]; // use slice to remove \r\n chars
    let mut v = Vec::new();
    for c in s2.chars() {
        v.push(c);
    }
    v
}

// *********************************************************************
//// Validate the symbos in regex
fn validate_regex(v: &Vec<char>, a: &Vec<char>, o: &Vec<char>) {
    println!("**********validate_regex method**********");
    for i in 0..v.len() {
        if a.contains(&v[i]) || o.contains(&v[i]) {
            //println!("symbos are good");
        }
        else {
            if let Err(msg) = return_result(false) {
                println!("{}*****Regex contains invlaid symbols*****", msg);
                std::process::exit(1);
            }
        }
    }
    println!("*****Symbos are Valid*****");
}

// *********************************************************************
//// Get Test String to Validate against Regex
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
impl Regex<> {
    fn create_regex(v: &Vec<char>, a: &Vec<char>, o: &Vec<char>) -> Box<Regex> {
        
        let x = v.len();        
        let mut a1 = Vec::new();
        for i in 0..x {
            if a.contains(&v[i]) {
                let x = v[i];
                a1.push(x);
            }
        }
        a1.sort();
        a1.dedup();

        let mut o1 = Vec::new();
        for i in 0..x {
            if o.contains(&v[i]) {
                let y = v[i];
                o1.push(y);
            }
        }
        o1.sort();
        o1.dedup();

        let regex = Box::new(Regex{regex: v.clone(), alphabet: a1, operators: o1, 
                                        start: 0, accept: vec!(), transitions: vec!()});
        regex
    }

    fn get_transitions(&mut self){
        let r = self.regex.clone();
        let x = r.len();
        println!("len of regex {}", x);
        let mut t: Vec<usize> = Vec::new();
        let mut k: Vec<usize> = Vec::new();
        let mut trans: Vec<Vec<usize>> = Vec::new();
        let mut s1 = 0;
        let mut s2 = 1;
        let mut pps1 = 0;
        let mut pps2 = 0;
        for i in 0..x {
            println!("i = {}, s1 = {}, s2 = {}, x = {}", i, s1, s2, x);
            if i < x-1 && r[i+1] == '*' {
                t.push(s1);
                t.push(s1);
                k = t.clone();
                trans.push(k);
                t.clear();
            }
            else if r[i] == '|' {
                println!("ON THE PIPE");
                s1 = 0;
            }
            else if i > 0 && r[i-1] == '|' {
                println!("CHAR AFTER THE PIPE");
                t.push(0);
                t.push(pps2);
                s1 = pps2;
                s2 = s2 + 1;
                k = t.clone();
                trans.push(k);
                t.clear();
            }
            else if r[i] == '*' {
                // DO NOTHING
                if r[i+1] == '|' {
                    pps1 = s1;
                    pps2 = s2;
                }
            }
            else {
                if i < x-1 && r[i+1] == '|' {
                    println!("CHAR BEFORE THE PIPE");
                    pps1 = s1;
                    pps2 = s2+1;
                    t.push(s1);
                    t.push(s2);
                    s1 = s1 + 1;
                    s2 = s2 + 1;
                    k = t.clone();
                    trans.push(k);
                    t.clear();
                }
                else {
                    println!("*****ELSE*****");
                    t.push(s1);
                    t.push(s2);
                    s1 = s1 + 1;
                    s2 = s2 + 1;
                    k = t.clone();
                    trans.push(k);
                    t.clear();
                }
            }
        }
        println!("{:?}", trans);
        self.transitions = trans.clone();
    }

    fn get_accept_state(&mut self){
        
    }
}

// *********************************************************************
//// Implement the methods of the StateGraph structure
impl StateGraph<> {
    fn construct_graph(reg: &Regex) -> Box<StateGraph> {

        let mut graph = Box::new(StateGraph{regex: reg.regex.clone(), alphabet: reg.alphabet.clone(),
                                    operators: reg.operators.clone(), states: vec!()});
        let x = reg.transitions.len();
        let mut ta: Vec<usize> = Vec::new(); 
        let mut s = 0;
        for i in 0..x {
            let mut st = Box::new(State{accept_state: false, transitions: vec!()});            
            // if i + 1 == x {
            //     st.accept_state = true;
            // }
            ta = reg.transitions[i].clone();
            for j in 0..ta.len() {
                st.transitions.push(ta[j]);
            }
            graph.states.push(st);
        }
        graph
    }

    //// Write the graph to stdout
    fn write_graphviz(&self) {

        println!("digraph {{");
        println!("\trankdir=LR;");
        println!("\tnode [shape=point]; start;");
        
        for (n, state) in self.states.iter().enumerate() {
            if state.accept_state {
                println!("\tnode [shape=doublecircle]; q{};", n);
            }
        }
        
        println!("\tnode [shape=circle];");
        println!("\tstart -> q{}", 0);
        let mut q = 0; //using this to track and skip operator symbols
        for (n, state) in self.states.iter().enumerate() {
            if self.regex[q] == '*' || self.regex[q] == '|' || self.regex[q] == '(' || self.regex[q] == ')' {
                q = q + 1;
                println!("\tq{} -> q{} [label=\"{}\"];", state.transitions[0], state.transitions[1], self.regex[q]);
                q = q + 1;
            }
            else {
                println!("\tq{} -> q{} [label=\"{}\"];", state.transitions[0], state.transitions[1], self.regex[q]);
                q = q + 1;
            }
        }

        println!("}}");
    }
    
    //// Validate Test String and show State Transitions
    fn validate_string(&self, v: &Vec<char>) {

        // Quick check to make sure all chars in test string/vector are part of the regex alphabet
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
                if v[i] == self.regex[i] && n == m {
                    println!("\t\u{03B4} (q{}, {}) -> q{}", m, v[i], state.transitions[1]);
                    m = state.transitions[1];
                    curr = &self.states[state.transitions[0]];
                    break;
                }
            }
        }

        let s: String = v.into_iter().collect();
        // String Accepted
        if curr.accept_state == true {
            println!("*****Accept {}*****", s);
        }
        // String Rejected
        else {
            println!("*****Reject {}*****", s);
        }
    }

}

// Result Method
fn return_result(tf : bool) -> Result<&'static str, &'static str> {    
    if tf {
        Ok("***GOOD***")
    } else {
        Err("*****ERROR*****")
    }
}

