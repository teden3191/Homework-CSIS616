Navigate to 'read_yaml2 folder and run "cargo run sample.yaml"

Programs asks the user to enter a string to test against the DFA provided in the yaml file

Once user enters a string, the program will check:
	1) did user actually enter characters

	2) are all characters in the string included in the alphabet for the DFA

Program then loops through the string and the StateGraph of the DFA to validate the string.

Each state transition is printed to the console, and once finished the program will state
	whether the string is accepted or rejected by the DFA

I added two methods to the code you provided from HW2.
	get_inputstring, line 122
	validate_string, line 231

Thanks,

Eric