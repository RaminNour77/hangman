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
        //.len() gives you the number of bytes and not the character count, so this will differ for graphemes
        let word_num = rand_num.gen_range(0, words.len());      //random number that corresponds with number of word in words.txt
        let word_to_guess = &words[word_num];                   
        let mut user_input = String::new();                     //initialization of user input
        let lives = word_to_guess.chars().count() + 2;          // the number of lives a player has based on the word they get

        let mut guess = String::new();
        for _c in word_to_guess.chars() {
            guess.push('_');
        }
        for c in guess.chars() {
            print!("{} ", c);
        }
        print!("\n");

        //take user input as long as they have lives length
        for _life in 0..lives{
            let b1 : String = io::stdin().read_line(&mut user_input).unwrap().to_string(); // not getting the right character somehow!
            //debug helper
            print!("{}", &b1);
            println!("{}", word_to_guess.contains(&b1));
            //this whole if-else needs to be better!

            /*TODO:
            check if the input is in the word_to_guess
            if it is replace it in the guess string
            else display lives and hangman!*/
            if word_to_guess.contains(&b1){
                println!("{} is in the word good job!", b1);
            } 
            else {
                println!("{}/{} lives left", (lives - (_life + 1)), lives);
            }
            
            }
        println!("{}", words[word_num]);
    }


}
