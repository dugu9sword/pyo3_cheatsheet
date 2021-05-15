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
    let mut char_vec: Vec<char> = sent.chars().collect();
    let mut i = 0;
    let mut last_is_space = false;
    loop{
        let c = char_vec[i];
        // strip spaces
        if c == ' ' && last_is_space {
            char_vec.remove(i);
            continue;
        }
        if PUNCT.contains(&c) {
            if !last_is_space{
                char_vec.insert(i, ' ');
                i += 1;
            }
            char_vec.insert(i + 1, ' ');
            i += 2;
            last_is_space = true;
        } else {
            i += 1;
            last_is_space = c == ' ';
        }
        
        if i == char_vec.len(){
            break
        }
    }
    let sent: String = char_vec.iter().collect();
    // println!("{:?}", sent);
    let words: Vec<&str> = sent.trim().split(' ').collect();
    // println!("{:?}", words);

    let mut subwords: Vec<String> = Vec::new();
    let mut tmp_str = String::with_capacity(32);

    for word in words{
        let mut start_id = 0;
        // println!("tokenize: {}", word);
        loop{
            let mut end_id = word.len();
            tmp_str.clear();
            if start_id != 0 {
                tmp_str.push_str("##");
            }
            tmp_str.push_str(&word[start_id..end_id]);
            loop {
                if VOCAB.contains(&tmp_str){
                    subwords.push(tmp_str.clone());
                    break;
                }
                end_id -= 1;
                // if end_id == start_id {
                //     subwords.push(String::from("[UNK]"));
                //     break;
                // }
                tmp_str.pop();
            }
            start_id = end_id;
            if start_id == word.len(){
                break;
            }
        }
    }
    let ret = join(&subwords, " ");
    Ok(ret)
}
