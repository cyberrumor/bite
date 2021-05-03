use std::env;
use std::process::exit;
use std::collections::HashMap;
mod lib;


fn main() {
    let args: Vec<String> = env::args()
        .collect();

    if args.len() < 1 {
        println!("usage: rust_version filename1.txt filename2.txt");
        exit(1);
    }

    let filenames: &[String] = &args[1..];

    // vectorize corpus
    let all_words: Vec<String> = lib::corpi_to_vec(filenames);


    // vec will hold our maps
    let mut vec: Vec<HashMap<String, usize>> = Vec::new();
    let mut sample_length: usize = 3;
    let mut trace_max: usize = 0;
    // populate frequency_dict, increasing sample_length by one until 
    loop {

        let frequency_dict: HashMap<String, usize> = lib::gen_frequency(&all_words, sample_length);
        let max_value: usize = frequency_dict.values().max().unwrap().clone();
        println!("max value: {:?}", max_value);
        if max_value == trace_max {
            vec.push(frequency_dict);
            break;
        }
        trace_max = max_value;
        vec.push(frequency_dict);
        sample_length += 1;
    }
}
