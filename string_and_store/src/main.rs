fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    //let s1 = String::from("hello");
    //let s2 = s1.clone();
    //println!("{}, world!", s1);

    let s_1 = String::from("s1dsdsds");
    let s_2 = s_1.clone();
    println!("s_1 is {}, s_2 is {}", s_1, s_2);

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}
