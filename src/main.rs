// use lib;
// use std::io::Error;
mod bert_tokenization;
use std::time::{Duration, SystemTime};

fn main(){
    // println!("{:}", bert_tokenization::bert_tokenize("i am happy to fudan university").unwrap());
    let x = "i am happy to fudan university";
    println!("{:}", bert_tokenization::bert_tokenize(x).unwrap());
    
    let now = SystemTime::now();
    for i in 0..1 {
        bert_tokenization::bert_tokenize(x);
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
