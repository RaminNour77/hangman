use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rand::Rng;

fn read_words(filename: &str) -> Result<Vec<String>, ()>{
    //create a path to words.txt
    let path = Path::new(filename);
    // let display = path.display();

    //call raed_lines and then push words to vector
    let mut words = Vec::new();
    if let Ok(lines) = read_lines(&path){
        for line in lines {
            if let Ok(ip) = line {
                words.push(ip);
            }
        }
        return Ok(words);
    }
    
    return Err(());
}
//open and read lines form file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {

    println!("Hello, this is 'Hangman'");
    //unpack result of read_words
    if let Ok(words) = read_words("words.txt") {
        
        let mut rand_num = rand::thread_rng();
        let word_num = rand_num.gen_range(0, words.len());
        let word_to_guess = &words[word_num];

        let mut guess = String::new();
        for _c in word_to_guess.chars() {
            guess.push('_');
        }
        for c in guess.chars() {
            print!("{} ", c);
        }
        print!("\n");

        // guess[i] = word_to_gues[i]

        // let mut user_input = String::new();
        // let b1 = io::stdin().read_line(&mut user_input).unwrap();
        // println!("{}", words[word_num]);
    }


}
