use std::collections::HashMap;

fn main() {
    // let mut scores = HashMap::new();
    // let team_bule= "bule".to_string();
    // let team_red= "red".to_string();
    // scores.insert(team_bule, 10);
    // scores.insert(team_red, 50);

    // println!("the valu of hash map id  {:?}", scores);

    //get map value

    // let value = scores.get(&String::from("red")).unwrap_or(&0);
    // let value = scores.get(&String::from("red")).copied().unwrap_or(0);

    // println!("the value is {value}");

    // for (key, val) in &scores{
    //     println!("key= {key} and value= {val}");
    // }


    // Overwriting a Value
    let mut scores = HashMap::new();

    scores.insert("hi".to_string(), 10);
    // scores.insert("hi".to_string(), 5);
    scores.insert("bye".to_string(), 45);

    println!("the value {:?}", scores);

    // Adding a Key and Value Only If a Key Isn’t Present
    scores.entry("red".to_string()).or_insert(10);
    scores.entry("bye".to_string()).or_insert(0);

    println!("the value {:?}", scores);

    // Updating a Value Based on the Old Value

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);

        *count+=1;
    }

    println!("{map:?}");

}
