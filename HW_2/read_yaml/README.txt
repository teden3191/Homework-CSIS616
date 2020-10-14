Navigate to folder 'read_yaml folder and run "cargo run sample.yaml"

Program prints the following to the terminal:

1) The DFA Struct created from the yaml file, with added n_states usize variable and states vector.

2) A node for each state deailing its identifier, states that tranistion to that state, states that this
	state transitions to, and a boolean representing wheather it is an accepting state or not
	(true means it is an accepting state)

3) The Graphviz dot code to generate the Automata for the DFA
	I used a Graphviz sandbox to test the dot code output. 
	http://www.webgraphviz.com/


Each of the requirements is labeled in the code.  Section #) and Part letter)


As you can tell, I am not really using the graph structure like you showed us in class.  Instead, I stored 
all the needed information for each state in a node struct and pushed each one to a vector.  I fought with 
the Box Option struct for a few days and decided it would be better to have a vector full of nodes insead 
of a Box Option Tree Graph of only the empty node pointing to the first state.  I have included the code I
wrote before giving up and moving on with the Vector of Node Structs.  This file is named main_NOTCOMPLETE.rs
and is located in the backup folder.

Thanks,

Eric