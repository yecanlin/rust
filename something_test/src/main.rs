use std::collections::HashMap;

fn main() {
    let mut hash_test = HashMap::new();
    let a = 1;
    let c = "fdsfd";
    hash_test.insert(a, a);
    hash_test.insert(c, c);
    println!("{:?}", hash_test);
}
