use std::fs;
use std::collections::HashMap;

pub fn textract(corpus: String) -> Vec<String> {
    let mut result: Vec::<String> = Vec::new();
    for word in corpus.to_lowercase().split_whitespace() {

        result.push(word.to_string());
    }
    result
}

pub fn gen_frequency(all_words: Vec<String>, sample_length: usize) -> HashMap<String, u64> {
    let mut frequency_dict: HashMap<String, u64> = HashMap::new();

    // prime
    let mut x: usize = 0;
    let mut y: usize = sample_length;
    let mut last_progress: usize = 0;
    // this loop populates frequency_dict
    while y <= all_words.len() {
        let progress: usize = ((y as f64 / all_words.len() as f64) * 100.0) as usize;
        if progress != last_progress {
            eprint!("Progress: {:?}% \r", progress);
            last_progress = progress;
        }
        // get a slice of sample_length words
        let slice = &all_words[x..y];
        let mut phrase = String::new();
        for word in slice {
            phrase.push_str(word);
            phrase.push_str(" ");
        }
        let key = phrase.trim_end().to_string();
        // println!("{:?}", key);
        // insert key only if it doesn't exist yet, if it does, update it
        let count = frequency_dict.entry(key).or_insert(0);
        *count += 1;
        y += 1;
        x += 1;
    }
    println!();
    frequency_dict
}

pub fn corpi_to_vec(filenames: &[String]) -> Vec::<String> {
    let mut all_words: Vec<String> = Vec::new();
    let mut index: usize = 1;
    for file in filenames {
        eprint!("Processing corpus {:?}/{:?}\r", index, filenames.len());
        let corpus = fs::read_to_string(file)
            .expect("Something went wrong reading the file");
        all_words.extend(textract(corpus));
        index += 1;
    }
    println!();
    all_words
}

