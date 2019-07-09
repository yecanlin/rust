struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let x = None;
    let y = 5;

    match x {
        Some(50) => println!("Got the value : 50!"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    match y {
        1 | 5 => println!("One or Five !"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    match y {
        1 ... 6 => println!("One through six !"),
        _ => println!("Something else !"),
    }

    let z = 'e';

    match z {
        'a' ... 'j' => println!("early ASCII letter"),
        'k' ... 'z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;

    assert_eq!(0, a);
    assert_eq!(7, b);

    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("fount a string");
    }
    println!("{:?}", s);

    let t = Some(3);

    match t {
        Some(x) if x < 5 => println!("Less than 5"),
        Some(x) => println!("{}", x),
        None => (),
    }

    let a = 4;
    let b = true;
    match a {
        4 | 5 | 6 if b => println!("yes"),
        _ => println!("no"),
    }

    let robot_name = &Some(String::from("Tralic"));

    match robot_name {
        Some(name) => println!("Found a name: {}", name),
        None => (),
    }
    println!("robot_name is : {:?}", robot_name);
}
