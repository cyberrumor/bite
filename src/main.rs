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
    let filenames: &[String] = &args[1..args.len()];

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
    let mut db: Vec<lib::Sentence> = Vec::new();
    for original in original_sentences {
        let stripped: String = lib::strip_nonalpha(&original.clone().to_lowercase());
        let tokenized: Vec<String> = lib::tokenize_words(&stripped);
        let trees: Vec<String> = lib::get_trees(tokenized.clone());
        let object: lib::Sentence = lib::Sentence {
            original,
            stripped,
            tokenized,
            trees,
            score: 0.0,
        };
        db.push(object)
    }


    let heatmap: HashMap<String, usize> = lib::gen_heatmap(&db);

    // get average lengths
    let mut lengths: usize = 0;
    for s in &db {
        lengths += s.trees.len();
    }
    let ave_length: f64 = lengths as f64 / db.len() as f64;

    // give each tree a score
    for s in &mut db {
        let mut score: usize = 0;
        for tree in &s.trees {
            score += *heatmap.get(tree).unwrap();
        }
        s.score = score as f64 / ave_length as f64;
    }

    // get the average scores and length
    let mut all_scores: f64 = 0.0;
    for s in &db {
        all_scores += s.score;
    }
    let ave_score: f64 = all_scores as f64 / db.len() as f64;

    let mut summary: String = String::new();
    for s in &db {
        if s.score as f64 > ave_score * 2.0 {
            summary.push_str(&s.original);
            summary.push(' ');

        }

    }
    summary = summary.trim_end().to_string();
    println!("{:?}", summary); 

}
