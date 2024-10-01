use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    let team_bule= "bule".to_string();
    let team_red= "red".to_string();
    scores.insert(team_bule, 10);
    scores.insert(team_red, 50);

    println!("the valu of hash map id  {:?}", scores);

    //get map value

    // let value = scores.get(&String::from("red")).unwrap_or(&0);
    let value = scores.get(&String::from("red")).copied().unwrap_or(0);

    println!("the value is {value}");
}
