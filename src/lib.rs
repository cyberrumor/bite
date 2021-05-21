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

#[must_use]
pub fn run(filenames: &[String]) -> String {
    // stringify corpora
    let mut corpora: String = String::new();
    for file in filenames {
        let corpus: String = stringify_corpus(file.clone());
        corpora.push_str(&" ");
        corpora.push_str(&corpus);
    }

    // split into sentences
    let original_sentences: Vec<String> = tokenize_sent(&corpora);

    // create sentence objects
    let mut db: Vec<Sentence> = Vec::new();
    for original in original_sentences {
        let stripped: String = strip_nonalpha(&original.clone().to_lowercase());
        let tokenized: Vec<String> = tokenize_words(&stripped);
        let trees: Vec<String> = get_trees(&tokenized);
        let object: Sentence = Sentence {
            original,
            stripped,
            tokenized,
            trees,
            score: 0.0,
        };
        db.push(object)
    }

    let heatmap: HashMap<String, usize> = gen_heatmap(&db);

    // get average lengths
    let mut lengths: usize = 0;
    for s in &db {
        lengths += s.trees.len();
    }
    let ave_length: f64 = lengths as f64 / db.len() as f64;

    // give each tree a score. If a tree somehow isn't in heatmap,
    // add zero.
    for s in &mut db {
        let mut score: usize = 0;
        for tree in &s.trees {
            score += *heatmap.get(tree).unwrap_or(&0);
        }
        s.score = score as f64 / ave_length as f64;
    }

    // get the average scores and length
    let mut all_scores: f64 = 0.0;
    for s in &db {
        all_scores += s.score;
    }
    let ave_score: f64 = all_scores as f64 / db.len() as f64;

    // produce summary
    let mut summary: String = String::new();
    for s in &db {
        if s.score as f64 > ave_score * 2.5 {
            summary.push_str(&s.original);
            summary.push(' ');

        }

    }
    summary = summary.trim_end().to_string();
    summary
}

#[must_use]
pub fn tokenize_words(corpus: &str) -> Vec<String> {
    let mut result: Vec::<String> = Vec::new();
    for word in corpus.to_lowercase().split_whitespace() {
        result.push(word.to_string());
    }
    result
}

#[must_use]
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
        let second_to_last: usize = &words[index - 1].chars().count() - 2;
        if sentence.ends_with('.') {
            if sentence.ends_with("Mr.")
            | sentence.ends_with("Mrs.")
            | sentence.ends_with("Ms.")
            | sentence.ends_with("Dr.")
            | sentence.ends_with("Mr.")
            // if it ends with a period and isn't a title, the last leter before
            // the period be lowercase. Otherwise, it must be a middle initial 
            // or an acronym. If chars is somehow empty, call it lowercase.
            | words[index - 1]
            .clone()
            .chars()
            .nth(second_to_last)
            // .unwrap_or('a')
            .unwrap()
            .is_uppercase() {
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
        // if there's no next word, call it uppercase and end sentence.
        if index < words.len()
        && words[index]
        .chars()
        .next()
        .unwrap_or('A')
        .is_lowercase() {
            continue;
        }

        // If we've reached the bottom, we have a full sentence.
        // Push it into the results and clear it.
        result.push(sentence.clone());
        sentence.clear();
    }
    result
}

#[must_use]
pub fn strip_nonalpha(corpus: &str) -> String {
    let mut result: String = String::new();
    for character in corpus.chars() {
        if character.is_alphabetic() || character.is_whitespace() {
            result.push(character);
        }
    }
    result
}

#[must_use]
pub fn gen_heatmap(db: &[Sentence]) -> HashMap<String, usize> {
    let mut freq_dict: HashMap<String, usize> = HashMap::new();
    for sent in db {
        for tree in &sent.trees {
            let count = freq_dict.entry(tree.to_string()).or_insert(0);
            *count += 1;
        }
    }
    freq_dict
}

#[must_use]
pub fn stringify_corpus(file: String) -> String {
    let corpus = fs::read_to_string(file)
        .expect("Something went wrong reading the file");
    let result: String = corpus.replace("\n", " ");
    result
}

#[must_use]
pub fn get_trees(sent: &[String]) -> Vec<String> {
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









































