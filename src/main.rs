use std::env;
use std::process::exit;
mod lib;


fn main() {
    let args: Vec<String> = env::args()
        .collect();

    if args.is_empty() {
        println!("usage: bite filename1.txt filename2.txt");
        exit(1);
    }

    let filenames: &[String] = &args[1..args.len()];

    let summary: String = lib::run(filenames);

    println!("{:?}", summary);

}
