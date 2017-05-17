extern crate hyper;
use self::hyper::Client;
use std::io::Read;

pub struct PearsonDictionary {

}
impl PearsonDictionary {

  pub fn define(term: &str) -> String {
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
