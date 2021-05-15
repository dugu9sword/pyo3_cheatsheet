#[allow(dead_code)]

// use lib;
// use std::io::Error;
mod bert_tokenization;
#[allow(unused_imports)]
use std::time::{SystemTime};

fn main(){
    let mut x = String::from("hello");
    x.insert(0, 'd');
    x.insert_str(0, "string");
    x.replace_range(0..3, "wtf");
    // x.retain(|ele| ele < 'f');

    println!("{:?}", x.get(0..1));

    let x = "i'm happy   to (2ht) ) )go to fudan university, what about you?";
    println!("{:}", bert_tokenization::bert_tokenize(x).unwrap());
    
    let now = SystemTime::now();
    for _ in 0..10 {
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
