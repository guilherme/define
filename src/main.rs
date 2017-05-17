extern crate hyper;
extern crate serde_json;
use hyper::Client;
use std::env;
use std::process;
use std::io::Read;

use serde_json::{Value};



struct PearsonDictionary {

}
impl PearsonDictionary {

  fn define(term: &str) -> String {
    let client = Client::new();
    let mut res = client.get(format!("http://api.pearson.com/v2/dictionaries/ldoce5/entries?headword={}", term).as_str()).send().unwrap();
    let mut body = String::new();
    match res.read_to_string(&mut body) {
      Ok(_) => {
        body
      },
      Err(_) => {
        body
      }
    }
  }
}


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

  let body = PearsonDictionary::define(&headword);
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
  println!("{}", body);

}
