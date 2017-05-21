extern crate serde_json;
extern crate define;
use std::env;
use std::process;
use define::dictionaries;

use serde_json::{Value};

fn main() {
  let args: Vec<_> = env::args().collect();
  let headword : String; 
  if args.len() == 2 {
    headword = String::from(args[1].clone());
    println!("Defining: {}", headword);
  } else {
    println!("Usage: define <word>");
    process::exit(1);
  }

  let body = dictionaries::PearsonDictionary::define(&headword);
  let v: Value = serde_json::from_str(body.as_str()).unwrap();
  for words in v["results"].as_array().unwrap() {
    println!("word: {}", words["headword"]);
    for sense in words["senses"].as_array().unwrap() {
      println!("definition(s):");
      for definition in sense["definition"].as_array().unwrap() {
        println!("{}", definition);
      }
    }
  }
}
