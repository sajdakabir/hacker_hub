use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("bule".to_string(), 10);
    scores.insert("red".to_string(), 50);

    println!("the valu of hash map id  {:?}", scores);
}
