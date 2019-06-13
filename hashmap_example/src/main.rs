use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    let (score) = score;
    println!("Team blue score is: {:?}", score);

    scores.entry(String::from("Red")).or_insert(25);
    scores.entry(String::from("Blue")).or_insert(100);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
        println!("{:?}", count);
    }
    
    println!("{:?}", map);

    println!("{:?}", scores.entry(String::from("Red")).or_insert(30));
    
    println!("{:?}", scores.entry(String::from("Red")));
}
