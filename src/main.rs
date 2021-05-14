// use lib;
// use std::io::Error;
mod bert_tokenization;
use std::time::{Duration, SystemTime};

fn main(){
    let x = "i'm happy   to go to fudan university, what about you?";
    println!("{:}", bert_tokenization::bert_tokenize(x).unwrap());
    
    let now = SystemTime::now();
    for i in 0..10 {
        // bert_tokenization::bert_tokenize(x).unwrap();
    }
    match now.elapsed() {
        Ok(elapsed) => {
            println!("{}", elapsed.as_millis());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }    
}
