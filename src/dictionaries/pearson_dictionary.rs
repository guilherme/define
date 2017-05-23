extern crate hyper;
extern crate serde_json;
use self::hyper::Client;
use super::Definition;
use super::Meaning;
use std::io::Read;
use self::serde_json::{Value};

pub struct PearsonDictionary {

}
impl PearsonDictionary {

  pub fn define(term: &str) -> Vec<Definition> {
    let client = Client::new();
    let mut res = client.get(format!("http://api.pearson.com/v2/dictionaries/ldoce5/entries?headword={}", term).as_str()).send().unwrap();
    let mut body = String::new();
    match res.read_to_string(&mut body) {
      Ok(_) => {
        // TODO: CONVERT INTO A PARSER
        let v: Value = serde_json::from_str(body.as_str()).unwrap();
        let mut definitions: Vec<Definition> = Vec::new();
        for words in v["results"].as_array().unwrap() {
          let mut meanings: Vec<Meaning> = Vec::new();
          for sense in words["senses"].as_array().unwrap() {
            for definition in sense["definition"].as_array().unwrap() {
              let meaning = Meaning { description: definition.as_str().unwrap().to_string() };
              meanings.push(meaning);
            }
          }
          // TODO: how to define it before the loop above?
          let definition = Definition { word:  words["headword"].as_str().unwrap().to_string(), meanings: meanings };
          definitions.push(definition);
        }
        definitions
      },
      Err(_) => {
        vec![]
      }
    }
  }
}
