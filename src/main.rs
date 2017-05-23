extern crate define;
use std::env;
use std::process;
use define::dictionaries;

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

  let definitions = dictionaries::PearsonDictionary::define(&headword);
  for definition in definitions.iter() {
    println!("word: {}", definition.word);
    println!("meanings(s):");
    for meaning in definition.meanings.iter() {
      println!("{}", meaning.description);
    }
  }
}
