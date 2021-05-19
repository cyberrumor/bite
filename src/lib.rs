use std::fs;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Sentence {
    pub original: String,
    pub stripped: String,
    pub tokenized: Vec<String>,
    pub trees: Vec<String>,
    pub score: f64,
}

pub fn tokenize_words(corpus: &str) -> Vec<String> {
    let mut result: Vec::<String> = Vec::new();
    for word in corpus.to_lowercase().split_whitespace() {
        result.push(word.to_string());
    }
    result
}

pub fn tokenize_sent(corpus: &str) -> Vec<String> {
    let mut result: Vec::<String> = Vec::new();
    // collect words until the result is a sentence
    let mut sentence: String = String::new();
    let splits = corpus.split_whitespace();
    let mut words: Vec<String> = Vec::new();
    for i in splits {
        words.push(i.to_string());
    }
    let mut index: usize = 0;
    while index < words.len() {
        // add a space if sentence isn't empty
        if !sentence.is_empty() {
            sentence.push(' ');
        }
        // add current word
        sentence.push_str(&words[index]);
        // prep for the next word
        index += 1;
        // reset our bools
        let mut punctuation: bool = false;

        // identify if the sentence ends with punctuation.
        if sentence.ends_with('.') {
            let second_to_last: usize = &words[index - 1].chars().count() - 2;
            if sentence.ends_with("Mr.")
            | sentence.ends_with("Mrs.")
            | sentence.ends_with("Ms.")
            | sentence.ends_with("Dr.")
            | sentence.ends_with("Mr.")
            // if it ends with a period and isn't a title, the last leter before
            // the period be lowercase. Otherwise, it must be a middle initial or an acronym.
            | &words[index - 1].clone().chars().nth(second_to_last).unwrap().is_uppercase() {
                punctuation = false;
            } else {
                // it's a valid end-of-sentence period.
                punctuation = true;
            }

        } else if sentence.ends_with('?')
        | sentence.ends_with('!')
        | sentence.ends_with(")\"")
        | sentence.ends_with('\"')
        | sentence.ends_with('”')
        | sentence.ends_with(")”")
        // workaround wikipedia references at sentence end
        | sentence.ends_with(']') {
            punctuation = true;
        }

        // a sentence needs balanced parentheses.
        if sentence.matches('(').count() != sentence.matches(')').count() {
            continue;
        }
        if sentence.matches('[').count() != sentence.matches(']').count() {
            continue;
        }

        // if the sentence doesn't end with punctation, keep going.
        if !punctuation {
            continue;
        }

        // a sentence must have a matching number of quotes.
        if sentence.matches('"').count() % 2 != 0 {
            continue;
        }

        // the first letter of the next word should be uppercase.
        if index < words.len() && words[index].chars().next().unwrap().is_lowercase() {
            continue;
        }

        // If we've reached the bottom, we have a full sentence.
        // Push it into the results and clear it.
        result.push(sentence.clone());
        sentence.clear();
    }
    result
}


pub fn strip_nonalpha(corpus: &str) -> String {
    let mut result: String = String::new();
    for character in corpus.chars() {
        if character.is_alphabetic() || character.is_whitespace() {
            result.push(character);
        }
    }
    result
}


pub fn gen_heatmap(db: &Vec<Sentence>) -> HashMap<String, usize> {
    let mut frequency_dict: HashMap<String, usize> = HashMap::new();
    for sent in db {
        for tree in &sent.trees {
            let count = frequency_dict.entry(tree.to_string()).or_insert(0);
            *count += 1;
        }
    }
    frequency_dict
}

pub fn stringify_corpus(file: String) -> String {
    let corpus = fs::read_to_string(file)
        .expect("Something went wrong reading the file");
    let result: String = corpus.replace("\n", " ");
    result
}


pub fn get_trees(sent: Vec<String>) -> Vec<String> {
    let mut trees: Vec<String> = Vec::new();
    //if sent.len() < 2 {
    //    return sent;
    //}

    for x in 0..sent.len() {
        let mut y: usize = x + 2;
        while y <= sent.len() {
            let vec: Vec<String> = sent[x..y].to_vec();
            let mut phrase: String = String::new();
            for word in &vec {
                phrase.push_str(word);
                phrase.push(' ');
            }

            let result = phrase.trim_end().to_string();
            trees.push(result);
            y += 1
        }
    }

    trees
}












































