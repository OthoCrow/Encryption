use std::io::stdin;

fn main() {
    //loop for repetition
    loop {
        let alphabet = "abcdefghijklmnopqrstuvwxyz".to_string();
        let caps_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
        let mut input = String::new(); //String for holding inputs
        stdin().read_line(&mut input).expect("Error with input");  //Take an input and append to "input"
        // Makes string immutable and trims of whitespaces 
        let text = input.trim();
        let mut output = String::new();
        for c in text.chars(){
            // If statement to manage spaces
            if c == ' ' {
                output.push(' ');
            }
            // Manages capital letters
            else if caps_alphabet.contains(c) {
                let index:Option<usize>  = caps_alphabet.find(c);
                let shift = 3;
                // Apply shift with wrapping allowed
                let shifted= (index.unwrap() + shift) % caps_alphabet.len();
                //Push encrypted content to a string 
                output.push(caps_alphabet.chars().nth(shifted).unwrap());
            }
            // Manages lowercase letters
            else if alphabet.contains(c){
                let index:Option<usize>  = alphabet.find(c);
                let shift = 3;
                // Apply shift with wrapping allowed
                let shifted= (index.unwrap() + shift) % alphabet.len();
                //Push encrypted content to a string 
                output.push(alphabet.chars().nth(shifted).unwrap());
            }
            // Prints � for unknown characters
            else {
                output.push('�');
            }
        }
        //Print encrypted content 
        println!("{}", output);
    } 
}