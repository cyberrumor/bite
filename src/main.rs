use std::env;
use std::process::exit;
use std::collections::HashMap;
mod lib;

fn main() {
    let args: Vec<String> = env::args()
        .collect();

    if args.len() < 1 {
        println!("usage: bite filename1.txt filename2.txt");
        exit(1);
    }

    let filenames: &[String] = &args[1..];

    // stringify corpi
    let mut corpora: String = String::new();
    for file in filenames {
        let corpus: String = lib::stringify_corpus(file.clone());
        corpora.push_str(&corpus);
    }

    let words: Vec<String> = lib::tokenize_words(corpora);
    let frequency_dict: HashMap<String, usize> = lib::gen_frequency(&words, 1);
    let max_value: usize = frequency_dict.values().max().unwrap().clone();
    println!("max value: {:?}", max_value);
}
