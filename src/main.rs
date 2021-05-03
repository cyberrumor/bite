use std::env;
use std::process::exit;
use std::collections::HashMap;
mod lib;


fn main() {
    let args: Vec<String> = env::args()
        .collect();

    if args.len() < 2 {
        println!("usage: rust_version filename1.txt filename2.txt 3");
        exit(1);
    }
    // println!("{:?}", args);
    let filenames: &[String] = &args[1..args.len() -1];
    let arg_length_string: &String = &args
        .last()
        .unwrap();

    let sample_length: usize = arg_length_string
        .parse()
        .unwrap();

    let all_words: Vec<String> = lib::corpi_to_vec(filenames);

    let frequency_dict: HashMap<String, u64> = lib::gen_frequency(all_words, sample_length);
    println!("Unique phrases: {:?}", frequency_dict.len());

}
