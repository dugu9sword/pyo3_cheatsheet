use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use itertools::join;
use std::collections::HashSet;


pub fn bert_tokenize(sent: &str) -> Result<String, Error> {
    let path = "res/vocab.txt";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut vocab: HashSet<String>= HashSet::new();

    for line in buffered.lines() {
        vocab.insert(line?);
    }

    let words: Vec<&str> = sent.split(" ").collect();   
    let mut subwords: Vec<String> = Vec::new();
    for word in words.iter(){
        let mut start_id = 0;
        let mut end_id = word.len();
        // println!("tokenize: {}", word);
        'start_loop: loop{
            'end_loop: loop{
                if end_id <= start_id {
                    subwords.push("[UNK]".to_string());
                    break 'start_loop;
                }
                let mut sub = word[start_id..end_id].to_string();
                if start_id != 0 {
                    sub.insert_str(0, "##");
                }
                if vocab.contains(&sub){
                    subwords.push(sub);
                    break 'end_loop;
                }
                end_id = end_id - 1;
            }
            if end_id == word.len(){
                break 'start_loop;
            }else{
                start_id = end_id;
                end_id = word.len();
            }
        }
    }
    let ret = join(&subwords, " ");
    Ok(ret)
}
