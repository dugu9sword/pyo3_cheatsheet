use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use itertools::join;
use std::collections::HashSet;
use lazy_static::lazy_static;

lazy_static! {
    static ref VOCAB: HashSet<String> = {
        let path = "res/vocab.txt";
        let input = File::open(path).unwrap();
        let buffered = BufReader::new(input);
        let mut vocab: HashSet<String>= HashSet::new();
        for line in buffered.lines() {
            vocab.insert(line.unwrap());
        }
        vocab
    };
    static ref PUNCT: HashSet<char> = [
        '.', ',', '\'', '"', ':',  ';', '-', '?', '!', '(', ')'
    ].iter().cloned().collect();
}


pub fn bert_tokenize(sent: &str) -> Result<String, Error> {
    // let space_split_words: Vec<&str> = sent.split(" ").collect();  
    let mut words: Vec<&str> = Vec::new();
    let mut c_sid = 0;
    for c_eid in 0..sent.len(){
        let c = sent.chars().nth(c_eid).unwrap();
        if c == ' ' || PUNCT.contains(&c) || c_eid == sent.len() {
            if c_sid != c_eid{
                words.push(&sent[c_sid..c_eid]);
            }
            c_sid = c_eid + 1;
        }
        if PUNCT.contains(&c) {
            words.push(&sent[c_eid..c_eid + 1]);
        }
    }
    // print!("{:?}", words);

    let mut subwords: Vec<String> = Vec::new();
    for word in words{
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
                if VOCAB.contains(&sub){
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
