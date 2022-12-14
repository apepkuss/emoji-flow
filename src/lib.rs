#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

lazy_static! {
    static ref EMOJIS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("lol", "π");
        map.insert("εεε", "π");
        map.insert("sad", "π");
        map.insert("ιΎε", "π");
        map.insert("XD", "π");
        map.insert("xD", "π");
        map
    };
}

#[wasmedge_bindgen]
pub fn run(message: String) -> String {
    let seps = [' ', 'οΌ'];
    let words: Vec<&str> = message.as_str().split(seps).collect();

    let mut s = String::new();
    for word in words {
        if EMOJIS.contains_key(word) {
            s.push_str(EMOJIS[word]);
        } else {
            s.push_str(word);
            s.push_str(" ");
        }
    }

    s.trim_end().to_string()
}

// #[test]
// fn test_flow() {
//     let ret = run(String::from("ηζθΆ£οΌεεε"));
//     assert_eq!(ret, String::from("ηζθΆ£ π"));

//     let ret = run(String::from("I am so sad"));
//     assert_eq!(ret, String::from("I am so π"));
// }
