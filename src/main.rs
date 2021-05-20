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
        if s.score as f64 > ave_score * 2.5 {
            summary.push_str(&s.original);
            summary.push(' ');

        }

    }
    summary = summary.trim_end().to_string();
    println!("{:?}", summary); 
    println!();





    // begin text generation

    // get a heatmap of the most common endings
    // let endmap: HashMap<String, usize> = lib::gen_sidemap(&db, "end".to_string());
    let begmap: HashMap<String, usize> = lib::gen_sidemap(&db, "beg".to_string());

    let mut generated_sentence: String = String::new();
    let mut last_choice: Vec<String> = Vec::new();

    // prime generated sentence and last choice
    {
        let mut tval: usize = 0;
        for (key, value) in begmap.iter() {
            if value > &tval {
                tval = *value;
                last_choice.push(key.to_string());
            }
        }
        generated_sentence.push_str(&last_choice.last().unwrap());
        generated_sentence.push(' ');
    }


    // loop {
    //    let mut most_common: String = String::new();
    //    let mut tval: usize = 0;
    //    let last_word: String = last_choice
    //        .last()
    //        .unwrap()
    //        .split_whitespace()
    //        .last()
    //        .unwrap()
    //        .to_string();

    //    for (key, value) in heatmap.iter() {
    //        let first_word: String = key.split_whitespace().next().unwrap().to_string();
    //        if !last_choice.contains(&key.to_string())
    //        && !first_word.is_empty()
    //        && first_word == last_word {
    //            // println!("key: {:?} value: {:?} tval: {:?}", key, value, tval);
    //            if value > &tval {
    //                most_common = key.to_string();
    //                tval = *value;
    //            }
    //        }
    //    }
    //    // generated_sentence.push_str(&most_common);
    //    last_choice.push(most_common.clone());
    //    let last_word: String = most_common
    //       .clone()
    //        .split_whitespace()
    //        .last()
    //        .unwrap()
    //        .to_string();

    //    generated_sentence.push_str(&last_word);
    //    generated_sentence.push(' ');
    //    if endmap.get(&most_common).is_none() {
    //        continue;
    //    } else if endmap.get(&most_common).unwrap() * 3 >= *heatmap.get(&most_common).unwrap() {
    //        break;
    //    }

    //}
    let result = generated_sentence.trim();
    println!("{:?}", result);

}
