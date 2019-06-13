fn main() {
    //let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3, 4, 5];

    println!("vector is {:?}", v);

    let third: &i32 = &v[2];

    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {:?}", v.get(2)),
        None => println!("There is no third element."),
    }

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
}
