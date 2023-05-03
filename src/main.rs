use std::env;
use std::fs;

use email_summariser::explainer;
use llm_chain_llama::ContextParams;

mod email_summariser;

fn read_file(filename: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(filename)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: <email-txt-file> 'your interests, comma, separated'");
        std::process::exit(1);
    }

    let email_file = args
        .get(1)
        .expect("Please provide a file containing the body of the email.");
    let interests = args
        .get(2)
        .expect("Please provide a comma separated string of interests.");
    let contents = read_file(&format!("{}", email_file))?;

    // println!("email file path: {}", email_file);
    // println!("interests: {}", interests);
    // println!("email content: {}", contents);

    explainer(&interests, &contents)?;

    Ok(())
}

// use std::fs::File;
// use std::io::prelude::*;
// use std::{env, io};

// use crate::code_explainer::explainer;
// mod code_explainer;

// fn main() {
//     // Get the filename from the command line arguments
//     let args: Vec<String> = env::args().collect();
//     if args.len() < 2 {
//         eprintln!("Usage: {} <filename>", args[0]);
//         return;
//     }
//     let filename = &args[1];

//     let mut lang = String::new();
//     println!("What language is the code written in?");
//     io::stdin()
//         .read_line(&mut lang)
//         .expect("error: unable to read user input");

//     // Read the file and print its contents
//     // match read_file(filename) {
//     //     Ok(contents) => println!("{}", contents),
//     //     Err(e) => eprintln!("Error reading file {}: {}", filename, e),
//     // }
//     match read_file(filename, |contents| {
//         explainer(&contents);
//     }) {
//         Ok(_) => (),
//         Err(e) => eprintln!("Error reading file {}: {}", filename, e),
//     }

//     // explainer(lang, )
// }

// fn read_file(filename: &str) -> Result<String, std::io::Error> {
//     let mut file = File::open(filename)?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     Ok(contents)
// }
