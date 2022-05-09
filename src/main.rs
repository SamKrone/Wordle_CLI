use colored::Colorize;
use rand::seq::IteratorRandom;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use text_io::read;
use std::io;
use std::io::Write;

fn main() {
    //Init
    let mut word = get_word();
    word = word.to_lowercase();
    let mut result = false;
    let mut count = 0;

    println!("{}", format!("Welcome to CMD Wordle").bold());
    println!("{}", format!("You have 6 guesses to find the word").bold());
    println!("");

    while result == false && count < 6 {
        let guess: String = input();
        let guess_arr = col_logic(&guess, &word);
        output(&guess_arr, &guess);
        result = end_logic(&guess_arr);
        count += 1
    }

    if result == true {
        println!("You win")
    } else {
        println!("You lose")
    }
    println!("The word was {word}");
}

fn get_word() -> String {
    //Read a line from wordList.txt and return as a string
    let f = File::open("src/wordList.txt")
        .unwrap_or_else(|e| panic!("(;_;) file not found: {}: {}", "src/wordList.txt", e));
    let f = BufReader::new(f);

    let lines = f.lines().map(|l| l.expect("Couldn't read line"));
    lines
        .choose(&mut rand::thread_rng())
        .expect("File had no lines")
}

fn input() -> String {
    //Return str input of 5 chars
    let mut value: String = read!();
    value = value.to_lowercase();
    if value == "panic!" { panic!("Terminated")} //Debug
    while value.chars().count() != 5 {
        println!("Error: Incorrect guess length");
        println!("");
        value = read!();
        println!("")
    }
    value
}

fn col_logic(guess_pass: &String, word_pass: &String) -> [u8; 5] {
    //An int_key is stored for each char of the guess
    //2: The character matches in the correct position
    //1: The character matches in the incorrect position
    //0: The character does not appear with the correct asnwer
    let mut int_key: u8;
    let mut temp_array: [u8; 5] = [0, 0, 0, 0, 0];
    for s in 0..5 {
        int_key = 0;
        if guess_pass.chars().nth(s).unwrap() == word_pass.chars().nth(s).unwrap() {
            int_key = 2
        } else {
            for n in 0..5 {
                if guess_pass.chars().nth(s).unwrap() == word_pass.chars().nth(n).unwrap() {
                    int_key = 1 
                }
            }
        }
        temp_array[s] = int_key;
    }
    temp_array
}

fn output(local_arr: &[u8; 5], guess_pass: &String) {
    //Outputs the user input with coloured text to indicate
    //if they are correct or not based off of the int_key
    //2: Green
    //1: Yellow
    //0: Red
    for x in 0..5 {
        match local_arr[x] {
            2 => print!("{}", format!("{}", guess_pass.chars().nth(x).unwrap()).bold().green()),
            1 => print!("{}", format!("{}", guess_pass.chars().nth(x).unwrap()).bold().yellow()),
            0 => print!("{}", format!("{}", guess_pass.chars().nth(x).unwrap()).bold().red()),
            _ => panic!("Error in match")
        }
        io::stdout().flush().unwrap();
    }
    println!("\n")
}

fn end_logic(local_arr: &[u8; 5]) -> bool {
    //A check to see if the correct word has been found
    let mut result: bool = false;
    let sum: u8 = local_arr.iter().sum();
    //Sum var is the total of the int_keys from col_logic()
    //As such, sum == 10 means all letters match
    if sum == 10 {
        result = true;
    }
    result
}