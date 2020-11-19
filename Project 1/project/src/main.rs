/// Eric Dennis
/// Fall 2020
/// CSIS 616
/// Project 1 and only
/// 
/// 
/// The purpose of this project will be to build experience in Rust programming while exploring
///     algorithms to process regular expressions.
/// 
/// You will build a Rust program to validate strings against a regular expression (definition below).
/// 
/// The program will: 
///     a) Accept a regular expression from the command line.
///     b) Build an internal representation of the state diagram for the regular expression.
///     c) Output to stdout the Graphiz definition of the state diagram.
///     d) Read lines from stdin. The reason for using stdin is that you can either type in lines to 
///         test with or produce a text file that you redirect into the program.
///     e) Each line from the file be a string that will be processed by the state machine.
///     f) If the string is accepted by the state machine (it matches the regular expression), print 
///         “Accept” and the string to stderr.
///     g) If the string is rejected by the state machine (it doesn’t match the regular expression), 
///         print “Reject” and the string to stderr
/// 

use serde::{Deserialize};
use std::io;
use std::io::prelude::*;
use std::io::Write;

// *********************************************************************
//// # Definition of a DFA
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
//// # State based representation of the RegEx
#[derive(Debug)]
struct StateGraph {
    /// The set of characters comprising the alphabet
    alphabet: Vec<char>,
    /// State number (0 relative) for the start state
    start_state: usize,
    /// Vector of state objects
    states: Vec<Box<State>>
}

fn main() {
    // Vector of the Alphabet ' ', 0-9, and a-z
    let alpha = vec![' ', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 
                        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',];
    // Vector of the possible operations
    let ops = vec!['(', ')', '*', '|'];     // no concatination operation symbol
                                            // '+', '\w', '\d' not included

    // DFA for ALL RegEx
    // sample.yaml is yaml DFA for ALL RegEx with all characters (42) in alphabet, includes symbols
    let filename = "sample.yaml";

    // Load the yaml file of DFA for ALL RegEx
    // Box pointing to a DFA instance on the heap
    let dfa = DFA::new_from_file(&filename);
    //println!("\nALL DFA DFA: \n{:?}", dfa);

    // Validate the DFA
    // dfa.validate().expect("Validation Failure:");

    // Create State Graph for ALL RegEx DFA
    let state_graph = StateGraph::new_from_dfa(&dfa);
    // eprintln!("\nALL DFA StateGraph: \n{:?}", state_graph);      // state_graph for ALL RegEx
    // state_graph.write_graphviz();                                // graphviz for ALL RegEx
    
    // Get Regex from CMD Line
    let reg = get_regex();

    // Needed another copy because of ownership rules
    let reg2 = reg.clone();

    println!("Your RegEx: {:?}", reg);
    
    // Validate the Symbos in RegEx
    validate_regex(&reg, &alpha, &ops);

    // Test that RegEx is in ALL RegEx
    match state_graph.test_regex(reg) {
        Ok(b) => println!("RegEx {} in the ALL RegEx language", 
                            if b {"is"} else {"is not"}),
        Err(s) => println!("Error processing sentence: {}", s)
    }
    
    // Get RegEx Alphabet to create regdfa
    let ralpha = get_reg_alpha(&reg2, &alpha, &ops);

    // Get RegEx Transitions/Rows to create regdfa
    let rtrans = get_reg_trans(&state_graph, reg2);

    // Get RegEx Columns to create regdfa
    let rcol = get_reg_cols(&dfa, &ralpha);

    // Additional Data to help me keep things straight
    // let mut sortrow = rtrans.clone();
    // println!("   RegEx Alphabet: {:?}", ralpha);
    // println!("\nRegEx Transitions: {:?}", rtrans);
    // println!("       RegEx Rows: {:?}", sortrow);
    // sortrow.sort();
    // println!("       RegEx Rows: {:?}", sortrow);
    // sortrow.dedup();
    // println!("       RegEx Rows: {:?}", sortrow);
    // println!("    RegEx Columns: {:?}", rcol);

    //// Create initial dfa representation of RegEx
    let regdfa = DFA::new_dfa_from_reg(&dfa, &rtrans, &ralpha, &rcol);
    //regdfa.print("\nBefore RegEx DFA: \n");

    //// Create new clean up dfa with reassigned state numbers
    let regdfa2 = DFA::clean_dfa(&regdfa);
    regdfa2.validate().expect("Validation Failure:");
    // regdfa2.print("\nAfter RegEx DFA: \n");
    regdfa2.print("\nRegEx DFA: \n");

    let regex_graph = StateGraph::new_from_dfa(&regdfa2);
    eprintln!("\nRegEx StateGraph: \n{:?}", regex_graph);
    regex_graph.write_graphviz();

    // Process through the input until end of file (cntl-z) is encountered
    println!("Enter a string to test against RegEx");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let sentence = &line.unwrap();
        match regex_graph.test_sentence(sentence) {
            Ok(b) => println!("{} <{}>", if b {"Accept"} else {"Reject"}, sentence),
            Err(s) => println!("Error processing sentence: {}", s)
        }
        println!("Enter another string or cntl-z to Exit");
    }
}

// *********************************************************************
/// Get RegEx from CMD Line
fn get_regex() -> Vec<char> {
    let mut s1 = String::new();
    println!("*****Enter a Regular Expression*****"); // Ask user for RegEx
    std::io::stdin().read_line(&mut s1).unwrap();
    // Check that something was entered
    if s1.len() <= 2 {
        // <= 2 because Rust includes \r\n as chars ending user input
        writeln!(std::io::stderr(), "Usage: Enter a string to validate").unwrap();
        std::process::exit(1);
    }
    //println!("RegEx {:?}", s1);
    let n = s1.chars().count() - 2;
    let s2 = &s1[0..n]; // use slice to remove \r\n chars
    let mut v = Vec::new();
    for c in s2.chars() {
        v.push(c);
    }
    v
}

// *********************************************************************
/// Validate the Symbos in RegEx
fn validate_regex(v: &Vec<char>, a: &Vec<char>, o: &Vec<char>) {
    
    // Check for Matching Parentheses
    let mut op = 0;
    let mut cp = 0;
    for i in 0..v.len() {
        if v[i] == '(' {
            op += 1;
        }
        if v[i] == ')' {
            cp += 1;
        }
    }
    if op != cp {
        if let Err(msg) = return_result(false) {
                println!("{}*****RegEx contains missmatched parentheses*****", msg);
                std::process::exit(1);
            }
        }

    // Check that all symbols are valid
    for i in 0..v.len() {
        if a.contains(&v[i]) || o.contains(&v[i]) {
            //println!("symbos are good");
        }
        else {
            if let Err(msg) = return_result(false) {
                println!("{}*****RegEx contains invlaid symbols*****", msg);
                std::process::exit(1);
            }
        }
    }
    println!("*****Symbos are Valid*****");
}

// *********************************************************************
/// Get RegEx Alphabet to create regdfa
fn get_reg_alpha(r: &Vec<char>, a: &Vec<char>, o: &Vec<char>) -> Vec<char> {
    let n = r.len();
    let mut ra = Vec::new();    // RegEx Alphabet
    for i in 0..n {
        if o.contains(&r[i]) {
            ra.push(r[i]);
        }
        if a.contains(&r[i]) {
            ra.push(r[i]);
        }
    }
    ra.sort();     // Sort acc order
    ra.dedup();    // Remove duplicates
    ra
}

// *********************************************************************
/// Get RegEx Transitions/Rows to create regdfa
fn get_reg_trans(sg: &StateGraph, r: Vec<char>) -> Vec<usize> {

    let mut state = sg.start_state;
    let mut trans = Vec::new();     // rows of interest per row of tranition table
    trans.push(0);  // Always push 0 for start state
    // A row of interest is the index of a row in the ALL RegEx DFA transitions table that was 
    // traversed when tracing the RegEx, entered on the CMD line, through the ALL RegEx DFA.
    // It tells us which rows/states to look for when creating the DFA for our RegEx entered on the CMD line.
    for ch in r {
        let state_no = match sg.alphabet.iter().position(|v| *v == ch) {
            Some(t) => t,
            None => 0
        };
        state = sg.states[state].transitions[state_no];
        trans.push(state+1);
    }
    trans
}

// *********************************************************************
/// Get RegEx Columns to create regdfa
fn get_reg_cols(dfa: &DFA, ra: &Vec<char>) -> Vec<usize> {
    let mut x = Vec::new();     // columns of interest per row of tranition table
        // A column of interest is the index value of a symbol in the ALL RegEx DFA alphabet.
        // It tells us which position to find the state transition for that symbol in each row.
        for ch in ra.iter() {
            for (v, sy) in dfa.alphabet.iter().enumerate() {
                if ch == sy {
                    x.push(v);
                }
            }
        }
    x
}

// *********************************************************************
impl DFA<> {
    /// Create and return a DFA on the heap
    /// Load the .yaml file specified into a DFA structure
    /// on the heap and return a point to it via a Box.
    fn new_from_file(filename: &str) -> Box<DFA> {

        let f = std::fs::File::open(filename)
                    .expect("Unable to open input");

        // Deserialize into the heap and return the pointer
        Box::new(serde_yaml::from_reader(f)
                    .expect("Unable to parse yaml") )
    }

    // *********************************************************************
    /// Create initial dfa representation of RegEx
    /// 
    /// This is one of the places I spent most of my time.  I am passing the ALL RegEx DFA(dfa), the 
    /// rows(y), colums(x), and alpabet(ra) of the RegEx entered on the CMD line.  In the nested for 
    /// and if statements below I am traversing the ALL RegEx DFA.  For each row of the dfa trasition
    /// table listed in _ysort, by looking though vec y for that value and pushing the values imedialty
    /// following, I create a keycol vector containing each transition for that state.  Using this 
    /// vector, I am then able to iterate through the DFA trasition matrix to the correct row and 
    /// column where I can find that state.  This state is then place it in the correct index of the
    ///  new transition vector representing that states possible transitions.
    /// 
    /// I kept coming back to this and the clean_dfa methods to try and get the correct transitions 
    /// for the DFA RegEx.  My hope was to find some clever way to do this and not use a mammoth and 
    /// hacky series of for loops and if statements.  What you see here is just the path of states 
    /// through the ALL RegEx DFA.  Its not acutally the dfa of the RegEx, and that isnt even right 
    /// in the majority of cases, but its the most complete version I can submit right now.
    /// 
    /// The program does pretty well with character strings.  Aldo, If you enter the same character 
    /// back to back, it acts like the + operator where it accepts one or more repetitions.
    /// I didn't do that on purpose.
    /// 
    fn new_dfa_from_reg(dfa: &DFA, y: &Vec<usize>, ra: &Vec<char>, x: &Vec<usize>) -> Box<DFA> {
        // New DFA structure
        let mut regdfa = Box::new(DFA{alphabet: ra.clone(), 
                                start: 1, accept: vec!(), 
                                transitions: vec!(),});

        // Build States for RegEx

        // Sort rows of interest and remove duplicates
        let mut _ysort = Vec::new();
        _ysort = y.clone();
        _ysort.sort();
        _ysort.dedup();
        //println!("y {:?}", y);
        //println!("_ysort {:?}", _ysort);

        //println!("Rows {:?}", _ysort);
        //println!("Columns {:?}", x);
        let mut keycol = Vec::new();

        for (_a, &ysortval) in _ysort.iter().enumerate() {
            let mut temp = Vec::new();
            //println!("Row needed index: {}", _a);
            //println!("Row needed: {}", ysortval);
            for (q, &yval) in y.iter().enumerate() {
                if ysortval == yval && q < y.len()-1 {
                    keycol.push(y[q+1]);
                }
            }
            //println!("keycol: {:?}", keycol);
            for (b, row) in dfa.transitions.iter().enumerate() {
                if ysortval == b {
                    //println!("DFA Row index: {}", b);
                    //println!("DFA Row: {:?}", row);
                    for (_c, &xval) in x.iter().enumerate() {
                        //println!("Col needed index: {}", _c);
                        //println!("Col needed: {}", xval);
                        for (d, &col) in row.iter().enumerate() {
                            if xval == d {
                                //println!("DFA Col index: {}", d);
                                //println!("DFA Col: {:?}", col);
                                if col > 0 && keycol.contains(&col) {
                                    temp.push(col);
                                }
                                else {
                                    temp.push(0);
                                }                                
                            }
                        }
                    }
                }
            }
            keycol.clear(); 
            //println!("Temp {:?}", temp);

            // Set Accept States
                // Not Correctly, but is relatively fixed in clean_dfa method
                // Since this is really only the path of states through the ALL RegEx DFA, I am 
                // looking for either the state with all 0's or the last state listed in vec y 
                // and making that my end state
            let mut var = 0;
            for &val in temp.iter() {
                var = var + val;
            }
            if var == 0 {
                regdfa.accept.push(ysortval);
            }
            else {
                regdfa.accept.push(y[y.len()-1]);
            }
            regdfa.transitions.push(temp);
        }
        regdfa
    }

    // *********************************************************************
    /// Create new clean up dfa with reassign state numbers
    /// 
    /// I used this method to clean up the DFA representing CMD line Regex.  Doing this made the 
    /// graphviz much simpler and also allowed me to reuse the validate method for the DFA.  
    /// 
    fn clean_dfa(messydfa: &DFA) -> Box<DFA> {
        // New DFA structure
        let mut tidydfa = Box::new(DFA{alphabet: messydfa.alphabet.clone(), 
                                start: 1, accept: vec!(), 
                                transitions: vec!(),});
        // Update State Numbers
        for row in messydfa.transitions.iter() {
            let mut tempvec = Vec::new();
            for i in 0..row.len() {
                if row[i] != 0 {
                    // index + 2 to update state numbers 
                    tempvec.push(i+2);
                    // Almost Correct Accept States
                    if messydfa.accept.contains(&row[i]) && !(tidydfa.accept.contains(&row[i])) {
                        tidydfa.accept.push(i+2);
                    }                    
                }
                else {
                    tempvec.push(0);
                }
            }
            //println!("tempvec: {:?}", tempvec);
            tidydfa.transitions.push(tempvec);
        }
        tidydfa
    }

    // *********************************************************************
    /// Validate the correctness of the DFA
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

    // *********************************************************************
    /// Print DFA
    fn print(&self, s: &str) {
        println!("{}: {:?}", s, self);
    }
}

// *********************************************************************
/// Implement the methods of the State Graph structure
impl StateGraph<> {

    /// Create a state graph from a DFA structure
    fn new_from_dfa(dfa: &DFA) -> Box<StateGraph> {

        // Create an empty graph object
        let mut graph = Box::new(StateGraph{alphabet: dfa.alphabet.clone(), 
                                        start_state: dfa.start - 1,
                                        states: vec!() });

        // Look through the transition table building state objects
        for row in dfa.transitions.iter() {
            let mut s = Box::new(State{accept_state: false, transitions: vec!()});
            for &col in row {
                if col > 0 {
                    s.transitions.push(col - 1);
                }
                else {
                    s.transitions.push(0);
                }
            }
            graph.states.push(s);
        }
        // Set the accept states
        for (_a, astate) in dfa.accept.iter().enumerate() {
            graph.states[*astate - 1].accept_state = true;
        }
        graph
    }

    // *********************************************************************
    /// Test that RegEx is in ALL RegEx
    /// modified your test_sentence method from HW3
    fn test_regex(&self, r: Vec<char>) -> Result<bool, String> {

        let mut state = self.start_state;
        for ch in r {
            let state_no = match self.alphabet.iter().position(|v| *v == ch) {
                Some(t) => t,
                None => return Err(format!("Character <{}> not in alphabet", ch))
            };
            state = self.states[state].transitions[state_no];
        }
        Ok(self.states[state].accept_state)
    }

    // *********************************************************************
    /// Execute the graph on a sentence
    /// Return Err if a character not in the alphabet is encountered
    /// Return Ok and a bool indicating accept (true) or reject (false)
    /// This is your test_sentence method from HW3
    fn test_sentence(&self, sentence: &str) -> Result<bool, String> {

        let mut state = self.start_state;
        for ch in sentence.chars() {
            let state_no = match self.alphabet.iter().position(|v| *v == ch) {
                Some(t) => t,
                None => return Err(format!("Character <{}> not in alphabet", ch))
            };
            state = self.states[state].transitions[state_no];
        }
        Ok(self.states[state].accept_state)
    }

    // *********************************************************************
    /// Write the graph to stdout
    fn write_graphviz(&self) {

        println!("\nRegEx Graphviz:{{");
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
                if state.transitions[i] > 0 {
                    println!("\tq{} -> q{} [label=\"{}\"];", n+1, state.transitions[i] + 1, ch);
                }
            }
        }
        println!("}}\n");
    }
}

// *********************************************************************
/// Result Method
fn return_result(tf : bool) -> Result<&'static str, &'static str> {    
    if tf {
        Ok("***GOOD***")
    }
    else {
        Err("*****ERROR*****")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dfa_test() {
        let mut testdfa = Box::new(DFA{alphabet: vec!(), 
                                start: 1, accept: vec!(), 
                                transitions: vec!(),});
        testdfa.alphabet.push('a');
        testdfa.alphabet.push('b');
        testdfa.alphabet.push('c');
        testdfa.accept.push(2);
        for _i in 0..5 {
            let mut temp = Vec::new();
            for q in 0..5 {
                temp.push(q)
            }
            testdfa.transitions.push(temp);
        }
        assert_eq!(testdfa.alphabet[1], 'b');
        assert_eq!(testdfa.accept[0], 2);
        for i in 0..5 {
            let temptest = testdfa.transitions[i].clone();
            for q in 0..5 {
                assert_eq!(temptest[q], q);
            }
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
}