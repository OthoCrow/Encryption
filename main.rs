use std::io::stdin;
fn main() {
    //loop for repetition
    loop {
        let alphabet = "abcdefghijklmnopqrstuvwxyz".to_string();
        let mut text = String::new();//String for holding inputs
        stdin().read_line(&mut text).expect("Error with input"); //Take an input and append to "text"
        let nigs = text.trim();
        for c in nigs.chars(){
            let index: Option<usize> = alphabet.find(c);
            let shift:usize = 3;
            let shifted:usize = index.unwrap() + shift;
            println!("{}", alphabet.chars().nth(shifted).unwrap());
        }
    }
}