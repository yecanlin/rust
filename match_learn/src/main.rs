fn main() {
    let a = Coin::A;
    let d = Coin::D;
    println!("a value is: {}", value_in_cents(a));
    println!("b value is: {}", value_in_cents(d));
}

enum Coin {
    A,
    B,
    C,
    D
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::A => 1,
        Coin::B => 2,
        Coin::C => 3,
        Coin::D => 4
    }
}
