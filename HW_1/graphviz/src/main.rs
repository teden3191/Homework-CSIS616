//  Eric Dennis
//  CSIS 616
//  HW1 - Graphviz Automata Generator
//  9/9/2020

// write message to terminal on failure
use std::io::Write;

fn main() {
    // Initialize Vector to store user input
    let mut input = Vec::new();
    // push 2nd arg to input vector, 1st arg/command is skipped
    for arg in std::env::args().skip(1) {
        input.push(arg);
    }

    // Check that vector lenghth is only one
    if input.len() != 1 {
        writeln!(std::io::stderr(), "Usage: Enter comma seperated node names, NO SPACES!!!")
            .unwrap();
        std::process::exit(1);
    }

    // Initialize string to store/clean up user input
    let nodestr = &input[0];
    // Initialize new vector to store cleaned up user input
    let mut nodes = Vec::new();
    // Iterate though string, seperate on comma, and push to nodes vector
    for x in nodestr.split(",") {
        nodes.push(x);
    }

    // Initialize variable for lenght of nodes vector
    let length = nodes.len();
    // Graph header
    println!("digraph {{");
    // Orient graph left to right
    println!("rankdir=LR;");
    // Start node and first state
    println!("start -> {};", nodes[0]);
    // Iterate through vector to get node names
    for y in 1..length {
        println!("{} -> {}", nodes[y-1], nodes[y]);
    }

    // Formatting start arrow
    println!("start [shape=point];");
    // Iterat though vector to format nodes
    for z in 1..(length) {
        println!("{} [shape=circle];", nodes[z-1]);
    }
    
    // Format end state with double circles
    println!("{} [shape=circle, peripheries=2];", nodes[length-1]);
    // Print closing bracket
    println!("}}")
}

