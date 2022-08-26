#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

lazy_static! {
    static ref EMOJIS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("lol", "😄");
        map.insert("哈哈哈", "😄");
        map.insert("sad", "😔");
        map.insert("难受", "😔");
        map.insert("XD", "😂");
        map.insert("xD", "😂");
        map
    };
}

#[wasmedge_bindgen]
pub fn run(message: String) -> String {
    let seps = [' ', '，'];
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
//     let ret = run(String::from("真有趣，哈哈哈"));
//     assert_eq!(ret, String::from("真有趣 😄"));

//     let ret = run(String::from("I am so sad"));
//     assert_eq!(ret, String::from("I am so 😔"));
// }
