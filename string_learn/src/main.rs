fn main() {
    let data = "initial contents";

    let s1 = String::from("Hello, ");
    let s2 = String::from("world");
    let s3 = s1 + &s2;
    println!("The value of s3 is: '{}'", s3);
    println!("The value of s2 is: {}", s2);

    let t1 = String::from("tic");
    let t2 = String::from("tac");
    let t3 = String::from("toe");

    let t = format!("{}-{}-{}", t1, t2, t3);

    println!("{}", t);


    let len = String::from("Здравствуйте").len();
    println!("len is: {}", len);
    let hello = "Здравствуйте";
    let answer = &hello[0..4];
    println!("answer is: {}", answer);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
