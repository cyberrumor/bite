use std::env;
use std::process::exit;
use std::collections::HashMap;
mod lib;

#[derive(Debug, Clone)]
struct Sentence {
    original: String,
    stripped: String,
    tokenized: Vec<String>,
    trees: Vec<Vec<String>>,
    score: f32,
}

fn main() {
    let args: Vec<String> = env::args()
        .collect();

    if args.len() < 2 {
        println!("usage: bite filename1.txt filename2.txt 3");
        exit(1);
    }

    let filenames: &[String] = &args[1..args.len() - 1];
    // let summary_length: usize = args.last().unwrap().parse().unwrap();

    // stringify corpora
    let mut corpora: String = String::new();
    for file in filenames {
        let corpus: String = lib::stringify_corpus(file.clone());
        corpora.push_str(&" ");
        corpora.push_str(&corpus);
    }
    
    // split into sentences
    let original_sentences: Vec<String> = lib::tokenize_sent(&corpora);


    // create sentence objects
    let mut db: Vec<Sentence> = Vec::new();
    for original in original_sentences {
        let stripped: String = lib::strip_nonalpha(&original.clone().to_lowercase());
        let tokenized: Vec<String> = lib::tokenize_words(&stripped);
        let trees: Vec<Vec<String>> = lib::get_trees(tokenized.clone());
        let object: Sentence = Sentence {
            original,
            stripped,
            tokenized,
            trees,
            score: 0.,
        };
        db.push(object)
    }

    for s in &db {
        println!();
        println!("{:?}", s.original);
        println!("{:?}", s.stripped);
        println!("{:?}", s.tokenized);
        println!("{:?}", s.trees);
        println!("{:?}", s.score);
    }
















}
