extern crate hyper;
extern crate serde_json;
use self::hyper::Client;
use super::Definition;
use super::Meaning;
use std::io::Read;
// use self::serde_json::{Value};

pub struct PearsonDictionary {

}
impl PearsonDictionary {

  pub fn define(term: &str) -> Vec<Definition> {
    let client = Client::new();
    let mut res = client.get(format!("http://api.pearson.com/v2/dictionaries/ldoce5/entries?headword={}", term).as_str()).send().unwrap();
    let mut body = String::new();
    match res.read_to_string(&mut body) {
      Ok(_) => {
// let v: Value = serde_json::from_str(body.as_str()).unwrap();
// for words in v["results"].as_array().unwrap() {
//   println!("word: {}", words["headword"]);
//   for sense in words["senses"].as_array().unwrap() {
//     println!("definition(s):");
//     for definition in sense["definition"].as_array().unwrap() {
//       println!("{}", definition);
//     }
//   }
// } */
        vec![Definition { word: "hello".to_string(), meanings: vec![Meaning { description: "hello".to_string() }] }]
      },
      Err(_) => {
        vec![]
      }
    }
  }
}
